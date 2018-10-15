/**
 *  UniProt
 *  -------
 *
 *  Record definitions for the UniProt KB service.
 *
 *  :copyright: (c) 2018 Alex Huszagh.
 *  :license: MIT, see LICENSE.md for more details.
 */

use std::error::Error;
use std::fmt;

use ::csv;
use ::ref_slice::ref_slice;
use ::serde_json;

use alias::ResultType;
use complete::Complete;
use fasta::{Fasta, FastaCollection};
use proteins::AverageMass;
use proteins::ProteinMass;
use tbt::{Tbt};       // TbtCollection
//use text::{Text, TextCollection};

// UNIPROT
// -------


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
#[allow(dead_code)]
pub fn process_organism(organism: &str) -> &str {
    // TODO(ahuszagh)
    //      Move Regular expressions to utility.
//    lazy_static! {
//            static ref ORGANISM_REGEX: Regex = Regex::new(r"(?x)
//                \A
//                (?P<genus>[A-Z][a-z]+)      # Genus (generic) name for species
//                \s                          # Word boundary
//                (?P<species>[A-Z][a-z]+)    # Specific name for species
//
//                # TODO: implement here...
//                #   Need the strain catcher
//                \z
//            ").unwrap();
//        }

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


impl Valid for Record {
    fn is_valid(&self) -> bool {
        // regular expression for the UniProt accession number provided by:
        //  https://www.uniprot.org/help/accession_numbers
//        lazy_static! {
//            static ref ACCESSION_REGEX: Regex = Regex::new(r"(?x)
//                \A
//                (?:
//                    [OPQ][0-9][A-Z0-9]{3}[0-9]|
//                    [A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2}
//                )
//                \z
//            ").unwrap();
//        }
//
//        lazy_static! {
//            static ref MNEMONIC_REGEX: Regex = Regex::new(r"(?x)
//                \A
//                (?:
//                    [a-zA-Z0-9]{1,5}
//                    _
//                    [a-zA-Z0-9]{1,5}
//                )
//                \z
//            ").unwrap();
//        }

        lazy_static! {
            static ref ORGANISM_REGEX: Regex = Regex::new(r"(?x)
                \A
                [[:alpha:]]+\x20[[:alpha:]]+
                \z
            ").unwrap();
        }

//        lazy_static! {
//            static ref AMINOACID_REGEX: Regex = Regex::new(r"(?x)
//                \A
//                [ABCDEFGHIJKLMNPQRSTVWXYZabcdefghijklmnpqrstvwxyz]*
//                \z
//            ").unwrap();
//        }
//
//        {
//            self.sequence_version >= 1 &&
//            self.protein_evidence < ProteinEvidence::Unknown &&
//            self.mass > 0 &&
//            self.length as usize == self.sequence.len() &&
//            self.sequence.len() > 0 &&
//            self.gene.len() > 0 &&
//            self.name.len() > 0 &&
//            ACCESSION_REGEX.is_match(&self.id) &&
//            MNEMONIC_REGEX.is_match(&self.mnemonic) &&
//            ORGANISM_REGEX.is_match(&self.organism) &&
//            AMINOACID_REGEX.is_match(&self.sequence)
//        }
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
    fn to_fasta(&self) -> ResultType<String> {
        if !self.is_valid() {
            let e = UniProtError(UniProtErrorKind::InvalidRecord);
            return Err(Box::new(e));
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
    fn from_fasta(fasta: &str) -> ResultType<Record> {
        // split along lines
        // first line is the header, rest are the sequences
        // short-circuit if the header is None.
        let mut lines = fasta.lines();
        let header_option = lines.next();
        if header_option.is_none() {
            let e = UniProtError(UniProtErrorKind::InvalidInputData);
            return Err(Box::new(e));
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
            let e = UniProtError(UniProtErrorKind::InvalidInputData);
            return Err(Box::new(e));
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
    fn to_tbt(&self) -> ResultType<String> {
        _slice_to_tbt(ref_slice(&self))
    }

    /**
     *  \brief Import UniProt record from a TBT row.
     */
    fn from_tbt(text: &str) -> ResultType<Record> {
        // TODO(ahuszagh) Implement...
        // 1. Need to find only the first 2 lines.
        // 2. Need to call the deserializer.
        // 3. Need to yank just the first item.

        //_text_to_list(text)[0];
        let _text = text;
        Err(From::from(""))
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
    fn to_fasta_strict(&self) -> ResultType<String> {
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
    fn to_fasta_lenient(&self) -> ResultType<String> {
        // exit early if empty list
        if self.is_empty() {
            return Ok(String::new());
        }

        // construct FASTA records from elements
        let mut v: Vec<String> = vec![];
        let mut e: Box<Error> = From::from("");
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
    fn from_fasta_strict(fasta: &str) -> ResultType<RecordList> {
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
    fn from_fasta_lenient(fasta: &str) -> ResultType<RecordList> {
        // exit early if empty input data
        if fasta.is_empty() {
            return Ok(RecordList::new());
        }

        // import records from FASTA
        let mut v: RecordList = vec![];
        let mut e: Box<Error> = From::from("");
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
    fn to_fasta(&self) -> ResultType<String> {
        self.to_fasta_lenient()
    }

    /**
     *  \brief Import UniProt record list from SwissProt FASTA records.
     */
    fn from_fasta(fasta: &str) -> ResultType<RecordList> {
        RecordList::from_fasta_lenient(fasta)
    }
}

impl Tbt for RecordList {
    /**
     *  \brief Export UniProt records to TBT.
     */
    fn to_tbt(&self) -> ResultType<String> {
        _slice_to_tbt(&self[..])
    }

    /**
     *  \brief Import UniProt records from TBT.
     */
    fn from_tbt(text: &str) -> ResultType<RecordList> {
        // TODO(ahuszagh) Implement...
        // 1. Need to call the deserializer.
        // 2. Return values.

        //_text_to_list(text)[0];
        let _text = text;
        Err(From::from(""))
    }
}

// PRIVATE
// -------

// RECORD(S) TO TBT

/**
 *  \brief Export the header columns to TBT.
 */
fn _header_to_row() -> Vec<&'static str> {
    vec![
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
    ]
}

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
fn _slice_to_tbt(records: &[Record]) -> ResultType<String> {
    // Create our custom writer.
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::Necessary)
        .flexible(false)
        .from_writer(vec![]);

    // Serialize the header to TBT.
    writer.write_record(&_header_to_row())?;

    // Serialize each row to TBT.
    for record in records {
        writer.write_record(&_record_to_row(&record))?;
    }

    // Return a string from the writer bytes.
    Ok(String::from_utf8(writer.into_inner()?)?)
}

// RECORD(S) FROM TBT


// TODO(ahuszagh)
//      Likely remove
//      Need to implement other logic for conversion from TBT.
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

    use std::error::Error;
    use reqwest;
    use url::form_urlencoded;

    use alias::ResultType;
    use tbt::{Tbt};

    use super::RecordList;

    // API
    // ---

    /**
     *  \brief Request UniProt records by accession number.
     *
     *  \param ids      Single accession number (eg. P46406)
     */
    pub fn by_id(id: &str) -> ResultType<RecordList> {
        _by_id(id)
    }

    /**
     *  \brief Request UniProt records by accession numbers.
     *
     *  \param ids      Slice of accession numbers (eg. [P46406])
     */
    pub fn by_id_list(ids: &[&str]) -> ResultType<RecordList> {
        _by_id(&ids.join(" OR "))
    }

    /**
     *  \brief Request UniProt records by mnemonic.
     *
     *  \param ids      Single mnemonic (eg. G3P_RABBIT)
     */
    pub fn by_mnemonic(mnemonic: &str) -> ResultType<RecordList> {
        _by_mnemonic(mnemonic)
    }

    /**
     *  \brief Request UniProt records by mnemonics.
     *
     *  \param ids      Slice of mnemonics (eg. [G3P_RABBIT])
     */
    pub fn by_mnemonic_list(ids: &[&str]) -> ResultType<RecordList> {
        _by_mnemonic(&ids.join(" OR "))
    }

    // PRIVATE
    // -------

    // Helper function for calling the UniProt KB service.
    #[allow(unused_variables)]
    fn _call(query: &str) -> ResultType<RecordList> {
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

    fn _url_to_body(url: &str) -> ResultType<String> {
        _url_to_body_impl(url).map_err(|e| {
            Box::new(e) as Box<Error>
        })
    }

    // Helper function for requesting by accession number.
    fn _by_id(id: &str) -> ResultType<RecordList> {
        _call(&format!("id:{}", id))
    }

    // Helper function for requesting by mnemonic.
    fn _by_mnemonic(mnemonic: &str)-> ResultType<RecordList> {
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
        let x = v.to_fasta().err().unwrap();
        assert_eq!(format!("{}", x), "UniProt error: invalid record found, cannot serialize data");
        assert_eq!(format!("{}", x), format!("{}", v.to_fasta_strict().err().unwrap()));
        assert_eq!(format!("{}", x), format!("{}", v.to_fasta_lenient().err().unwrap()));

        // to_fasta (1 valid, 1 invalid)
        let v: RecordList = vec![gapdh(), Record::new()];
        let x = v.to_fasta().unwrap();
        assert_eq!(x, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE");
        assert_eq!(x, v.to_fasta_lenient().unwrap());

        let y = v.to_fasta_strict().err().unwrap();
        assert_eq!(format!("{}", y), "UniProt error: invalid record found, cannot serialize data");
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
