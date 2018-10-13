/**
 *  UniProt
 *  -------
 *
 *  Record definitions for the UniProt KB service.
 *
 *  :copyright: (c) 2018 Alex Huszagh.
 *  :license: MIT, see LICENSE.md for more details.
 */

use regex::Regex;
use std::error::Error;
use std::fmt;

use ::csv;
use ::ref_slice::ref_slice;
use ::serde_json;

use complete::Complete;
use fasta::{Fasta, FastaCollection};
use proteins::AverageMass;
use proteins::ProteinMass;
use tbt::{Tbt};       // TbtCollection
//use text::{Text, TextCollection};
use valid::Valid;

// UNIPROT
// -------

//  The following is a mapping of the UniProt form-encoded keys, struct
//  field names, and UniProt displayed column names.
//  Despite the name correspondence, the information may not be a
//  identical in one format or another, for example, `protein_evidence`
//  is an enumeration, while in a displayed column it's a string, and
//  in FASTA it's a numerical identifier. `ProteinEvidence` is the same
//  as `"Evidence at protein level"` which is the same as `1`.
//
//  Entry:
//      Field Name:         "sequence_version"
//      Form-Encoded Key:   "version(sequence)"
//      Displayed Column:   "Sequence version"
//      Notes:
//          Simple integer in all variants.
//
//  Entry:
//      Field Name:         "protein_evidence"
//      Form-Encoded Key:   "existence"
//      Displayed Column:   "Protein existence"
//      Notes:
//          Enumerated value, which appears as a string or integer, with
//          the mapping defined in `ProteinEvidence` and
//          `protein_evidence_verbose`.
//
//  Entry:
//      Field Name:         "mass"
//      Form-Encoded Key:   "mass"
//      Displayed Column:   "Mass"
//      Notes:
//          Simple integer in all variants.
//
//  Entry:
//      Field Name:         "length"
//      Form-Encoded Key:   "length"
//      Displayed Column:   "Length"
//      Notes:
//          Simple integer in all variants.
//
//  Entry:
//      Field Name:         "gene"
//      Form-Encoded Key:   "genes(PREFERRED)"
//      Displayed Column:   "Gene names  (primary )"
//      Notes:
//          TODO(ahuszagh) [I believe this frequently gives more than
//          one gene name, confirm with the unannotated human proteome.
//          If so, designate a regex for filtering from external queries.]
//
//  Entry:
//      Field Name:         "id"
//      Form-Encoded Key:   "id"
//      Displayed Column:   "Entry"
//      Notes:
//          Accession number as a string.
//
//  Entry:
//      Field Name:         "mnemonic"
//      Form-Encoded Key:   "entry name"
//      Displayed Column:   "Entry name"
//      Notes:
//          Mnemonic identifier as a string.
//
//  Entry:
//      Field Name:         "name"
//      Form-Encoded Key:   "protein names"
//      Displayed Column:   "Protein names"
//      Notes:
//          Name for the protein (ex. Glyceraldehyde-3-phosphate
//          dehydrogenase). However, UniProt frequently spits out
//          more than one possible protein name, with each subsequent
//          name enclosed in parentheses (ex. "Glyceraldehyde-3-phosphate
//          dehydrogenase (GAPDH) (EC 1.2.1.12) (Peptidyl-cysteine
//          S-nitrosylase GAPDH) (EC 2.6.99.-)").
//
//  Entry:
//      Field Name:         "organism"
//      Form-Encoded Key:   "organism"
//      Displayed Column:   "Organism"
//      Notes:
//          Species name (with an optional common name in parentheses).
//          BDB considers the common name superfluous, and therefore
//          removes it from all records fetched from external queries.
//          Strain information, which is also enclosed in parentheses,
//          however, should not be removed.
//
//  Entry:
//      Field Name:         "proteome"
//      Form-Encoded Key:   "proteome"
//      Displayed Column:   "Proteomes"
//      Notes:
//          Proteomes include a proteome identifier and an optional
//          proteome location, for example, "UP000001811: Unplaced",
//          "UP000001114: Chromosome", and "UP000001811" are all valid
//          values. We discard the location, and solely store the proteome
//          identifier.
//
//
//  Entry:
//      Field Name:         "sequence"
//      Form-Encoded Key:   "sequence"
//      Displayed Column:   "Sequence"
//      Notes:
//          Aminoacid sequence of the protein, as a string.
//
//  Entry:
//      Field Name:         "taxonomy"
//      Form-Encoded Key:   "organism-id"
//      Displayed Column:   "Organism ID"
//      Notes:
//          Numerical identifier for the species, described by "name".


/**
 *  \brief Identifier for the evidence type for protein existence.
 *
 *  An identifier used by biological databases for the level of evidence
 *  that supports a protein's existence. Strong evidence includes
 *  evidence at the protein level, while weaker evidence is evidence
 *  at the transcript (or mRNA) level. Weak evidence is inferred from
 *  homology from similar species. Curated protein databases frequently
 *  only include proteins identified at the protein level.
 *
 *  `Unknown` is a custom value for invalid entries, or those with yet-
 *  to-be annotated protein evidence scores.
 *
 *  More documentation can be found at:
 *      https://www.uniprot.org/help/protein_existence
 */
enum_number!(ProteinEvidence {
    ProteinLevel = 1,
    TranscriptLevel = 2,
    Inferred = 3,
    Predicted = 4,
    Unknown = 5,
});

/**
 *  \brief Convert enumerated value for ProteinEvidence to verbose text.
 */
pub fn protein_evidence_verbose(evidence: ProteinEvidence) -> &'static str {
    match evidence {
        ProteinEvidence::ProteinLevel       => "Evidence at protein level",
        ProteinEvidence::TranscriptLevel    => "Evidence at transcript level",
        ProteinEvidence::Inferred           => "Inferred from homology",
        ProteinEvidence::Predicted          => "Predicted",
        ProteinEvidence::Unknown            => "Unknown evidence (BDB-only designation)",
    }
}

/**
 *  \brief Model for a single record from a UniProt KB query.
 *
 *  Record including core query fields for a given UniProt identifier.
 *  The query fields are defined [here](http://www.uniprot.org/help/query-fields).
 *
 *  \param sequence_version Numerical identifier for protein version.
 *  \param protein_evidence Numerical identifier for protein evidence.
 *  \param mass             Mass of the protein.
 *  \param length           Protein sequence length.
 *  \param gene             HGNC Gene name.
 *  \param id               Accession number (randomly assigned identifier).
 *  \param mnemonic         Entry name (readable identifier).
 *  \param name             Protein name.
 *  \param organism         Readable organism name.
 *  \param proteome         UniProt proteome identifier.
 *  \param sequence         Protein aminoacid sequence.
 *  \param taxonomy         Taxonomic identifier.
 *
 *  The sequence version is a numerical identifier starting from 1 for
 *  the revision of the protein ID.
 *
 *  The protein evidence is a numerical identifier, with 1 meaning
 *  evidence at the protein level, 2 meaning evidence at the
 *  transcript level, and 3 meaning the protein was inferred via
 *  homology.
 */
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Record {
    sequence_version: u8,
    protein_evidence: ProteinEvidence,
    mass: u64,
    length: u32,
    gene: String,
    id: String,
    mnemonic: String,
    name: String,
    organism: String,
    proteome: String,
    sequence: String,
    taxonomy: String,
}

/**
 *  \brief Process organism name from UniProt KB databases.
 *
 *  Remove the enclosed common names or synonym, when applicable.
 *
 *  \example
 *      // process_organism("Oryctolagus cuniculus (Rabbit)")
 *      //     -> "Oryctolagus cuniculus"
 *
 *      // process_organism("Homo sapiens (Human)")
 *      //     -> "Homo sapiens"
 *
 *      // process_organism("Histophilus somni (strain 2336) (Haemophilus somnus)")
 *      //     -> "Histophilus somni (strain 2336)"
 *
 *      // process_organism("Actinobacillus succinogenes (strain ATCC 55618 / 130Z)")
 *      //     -> "Actinobacillus succinogenes (strain ATCC 55618 / 130Z)"
 */
// TODO(ahuszagh)
//      Write unittests
//      Need to ensure that the common name or synonym is removed,
//      but any strain information is not.
pub fn process_organism(organism: &str) -> &str {
    // TODO: Implement...
    let _organism = organism;
    ""
}

/**
 *  \brief Remove subsequent names from the protein name.
 *
 *  Remove the parentheses-enclosed names from the protein name.
 *
 *  \example
 *      // process_protein_name("Glyceraldehyde-3-phosphate
//          dehydrogenase (GAPDH) (EC 1.2.1.12) (Peptidyl-cysteine
//          S-nitrosylase GAPDH) (EC 2.6.99.-)")
 *      //    -> "Glyceraldehyde-3-phosphate
//          dehydrogenase"
 */
// TODO(ahuszagh)
//      Write unittests
//      Need to ensure that there are no parentheses in any valid names.
//          Use the entire human proteome to determine that.
pub fn process_protein_name(name: &str) -> &str {
    // TODO: Implement...
    let _name = name;
    ""
}

/**
 *  \brief Remove the proteome location from the identifier.
 *
 *  Remove the colon-delineated location from the proteome identifier,
 *  when applicable.
 *
 *  \example
 *      // process_proteome("UP000001811: Unplaced")
 *      //    -> "UP000001811"
 *
 *      // process_proteome("UP000001114: Chromosome")
 *      //    -> "UP000001114"
 */
// TODO(ahuszagh)
//      Write unittests
//      Need to ensure that the code works when the location is not present.
pub fn process_proteome(proteome: &str) -> &str {
    // TODO: Implement...
    let _proteome = proteome;
    ""
}

// TODO: need more processing functions


impl Record {
    /**
     *  \brief Create a blank UniProt record.
     */
    pub fn new() -> Record {
        Record {
            sequence_version: 0,
            protein_evidence: ProteinEvidence::Unknown,
            mass: 0,
            length: 0,
            gene: String::new(),
            id: String::new(),
            mnemonic: String::new(),
            name: String::new(),
            organism: String::new(),
            proteome: String::new(),
            sequence: String::new(),
            taxonomy: String::new(),
        }
    }
}


impl Valid for Record {
    /**
     *  \brief Check if the UniProt record contains valid information.
     */
    fn is_valid(&self) -> bool {
        // regular expression for the UniProt accession number provided by:
        //  https://www.uniprot.org/help/accession_numbers
        lazy_static! {
            static ref ACCESSION_REGEX: Regex = Regex::new(r"(?x)
                \A
                (?:
                    [OPQ][0-9][A-Z0-9]{3}[0-9]|
                    [A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2}
                )
                \z
            ").unwrap();
        }

        lazy_static! {
            static ref MNEMONIC_REGEX: Regex = Regex::new(r"(?x)
                \A
                (?:
                    [a-zA-Z0-9]{1,5}
                    _
                    [a-zA-Z0-9]{1,5}
                )
                \z
            ").unwrap();
        }

        lazy_static! {
            static ref ORGANISM_REGEX: Regex = Regex::new(r"(?x)
                \A
                [[:alpha:]]+\x20[[:alpha:]]+
                \z
            ").unwrap();
        }

        lazy_static! {
            static ref AMINOACID_REGEX: Regex = Regex::new(r"(?x)
                \A
                [ABCDEFGHIJKLMNPQRSTVWXYZabcdefghijklmnpqrstvwxyz]*
                \z
            ").unwrap();
        }

        {
            self.sequence_version >= 1 &&
            self.protein_evidence < ProteinEvidence::Unknown &&
            self.mass > 0 &&
            self.length as usize == self.sequence.len() &&
            self.sequence.len() > 0 &&
            self.gene.len() > 0 &&
            self.name.len() > 0 &&
            ACCESSION_REGEX.is_match(&self.id) &&
            MNEMONIC_REGEX.is_match(&self.mnemonic) &&
            ORGANISM_REGEX.is_match(&self.organism) &&
            AMINOACID_REGEX.is_match(&self.sequence)
        }
    }
}


impl Complete for Record {
    /**
     *  \brief Check if the UniProt record contains all identifiers.
     */
    fn is_complete(&self) -> bool {
        lazy_static! {
            static ref PROTEOME_REGEX: Regex = Regex::new(r"(?x)
                \A
                UP[0-9]{9}
                \z
            ").unwrap();
        }

        lazy_static! {
            static ref TAXONOMY_REGEX: Regex = Regex::new(r"(?x)
                \A
                \d+
                \z
            ").unwrap();
        }

        {
            self.is_valid() &&
            TAXONOMY_REGEX.is_match(&self.taxonomy) &&
            PROTEOME_REGEX.is_match(&self.proteome)
        }
    }
}


impl Fasta for Record {
    /**
     *  \brief Export UniProt record to SwissProt FASTA record.
     */
    fn to_fasta(&self) -> Result<String, &str> {
        if !self.is_valid() {
            return Err("Invalid UniProt record, cannot serialize to FASTA.");
        }

        const SEQUENCE_LINE_LENGTH: usize = 60;

        // initialize string and avoid reallocations.
        let mut fasta = String::new();
        let size = 20 +
                   self.gene.len() +
                   self.id.len() +
                   self.name.len() +
                   self.organism.len() +
                   self.sequence.len();
        fasta.reserve(size);

        // write SwissProt header
        fasta.push_str(">sp|");
        fasta.push_str(&self.id);

        fasta.push('|');
        fasta.push_str(&self.mnemonic);

        fasta.push(' ');
        fasta.push_str(&self.name);

        fasta.push_str(" OS=");
        fasta.push_str(&self.organism);

        fasta.push_str(" GN=");
        fasta.push_str(&self.gene);

        fasta.push_str(" PE=");
        let protein_evidence = self.protein_evidence as u32;
        fasta.push_str(&protein_evidence.to_string());

        fasta.push_str(" SV=");
        fasta.push_str(&self.sequence_version.to_string());

        // write SwissProt sequence, formatted at 60 characters
        // Start from 1, so we go 1..60, rather than 0..59
        let mut i: usize = 1;
        for c in self.sequence.chars() {
            match i {
                1                    => { fasta.push('\n'); i += 1; },
                SEQUENCE_LINE_LENGTH => { i = 1; },
                _                    => { i += 1; },
            }
            fasta.push(c);
        }

        Ok(fasta)
    }

    /**
     *  \brief Import UniProt record from a SwissProt FASTA record.
     */
    fn from_fasta<'a>(fasta: &str) -> Result<Record, &'a str> {
        // split along lines
        // first line is the header, rest are the sequences
        // short-circuit if the header is None.
        let mut lines = fasta.lines();
        let header_option = lines.next();
        if header_option.is_none() {
            return Err("No input data provided to FASTA deserializer.");
        }
        let header = header_option.unwrap();

        // regular expression to extract data from the header
        // line of a SwissProt FASTA record.
        lazy_static! {
            static ref SP_HEADER_REGEX: Regex = Regex::new(r"(?x)
                \A
                (?P<SwissProt>>sp\|)
                (?P<id>[OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2})
                \|
                (?P<mnemonic>[a-zA-Z0-9]{1,5}_[a-zA-Z0-9]{1,5})
                \x20
                (?P<name>[-()_./\x200-9A-Za-z]+)
                \x20OS=
                (?P<organism>[[:alpha:]]+\x20[[:alpha:]]+)
                \x20GN=
                (?P<gene>[[:alnum:]]+)
                \x20PE=
                (?P<pe>\d+)
                \x20SV=
                (?P<sv>\d+)
                \z
            ").unwrap();
        }

        // process the header and match it to the FASTA record
        let cap_option = SP_HEADER_REGEX.captures(&header);
        if header_option.is_none() {
            return Err("Unable to match data to SwissProt header format.");
        }
        let cap = cap_option.unwrap();

        let match_as_str    = | i | cap.name(i).unwrap().as_str();
        let match_as_string = | i | String::from(match_as_str(i));

        // initialize the record with header data
        let mut result = Record {
            sequence_version: match_as_str("sv").parse().unwrap(),
            protein_evidence: serde_json::from_str(match_as_str("pe")).unwrap(),
            mass: 0,
            length: 0,
            gene: match_as_string("gene"),
            id: match_as_string("id"),
            mnemonic: match_as_string("mnemonic"),
            name: match_as_string("name"),
            organism: match_as_string("organism"),

            // unused fields in header
            proteome: String::new(),
            sequence: String::new(),
            taxonomy: String::new(),
        };

        // add sequence data to the FASTA sequence
        for line in lines {
            result.sequence.push_str(&line);
        }

        // calculate the protein length and mass
        result.length = result.sequence.len() as u32;
        let mass = AverageMass::protein_sequence_mass(result.sequence.as_bytes());
        result.mass = mass.round() as u64;

        Ok(result)
    }
}

impl Tbt for Record {
    /**
     *  \brief Export UniProt record to TBT.
     */
    fn to_tbt(&self) -> Result<String, &str> {
        _slice_to_tbt(ref_slice(&self))
    }

    /**
     *  \brief Import UniProt record from a TBT row.
     */
    fn from_tbt<'a>(text: &str) -> Result<Record, &'a str> {
        // TODO(ahuszagh) Implement...
        // 1. Need to find only the first 2 lines.
        // 2. Need to call the deserializer.
        // 3. Need to yank just the first item.

        //_text_to_list(text)[0];
        let _text = text;
        Err("")
    }
}

/**
 *  \brief Collection of UniProt records.
 */
pub type RecordList = Vec<Record>;

impl Valid for RecordList {
    /**
     *  \brief Check if all records are valid.
     */
    fn is_valid(&self) -> bool {
        self.iter().all(|ref x| x.is_valid())
    }
}

impl Complete for RecordList {
    /**
     *  \brief Check if all records are complete.
     */
    fn is_complete(&self) -> bool {
        self.iter().all(|ref x| x.is_complete())
    }
}

impl FastaCollection for RecordList {
    /**
     *  \brief Strict exporter of UniProt record list to SwissProt FASTA.
     */
    fn to_fasta_strict(&self) -> Result<String, &str> {
        // exit early if empty list
        if self.is_empty() {
            return Ok(String::new());
        }

        // construct FASTA records from elements
        let mut v: Vec<String> = vec![];
        for record in self {
            v.push(record.to_fasta()?);
        }

        Ok(v.join("\n\n"))
    }

    /**
     *  \brief Lenient exporter of UniProt record list to SwissProt FASTA.
     */
    fn to_fasta_lenient(&self) -> Result<String, &str> {
        // exit early if empty list
        if self.is_empty() {
            return Ok(String::new());
        }

        // construct FASTA records from elements
        let mut v: Vec<String> = vec![];
        let mut e: &str = "";
        for record in self {
            match record.to_fasta() {
                Err(_e)     => e = _e,
                Ok(_v)      => v.push(_v),
            }
        }

        match v.is_empty() {
            true  => Err(e),
            false => Ok(v.join("\n\n"))
        }
    }

    /**
     *  \brief Strict importer UniProt of record list from SwissProt FASTA.
     */
    fn from_fasta_strict<'a>(fasta: &str) -> Result<RecordList, &'a str> {
        // exit early if empty input data
        if fasta.is_empty() {
            return Ok(RecordList::new());
        }

        // import records from FASTA
        let mut v: RecordList = vec![];
        let records = fasta.split("\n\n");
        for record in records {
            v.push(Record::from_fasta(record)?);
        }

        Ok(v)
    }

    /**
     *  \brief Lenient importer UniProt of record list from SwissProt FASTA.
     */
    fn from_fasta_lenient<'a>(fasta: &str) -> Result<RecordList, &'a str> {
        // exit early if empty input data
        if fasta.is_empty() {
            return Ok(RecordList::new());
        }

        // import records from FASTA
        let mut v: RecordList = vec![];
        let mut e: &str= "";
        let records = fasta.split("\n\n");
        for record in records {
            match Record::from_fasta(record) {
                Err(_e)     => e = _e,
                Ok(_v)      => v.push(_v),
            }
        }

        match v.is_empty() {
            true  => Err(e),
            false => Ok(v)
        }
    }
}

impl Fasta for RecordList {
    /**
     *  \brief Export UniProt record list to SwissProt FASTA records.
     */
    fn to_fasta(&self) -> Result<String, &str> {
        self.to_fasta_lenient()
    }

    /**
     *  \brief Import UniProt record list from SwissProt FASTA records.
     */
    fn from_fasta<'a>(fasta: &str) -> Result<RecordList, &'a str> {
        RecordList::from_fasta_lenient(fasta)
    }
}

// TODO(ahuszagh)
//      Change to Tbt trait
impl Tbt for RecordList {
    /**
     *  \brief Export UniProt records to TBT.
     */
    fn to_tbt(&self) -> Result<String, &str> {
        _slice_to_tbt(&self[..])
    }

    /**
     *  \brief Import UniProt records from TBT.
     */
    fn from_tbt<'a>(text: &str) -> Result<RecordList, &'a str> {
        // TODO(ahuszagh) Implement...
        // 1. Need to call the deserializer.
        // 2. Return values.

        //_text_to_list(text)[0];
        let _text = text;
        Err("")
    }
}

// PRIVATE
// -------

/**
 *  \brief Convert a record to vector of strings to serialize into TBT.
 */
fn _record_to_row(record: &Record) -> Vec<String> {
    vec![
        nonzero_to_string!(record.sequence_version),
        String::from(match record.protein_evidence {
            ProteinEvidence::Unknown    => "",
            _                           => protein_evidence_verbose(record.protein_evidence),
        }),
        nonzero_to_string!(record.mass),
        nonzero_to_string!(record.length),
        record.gene.clone(),
        record.id.clone(),
        record.mnemonic.clone(),
        record.name.clone(),
        record.organism.clone(),
        record.proteome.clone(),
        record.sequence.clone(),
        record.taxonomy.clone(),
    ]
}


/**
 *  \brief Convert a slice of records into TBT.
 */
#[allow(unused_variables)]
#[allow(unused_mut)]
fn _slice_to_tbt_impl(records: &[Record]) -> Result<String, Box<Error>> {
    // Create our custom writer.
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::Necessary)
        .flexible(false)
        .from_writer(vec![]);

    // Serialize the header to TBT.
    writer.write_record(&[
        "Sequence version",         // sequence_version
        "Protein existence",        // protein_evidence
        "Mass",                     // mass
        "Length",                   // length
        "Gene names  (primary )",   // gene
        "Entry",                    // id
        "Entry name",               // mnemonic
        "Protein names",            // name
        "Organism",                 // organism
        "Proteomes",                // proteome
        "Sequence",                 // sequence
        "Organism ID",              // taxonomy
    ])?;

    // Serialize each row to TBT.
    for record in records {
        writer.write_record(&_record_to_row(&record)[..])?;
    }

    // Return a string from the writer bytes.
    Ok(String::from_utf8(writer.into_inner()?)?)
}


/**
 *  \brief Wrap `_slice_to_tbt_impl` with a text-based error.
 */
fn _slice_to_tbt(records: &[Record]) -> Result<String, &str> {
    // TODO: properly implement...
    _slice_to_tbt_impl(records).map_err(|_e| {
        ""
    })
}


// TODO(ahuszagh)
//      Likely remove
///**
// *  \brief Convert tab-delimited text records to a UniProt record list.
// */
//#[allow(unused_variables)]
//fn _text_to_list<'a>(text: &str) -> Result<RecordList, &'a str> {
//    // TODO(ahuszagh)
//    //  Implement the slice to text code.
//
//    Err("Not yet implemented...")
//}

// CONNECTION
// ----------

/**
 *  \brief Module to fetch records using the Uniprot KB service.
 */
pub mod fetch {

    // CONSTANTS
    // ---------

    const HOST: &str = "https://www.uniprot.org:443/uniprot/";

    // ALIAS
    // -----

    use reqwest;
    use url::form_urlencoded;

    use tbt::{Tbt};

    use super::RecordList;

    // API
    // ---

    /**
     *  \brief Request UniProt records by accession number.
     *
     *  \param ids      Single accession number (eg. P46406)
     */
    pub fn by_id<'a>(id: &str) -> Result<RecordList, &'a str> {
        _by_id(id)
    }

    /**
     *  \brief Request UniProt records by accession numbers.
     *
     *  \param ids      Slice of accession numbers (eg. [P46406])
     */
    pub fn by_id_list<'a>(ids: &[&str]) -> Result<RecordList, &'a str> {
        _by_id(&ids.join(" OR "))
    }

    /**
     *  \brief Request UniProt records by mnemonic.
     *
     *  \param ids      Single mnemonic (eg. G3P_RABBIT)
     */
    pub fn by_mnemonic<'a>(mnemonic: &str) -> Result<RecordList, &'a str> {
        _by_mnemonic(mnemonic)
    }

    /**
     *  \brief Request UniProt records by mnemonics.
     *
     *  \param ids      Slice of mnemonics (eg. [G3P_RABBIT])
     */
    pub fn by_mnemonic_list<'a>(ids: &[&str]) -> Result<RecordList, &'a str> {
        _by_mnemonic(&ids.join(" OR "))
    }

    // PRIVATE
    // -------

    // Helper function for calling the UniProt KB service.
    #[allow(unused_variables)]
    fn _call<'a>(query: &str) -> Result<RecordList, &'a str> {
        // create our url with form-encoded parameters
        let params = form_urlencoded::Serializer::new(String::new())
            .append_pair("sort", "score")
            .append_pair("desc", "")
            .append_pair("fil", "")
            .append_pair("force", "no")
            .append_pair("format", "tab")
            .append_pair("query", query)
            .append_pair("columns", "version(sequence),existence,mass,length,genes(PREFERRED),id,entry name,protein names,organism,proteome,sequence,organism-id")
            .finish();
        let url = format!("{}?{}", HOST, params);
        let body = _url_to_body(&url)?;
        // TODO(ahuszagh)   Remove the following debug statements.
        println!("url = {:?}", url);
        println!("body = {:?}", body);

        RecordList::from_tbt(&body)
    }

    // Helper functions to convert URL to UniProt body.
    fn _url_to_body_impl(url: &str) -> Result<String, reqwest::Error> {
        reqwest::get(url)?.text()
    }

    fn _url_to_body<'a>(url: &str) -> Result<String, &'a str> {
        _url_to_body_impl(url).map_err(|e| {
            match e.status() {
                None    => "Internal error, unable to get response.",
                Some(v) => match v.canonical_reason() {
                    None    => "Unknown response code for error code.",
                    Some(r) => r,
                }
            }
        })
    }

    // Helper function for requesting by accession number.
    fn _by_id<'a>(id: &str) -> Result<RecordList, &'a str> {
        _call(&format!("id:{}", id))
    }

    // Helper function for requesting by mnemonic.
    fn _by_mnemonic<'a>(mnemonic: &str)-> Result<RecordList, &'a str> {
        _call(&format!("mnemonic:{}", mnemonic))
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use ::serde_json;
    use super::Record;
    use super::RecordList;
    use super::ProteinEvidence;
    use super::protein_evidence_verbose;
    use complete::Complete;
    use fasta::{Fasta, FastaCollection};
    use tbt::{Tbt};     // TbtCollection
    use valid::Valid;

    fn gapdh() -> Record {
        Record {
            sequence_version: 3,
            protein_evidence: ProteinEvidence::ProteinLevel,
            mass: 35780,
            length: 333,
            gene: String::from("GAPDH"),
            id: String::from("P46406"),
            mnemonic: String::from("G3P_RABIT"),
            name: String::from("Glyceraldehyde-3-phosphate dehydrogenase"),
            organism: String::from("Oryctolagus cuniculus"),
            proteome: String::from("UP000001811"),
            sequence: String::from("MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE"),
            taxonomy: String::from("9986")
        }
    }

    fn bsa() -> Record {
        Record {
            sequence_version: 4,
            protein_evidence: ProteinEvidence::ProteinLevel,
            mass: 69293,
            length: 607,
            gene: String::from("ALB"),
            id: String::from("P02769"),
            mnemonic: String::from("ALBU_BOVIN"),
            name: String::from("Serum albumin"),
            organism: String::from("Bos taurus"),
            proteome: String::from("UP000009136"),
            sequence: String::from("MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA"),
            taxonomy: String::from("9913")
        }
    }

    // PROTEIN EVIDENCE
    // Note: Do not test Unknown, it is an implementation detail.

    #[test]
    fn debug_protein_evidence() {
        let formatted_protein_level = format!("{:?}", ProteinEvidence::ProteinLevel);
        let formatted_transcript_level = format!("{:?}", ProteinEvidence::TranscriptLevel);
        let formatted_inferred = format!("{:?}", ProteinEvidence::Inferred);
        let formatted_predicted = format!("{:?}", ProteinEvidence::Predicted);
        assert_eq!(formatted_protein_level, "ProteinLevel");
        assert_eq!(formatted_transcript_level, "TranscriptLevel");
        assert_eq!(formatted_inferred, "Inferred");
        assert_eq!(formatted_predicted, "Predicted");
    }

    #[test]
    fn serialize_protein_evidence() {
        // to_string
        let w = serde_json::to_string(&ProteinEvidence::ProteinLevel).unwrap();
        assert_eq!(w, "1");

        let x = serde_json::to_string(&ProteinEvidence::TranscriptLevel).unwrap();
        assert_eq!(x, "2");

        let y = serde_json::to_string(&ProteinEvidence::Inferred).unwrap();
        assert_eq!(y, "3");

        let z = serde_json::to_string(&ProteinEvidence::Predicted).unwrap();
        assert_eq!(z, "4");

        // from_str
        let a: ProteinEvidence = serde_json::from_str(&w).unwrap();
        assert_eq!(a, ProteinEvidence::ProteinLevel);

        let b: ProteinEvidence = serde_json::from_str(&x).unwrap();
        assert_eq!(b, ProteinEvidence::TranscriptLevel);

        let c: ProteinEvidence = serde_json::from_str(&y).unwrap();
        assert_eq!(c, ProteinEvidence::Inferred);

        let d: ProteinEvidence = serde_json::from_str(&z).unwrap();
        assert_eq!(d, ProteinEvidence::Predicted);
    }

    #[test]
    fn protein_evidence_verbose_test() {
        assert_eq!(protein_evidence_verbose(ProteinEvidence::ProteinLevel), "Evidence at protein level");
        assert_eq!(protein_evidence_verbose(ProteinEvidence::TranscriptLevel), "Evidence at transcript level");
        assert_eq!(protein_evidence_verbose(ProteinEvidence::Inferred), "Inferred from homology");
        assert_eq!(protein_evidence_verbose(ProteinEvidence::Predicted), "Predicted");
        assert_eq!(protein_evidence_verbose(ProteinEvidence::Unknown), "Unknown evidence (BDB-only designation)");
    }

    // RECORD

    #[test]
    fn debug_record() {
        let formatted_gapdh = format!("{:?}", gapdh());
        let formatted_bsa = format!("{:?}", bsa());
        assert_eq!(formatted_gapdh, "Record { sequence_version: 3, protein_evidence: ProteinLevel, mass: 35780, length: 333, gene: \"GAPDH\", id: \"P46406\", mnemonic: \"G3P_RABIT\", name: \"Glyceraldehyde-3-phosphate dehydrogenase\", organism: \"Oryctolagus cuniculus\", proteome: \"UP000001811\", sequence: \"MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\", taxonomy: \"9986\" }");
        assert_eq!(formatted_bsa, "Record { sequence_version: 4, protein_evidence: ProteinLevel, mass: 69293, length: 607, gene: \"ALB\", id: \"P02769\", mnemonic: \"ALBU_BOVIN\", name: \"Serum albumin\", organism: \"Bos taurus\", proteome: \"UP000009136\", sequence: \"MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\", taxonomy: \"9913\" }");
    }

    #[test]
    fn equality_record() {
        let gapdh_ = gapdh();
        let gapdh_2 = gapdh();
        let bsa_ = bsa();
        assert_eq!(gapdh_, gapdh_2);
        assert_ne!(gapdh_, bsa_);
        assert_ne!(gapdh_2, bsa_);
    }

    #[test]
    fn serialize_record() {
        let gapdh_ = gapdh();
        let x = serde_json::to_string(&gapdh_).unwrap();
        assert_eq!(x, "{\"sequence_version\":3,\"protein_evidence\":1,\"mass\":35780,\"length\":333,\"gene\":\"GAPDH\",\"id\":\"P46406\",\"mnemonic\":\"G3P_RABIT\",\"name\":\"Glyceraldehyde-3-phosphate dehydrogenase\",\"organism\":\"Oryctolagus cuniculus\",\"proteome\":\"UP000001811\",\"sequence\":\"MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\",\"taxonomy\":\"9986\"}");

        let bsa_ = bsa();
        let x = serde_json::to_string(&bsa_).unwrap();
        assert_eq!(x, "{\"sequence_version\":4,\"protein_evidence\":1,\"mass\":69293,\"length\":607,\"gene\":\"ALB\",\"id\":\"P02769\",\"mnemonic\":\"ALBU_BOVIN\",\"name\":\"Serum albumin\",\"organism\":\"Bos taurus\",\"proteome\":\"UP000009136\",\"sequence\":\"MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\",\"taxonomy\":\"9913\"}");
    }

    // Check a record `from_fasta` with incomplete data is equal to the original.
    fn _incomplete_eq(x: &Record, y: &Record) {
        assert_eq!(y.sequence_version, x.sequence_version);
        assert_eq!(y.protein_evidence, x.protein_evidence);
        assert_eq!(y.mass, x.mass);
        assert_eq!(y.length, x.length);
        assert_eq!(y.gene, x.gene);
        assert_eq!(y.id, x.id);
        assert_eq!(y.mnemonic, x.mnemonic);
        assert_eq!(y.name, x.name);
        assert_eq!(y.organism, x.organism);
        assert_eq!(y.proteome, "");
        assert_eq!(y.sequence, x.sequence);
        assert_eq!(y.taxonomy, "");

        assert!(x.is_valid());
        assert!(x.is_complete());

        assert!(y.is_valid());
        assert!(!y.is_complete());
    }

    #[test]
    fn fasta_gapdh() {
        let g = gapdh();

        // to_fasta
        let x = g.to_fasta().unwrap();
        assert_eq!(x, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE");

        // from_fasta
        let y = Record::from_fasta(&x).unwrap();
        _incomplete_eq(&g, &y);
    }

    #[test]
    fn fasta_bsa() {
        let b = bsa();

        // to_fasta
        let x = b.to_fasta().unwrap();
        assert_eq!(x, ">sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4\nMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPF\nDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEP\nERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYY\nANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVA\nRLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKE\nCCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRR\nHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEK\nLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLIL\nNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLP\nDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVV\nSTQTALA");

        // from_fasta
        let y = Record::from_fasta(&x).unwrap();
        _incomplete_eq(&b, &y);
    }

    #[test]
    fn tbt_gapdh() {
        let g = gapdh();

        // to_tbt
        let x = g.to_tbt().unwrap();
        assert_eq!(x, "Sequence version\tProtein existence\tMass\tLength\tGene names  (primary )\tEntry\tEntry name\tProtein names\tOrganism\tProteomes\tSequence\tOrganism ID\n3\tEvidence at protein level\t35780\t333\tGAPDH\tP46406\tG3P_RABIT\tGlyceraldehyde-3-phosphate dehydrogenase\tOryctolagus cuniculus\tUP000001811\tMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\t9986\n");

        // TODO(ahuszagh) Implement deserializer
    }

    #[test]
    fn tbt_bsa() {
        let b = bsa();

        // to_tbt
        let x = b.to_tbt().unwrap();
        assert_eq!(x, "Sequence version\tProtein existence\tMass\tLength\tGene names  (primary )\tEntry\tEntry name\tProtein names\tOrganism\tProteomes\tSequence\tOrganism ID\n4\tEvidence at protein level\t69293\t607\tALB\tP02769\tALBU_BOVIN\tSerum albumin\tBos taurus\tUP000009136\tMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\t9913\n");

        // TODO(ahuszagh) Implement deserializer
    }

    #[test]
    fn is_record() {
        // test various permutations that can lead to
        // invalid or incomplete identifications
        let g1 = gapdh();
        let mut g2 = g1.clone();
        assert!(g2.is_valid());
        assert!(g2.is_complete());

        // check keeping the protein valid but make it incomplete
        g2.proteome = String::new();
        assert!(g2.is_valid());
        assert!(!g2.is_complete());
        g2.proteome = g1.proteome.clone();

        g2.taxonomy = String::new();
        assert!(g2.is_valid());
        assert!(!g2.is_complete());
        g2.taxonomy = g1.taxonomy.clone();

        // check replacing items with valid, but different data
        g2.sequence_version = 1;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.sequence_version = g1.sequence_version;

        g2.protein_evidence = ProteinEvidence::Inferred;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.protein_evidence = g1.protein_evidence;

        g2.mass = 64234;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.mass = g1.mass;

        g2.sequence = String::from(&g2.sequence[0..200]);
        g2.length = 200;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.sequence = g1.sequence.clone();
        g2.length = g1.length;

        g2.gene = String::from("HIST1H1A");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.gene = g1.gene.clone();

        g2.id = String::from("A0A022YWF9");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.id = g1.id.clone();

        g2.id = String::from("A2BC19");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.id = g1.id.clone();

        g2.mnemonic = String::from("H11_HUMAN");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.mnemonic = g1.mnemonic.clone();

        g2.name = String::from("Histone H1.1");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.name = g1.name.clone();

        g2.organism = String::from("Homo sapiens");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.organism = g1.organism.clone();

        g2.proteome = String::from("UP000005640");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.proteome = g1.proteome.clone();

        g2.taxonomy = String::from("9606");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        g2.taxonomy = g1.taxonomy.clone();

        // check replacing items with invalid data
        g2.sequence_version = 0;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.sequence_version = g1.sequence_version;

        g2.protein_evidence = ProteinEvidence::Unknown;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.protein_evidence = g1.protein_evidence;

        g2.mass = 0;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.mass = g1.mass;

        g2.length = 334;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.length = g1.length;

        g2.gene = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.gene = g1.gene.clone();

        g2.id = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.id = g1.id.clone();

        g2.mnemonic = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.mnemonic = g1.mnemonic.clone();

        g2.name = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.name = g1.name.clone();

        g2.organism = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.organism = g1.organism.clone();

        g2.sequence = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        g2.sequence = g1.sequence.clone();
    }

    // LIST

    #[test]
    fn debug_list() {
        let l = format!("{:?}", vec![gapdh(), bsa()]);
        assert_eq!(l, "[Record { sequence_version: 3, protein_evidence: ProteinLevel, mass: 35780, length: 333, gene: \"GAPDH\", id: \"P46406\", mnemonic: \"G3P_RABIT\", name: \"Glyceraldehyde-3-phosphate dehydrogenase\", organism: \"Oryctolagus cuniculus\", proteome: \"UP000001811\", sequence: \"MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\", taxonomy: \"9986\" }, Record { sequence_version: 4, protein_evidence: ProteinLevel, mass: 69293, length: 607, gene: \"ALB\", id: \"P02769\", mnemonic: \"ALBU_BOVIN\", name: \"Serum albumin\", organism: \"Bos taurus\", proteome: \"UP000009136\", sequence: \"MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\", taxonomy: \"9913\" }]");
    }

    #[test]
    fn equality_list() {
        let l1 = vec![gapdh(), bsa()];
        let l2 = vec![gapdh(), bsa()];
        let l3 = vec![gapdh(), gapdh()];
        assert_eq!(l1, l2);
        assert_ne!(l1, l3);
        assert_ne!(l2, l3);
    }

    #[test]
    fn serialize_list() {
        let l = vec![gapdh(), bsa()];
        let x = serde_json::to_string(&l).unwrap();
        assert_eq!(x, "[{\"sequence_version\":3,\"protein_evidence\":1,\"mass\":35780,\"length\":333,\"gene\":\"GAPDH\",\"id\":\"P46406\",\"mnemonic\":\"G3P_RABIT\",\"name\":\"Glyceraldehyde-3-phosphate dehydrogenase\",\"organism\":\"Oryctolagus cuniculus\",\"proteome\":\"UP000001811\",\"sequence\":\"MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\",\"taxonomy\":\"9986\"},{\"sequence_version\":4,\"protein_evidence\":1,\"mass\":69293,\"length\":607,\"gene\":\"ALB\",\"id\":\"P02769\",\"mnemonic\":\"ALBU_BOVIN\",\"name\":\"Serum albumin\",\"organism\":\"Bos taurus\",\"proteome\":\"UP000009136\",\"sequence\":\"MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\",\"taxonomy\":\"9913\"}]");
    }

    #[test]
    fn fasta_list() {
        let v: RecordList = vec![gapdh(), bsa()];
        // to_fasta (valid, 2 items)
        let x = v.to_fasta().unwrap();
        assert_eq!(x, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\n\n>sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4\nMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPF\nDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEP\nERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYY\nANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVA\nRLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKE\nCCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRR\nHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEK\nLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLIL\nNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLP\nDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVV\nSTQTALA");
        assert_eq!(x, v.to_fasta_strict().unwrap());
        assert_eq!(x, v.to_fasta_lenient().unwrap());

        let y = RecordList::from_fasta(&x).unwrap();
        assert_eq!(y, RecordList::from_fasta_strict(&x).unwrap());
        assert_eq!(y, RecordList::from_fasta_lenient(&x).unwrap());
        assert_eq!(y[0].id, "P46406");
        assert_eq!(y[1].id, "P02769");

        assert!(v[0].is_valid());
        assert!(v[0].is_complete());

        assert!(v[1].is_valid());
        assert!(v[1].is_complete());

        assert!(y[0].is_valid());
        assert!(!y[0].is_complete());

        assert!(y[1].is_valid());
        assert!(!y[1].is_complete());

        // to_fasta (empty)
        let v: RecordList = vec![];
        let x = v.to_fasta().unwrap();
        assert_eq!(x, "");
        assert_eq!(x, v.to_fasta_strict().unwrap());
        assert_eq!(x, v.to_fasta_lenient().unwrap());

        let y = RecordList::from_fasta(&x).unwrap();
        assert_eq!(y, RecordList::from_fasta_strict(&x).unwrap());
        assert_eq!(y, RecordList::from_fasta_lenient(&x).unwrap());
        assert_eq!(y.len(), 0);

        // to_fasta (1 invalid)
        let v: RecordList = vec![Record::new()];
        let x = v.to_fasta();
        assert_eq!(x, Err("Invalid UniProt record, cannot serialize to FASTA."));
        assert_eq!(x, v.to_fasta_strict());
        assert_eq!(x, v.to_fasta_lenient());

        // to_fasta (1 valid, 1 invalid)
        let v: RecordList = vec![gapdh(), Record::new()];
        let x = v.to_fasta().unwrap();
        assert_eq!(x, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE");
        assert_eq!(v.to_fasta_strict(), Err("Invalid UniProt record, cannot serialize to FASTA."));
        assert_eq!(x, v.to_fasta_lenient().unwrap());
    }

    #[test]
    fn tbt_list() {
        // TODO(ahuszagh) Implement the TBT serializer test for lists.
    }

    // FETCH
    // TODO(ahuzagh)
    //      Need to implement the fetch tests here.

    use super::fetch;

    #[test]
    fn by_id() {
        fetch::by_id("P46406");
        // TODO(ahuszagh) implement
    }

    // by_id_list
    // by_mnemonic
    // by_mnemonic_list
}
