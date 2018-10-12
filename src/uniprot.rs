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
use std::fmt;

use ::serde_json;

use complete::Complete;
use fasta::{Fasta, FastaCollection};
use proteins::AverageMass;
use proteins::ProteinMass;
use valid::Valid;

// UNIPROT
// -------


/**
 *  \brief Identifier for the evidence type for protein existence.
 *
 *  An identifier used by biological databases for the level of evidence
 *  that supports a protein's existence. Strong evidence includes
 *  evidence at the protein level, while weaker evidence is evidence
 *  at the transcript (or mRNA) level. Weak evidence is inferred from
 *  homology from similar species. Curated protein databases frequently
 *  only include proteins identified at the protein level.
 */
enum_number!(ProteinEvidence {
    ProteinLevel = 1,
    TranscriptLevel = 2,
    Inferred = 3,
    Unknown = 4,
});

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
    fn from_fasta(fasta: &str) -> Result<Record, &str> {
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
    fn from_fasta_strict(fasta: &str) -> Result<RecordList, &str> {
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
    fn from_fasta_lenient(fasta: &str) -> Result<RecordList, &str> {
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
    fn from_fasta(fasta: &str) -> Result<RecordList, &str> {
        RecordList::from_fasta_lenient(fasta)
    }
}

// CONNECTION
// ----------

/**
 *  \brief Module to fetch records using the Uniprot KB service.
 */
pub mod fetch {

    // CONSTANTS
    // ---------

    const HOST: &str = "http://www.uniprot.org/uniprot/";

    // ALIAS
    // -----

    use hyper::Client;
    use hyper::Uri;
    use hyper::client::{ResponseFuture};
    use hyper::rt::{Future, run};
    use url::form_urlencoded;


    /**
     *  \brief Request UniProt records by accession number.
     *
     *  \param ids      Single accession number (eg. P46406)
     */
    pub fn by_id(id: &str) {
        _by_id(id)
    }

    /**
     *  \brief Request UniProt records by accession numbers.
     *
     *  \param ids      Slice of accession numbers (eg. [P46406])
     */
    pub fn by_id_list(ids: &[&str]) {
        _by_id(&ids.join(" OR "))
    }

    /**
     *  \brief Request UniProt records by mnemonic.
     *
     *  \param ids      Single mnemonic (eg. G3P_RABBIT)
     */
    pub fn by_mnemonic(mnemonic: &str) {
        _by_mnemonic(mnemonic)
    }

    /**
     *  \brief Request UniProt records by mnemonics.
     *
     *  \param ids      Slice of mnemonics (eg. [G3P_RABBIT])
     */
    pub fn by_mnemonic_list(ids: &[&str]) {
        _by_mnemonic(&ids.join(" OR "))
    }

    // Helper function for calling the UniProt KB service.
    #[allow(unused_variables)]
    fn _call(query: &str) /* -> Option<RecordList> */ {
        // create our url with form-encoded parameters
        let params = form_urlencoded::Serializer::new(String::new())
            .append_pair("sort", "score")
            .append_pair("desc", "")
            .append_pair("fil", "")
            .append_pair("force", "no")
            .append_pair("format", "tab")
            .append_pair("reviewed", "yes")
            .append_pair("query", query)
            .append_pair("columns", "id,entry name,genes(PREFERRED),sequence")
            .finish();
        let url = format!("{}?{}", HOST, params);
        println!("{}", url);        // TODO: remove
        let url = url.parse::<Uri>().unwrap();

        // we can block until we get the request
        //run(_fetch_url(url));

        // TODO: here....

        //let result = client.get(url)
            //.wait();
//        //  rt::spawn(fut);
//        match result {
//            Err(e) => {
//                // TODO: implement...
//                println!("Error: {}", e);   // TODO: remove
//            },
//            Ok(t) => {
//                // TODO: implement...
//                println!("Response!");      // TODO: remove
//            },
//        }
//            .and_then(|res| {
////                println!("Response: {}", res.status());
////                println!("Headers: {:#?}", res.headers());
////
////                // The body is a stream, and for_each returns a new Future
////                // when the stream is finished, and calls the closure on
////                // each chunk of the body...
////                res.into_body().for_each(|chunk| {
////                    //io::stdout().write_all(&chunk)
////                        //.map_err(|e| panic!("example expects stdout is open, error={}", e))
////                //})
//            })
//            // If all good, just tell the user...
//            .map(|_| {
//                println!("\n\nDone.");
//            })
//            // If there was an error, let the user know...
//            .map_err(|err| {
//                eprintln!("Error {}", err);
//            });
    }

    // TODO: probably need to change this into a future...
    fn _fetch_url(url: Uri) /*-> ResponseFuture*/ {
        let client = Client::new();
        client.get(url);
//            .and_then(|res| {
//                println!("Response: {}", res.status());
//                println!("Headers: {:#?}", res.headers());
//
//                // The body is a stream, and for_each returns a new Future
//                // when the stream is finished, and calls the closure on
//                // each chunk of the body...
//                res
////                res.into_body().for_each(|chunk| {
////                    io::stdout().write_all(&chunk)
////                        .map_err(|e| panic!("example expects stdout is open, error={}", e))
////                })
//            })
//            // If all good, just tell the user...
//            .map(|_| {
//                println!("\n\nDone.");
//            })
//            // If there was an error, let the user know...
//            .map_err(|err| {
//                eprintln!("Error {}", err);
//            })
    }

    // Helper function for requesting by accession number.
    fn _by_id(id: &str){
        _call(&format!("id:{}", id))
    }

    // Helper function for requesting by mnemonic.
    fn _by_mnemonic(mnemonic: &str){
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
    use complete::Complete;
    use fasta::{Fasta, FastaCollection};
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

    #[test]
    fn debug_protein_evidence() {
        let formatted_protein_level = format!("{:?}", ProteinEvidence::ProteinLevel);
        let formatted_transcript_level = format!("{:?}", ProteinEvidence::TranscriptLevel);
        let formatted_inferred = format!("{:?}", ProteinEvidence::Inferred);
        assert_eq!(formatted_protein_level, "ProteinLevel");
        assert_eq!(formatted_transcript_level, "TranscriptLevel");
        assert_eq!(formatted_inferred, "Inferred");
    }

    #[test]
    fn serialize_protein_evidence() {
        // to_string
        let x = serde_json::to_string(&ProteinEvidence::ProteinLevel).unwrap();
        assert_eq!(x, "1");

        let y = serde_json::to_string(&ProteinEvidence::TranscriptLevel).unwrap();
        assert_eq!(y, "2");

        let z = serde_json::to_string(&ProteinEvidence::Inferred).unwrap();
        assert_eq!(z, "3");

        // from_str
        let a: ProteinEvidence = serde_json::from_str(&x).unwrap();
        assert_eq!(a, ProteinEvidence::ProteinLevel);

        let b: ProteinEvidence = serde_json::from_str(&y).unwrap();
        assert_eq!(b, ProteinEvidence::TranscriptLevel);

        let c: ProteinEvidence = serde_json::from_str(&z).unwrap();
        assert_eq!(c, ProteinEvidence::Inferred);
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
