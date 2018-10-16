//! Private implementations for tab-delimited text routines.

use csv;
//use radix_trie;

use util::ResultType;
use super::record::{ProteinEvidence, protein_evidence_verbose, Record};
use super::record_list::RecordList;

// TO CSV

//// Header columns for UniProt CSV export format.
static HEADER: [&'static str; 12] = [
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
];

/// Convert a record to an array of strings for CSV serialization.
fn to_row(record: &Record) -> [String; 12] {
    [
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

/// Convert a slice of records to CSV.
#[allow(dead_code)]     // TODO: remove
fn to_csv(slice: &[Record]) -> ResultType<String> {
    // Create our custom writer.
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::Necessary)
        .flexible(false)
        .from_writer(vec![]);

    // Serialize the header to TBT.
    writer.write_record(&HEADER)?;

    // Serialize each row to TBT.
    for record in slice {
        writer.write_record(&to_row(record))?;
    }

    // Return a string from the writer bytes.
    Ok(String::from_utf8(writer.into_inner()?)?)
}


// FROM CSV

// TODO(ahuszagh)
//  Is a btree or hashmap faster for string lookups?


// TODO(ahuszagh)
//  The loaders really shouldn't take a string object, they should take a stream
///// Convert a slice of records to CSV.
//#[allow(dead_code)]     // TODO: remove
//fn from_csv(text: &str) -> ResultType<RecordList> {
//    // TODO(ahuszagh) Implement...
//    Err(From::from(""))
//}
