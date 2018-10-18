//! Private implementations for tab-delimited text routines.

use csv;
use std::collections::BTreeMap;
use std::io::{self, Read, Write};

use bio::proteins::{AverageMass, ProteinMass};
use util::ResultType;
use super::error::{new_boxed_error, UniProtErrorKind};
use super::record::{ProteinEvidence, protein_evidence_from_verbose,
                    protein_evidence_verbose, Record, RecordField};
//use super::record_list::RecordList;

// SHARED

/// Header `sequence_version`.
const SEQUENCE_VERSION: &'static str = "Sequence version";

/// Header `protein_evidence`.
const PROTEIN_EVIDENCE: &'static str = "Protein existence";

/// Header `mass`.
const MASS: &'static str = "Mass";

/// Header `length`.
const LENGTH: &'static str = "Length";

/// Header `gene`.
const GENE: &'static str = "Gene names  (primary )";

/// Header `id`.
const ID: &'static str = "Entry";

/// Header `mnemonic`.
const MNEMONIC: &'static str = "Entry name";

/// Header `name`.
const NAME: &'static str = "Protein names";

/// Header `organism`.
const ORGANISM: &'static str = "Organism";

/// Header `proteome`.
const PROTEOME: &'static str = "Proteomes";

/// Header `sequence`.
const SEQUENCE: &'static str = "Sequence";

/// Header `taxonomy`.
const TAXONOMY: &'static str = "Organism ID";

// TO CSV

//// Header columns for UniProt CSV export format.
static HEADER: [&'static str; 12] = [
    SEQUENCE_VERSION,
    PROTEIN_EVIDENCE,
    MASS,
    LENGTH,
    GENE,
    ID,
    MNEMONIC,
    NAME,
    ORGANISM,
    PROTEOME,
    SEQUENCE,
    TAXONOMY
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
pub fn to_csv<T: Write>(writer: &mut T, slice: &[Record], delimiter: u8) -> ResultType<()> {
    // Create our custom writer.
    let mut csv_writer = csv::WriterBuilder::new()
        .delimiter(delimiter)
        .quote_style(csv::QuoteStyle::Necessary)
        .flexible(false)
        .from_writer(writer);

    // Serialize the header to TBT.
    csv_writer.write_record(&HEADER)?;

    // Serialize each row to TBT.
    for record in slice {
        csv_writer.write_record(&to_row(record))?;
    }

    Ok(())
}

// RECORD ITERATOR

/// Type for the record field index.
type RecordFieldIndex = BTreeMap<RecordField, usize>;

/// Return type for the CSV `next()`.
type CsvIterResult = Option<csv::Result<csv::StringRecord>>;

/// Helper function to parse the header from a record iterator.
fn parse_header(opt: CsvIterResult, map: &mut RecordFieldIndex) -> ResultType<()> {
    let row = match opt {
        None    => return Err(new_boxed_error(UniProtErrorKind::InvalidInputData)),
        Some(v) => v?,
    };

    for tup in row.iter().enumerate() {
        let (index, item) = tup;
        let key: RecordField = match item {
            SEQUENCE_VERSION    => RecordField::SequenceVersion,
            PROTEIN_EVIDENCE    => RecordField::ProteinEvidence,
            MASS                => RecordField::Mass,
            LENGTH              => RecordField::Length,
            GENE                => RecordField::Gene,
            ID                  => RecordField::Id,
            MNEMONIC            => RecordField::Mnemonic,
            NAME                => RecordField::Name,
            ORGANISM            => RecordField::Organism,
            PROTEOME            => RecordField::Proteome,
            SEQUENCE            => RecordField::Sequence,
            TAXONOMY            => RecordField::Taxonomy,
            _   => continue,
        };
        map.insert(key, index);
    }

    Ok(())
}

/// Helper function to return the next `Record` from the CSV iterator.
fn next(opt: CsvIterResult, map: &RecordFieldIndex) -> Option<io::Result<Record>> {
    // Get the next record, and short-circuit if None or an Error.
    let row = match opt? {
        Err(e)  => return Some(Err(From::from(e))),
        Ok(v)   => v,
    };

    let mut record = Record::new();
    for (key, index) in map.iter() {
        // We know the index is valid, since flexible is false.
        // Just unwrap().
        let value = row.get(*index).expect("Invalid index, dead code...");

        // match the key and diligently handle errors to percolate up
        match key {
            RecordField::SequenceVersion    => {
                match value {
                    ""  => record.sequence_version = 0,
                    _   => match value.parse::<u8>() {
                        Err(_e) => return Some(Err(From::from(io::ErrorKind::InvalidInput))),
                        Ok(v)   => record.sequence_version = v,
                    }
                }
            },

            RecordField::ProteinEvidence    => {
                match value {
                    ""  => record.protein_evidence = ProteinEvidence::Unknown,
                    _   => match protein_evidence_from_verbose(value) {
                        Err(_e) => return Some(Err(From::from(io::ErrorKind::InvalidInput))),
                        Ok(v)   => record.protein_evidence = v,
                    }
                }
            }

            RecordField::Mass               => {
                match value {
                    ""  => record.mass = 0,
                    _   => match value.parse::<u64>() {
                        Err(_e) => return Some(Err(From::from(io::ErrorKind::InvalidInput))),
                        Ok(v)   => record.mass = v,
                    }
                }
            },

            RecordField::Length             => {
                match value {
                    ""  => record.length = 0,
                    _   => match value.parse::<u32>() {
                        Err(_e) => return Some(Err(From::from(io::ErrorKind::InvalidInput))),
                        Ok(v)   => record.length = v,
                    }
                }
            },
            RecordField::Gene               => record.gene = String::from(value),
            RecordField::Id                 => record.id = String::from(value),
            RecordField::Mnemonic           => record.mnemonic = String::from(value),
            RecordField::Name               => record.name = String::from(value),
            RecordField::Organism           => record.organism = String::from(value),
            RecordField::Proteome           => record.proteome = String::from(value),
            RecordField::Sequence           => record.sequence = String::from(value),
            RecordField::Taxonomy           => record.taxonomy = String::from(value),
        }
    }

    // fix the mass if not present
    if record.mass == 0 && !record.sequence.is_empty() {
        let mass = AverageMass::protein_sequence_mass(record.sequence.as_bytes());
        record.mass = mass.round() as u64;
    }

    // fix the length if not present
    if record.length == 0 && !record.sequence.is_empty() {
        record.length = record.sequence.len() as u32;
    }

    Some(Ok(record))
}

/// Non-owning iterator to extract records lazily from a document.
pub struct RecordIter<'r, T: 'r + Read> {
    map: RecordFieldIndex,
    iter: csv::StringRecordsIntoIter<&'r mut T>
}

impl<'r, T: 'r + Read> RecordIter<'r, T> {
     /// Create new RecordIter from a reader.
    pub fn new(reader: &mut T, delimiter: u8) -> RecordIter<T> {
        RecordIter {
            map: RecordFieldIndex::new(),
            iter: csv::ReaderBuilder::new()
                .delimiter(delimiter)
                .flexible(false)
                .has_headers(false)
                .from_reader(reader)
                .into_records(),
        }
    }

    /// Parse the header to determine the fields for the map.
    pub fn parse_header(&mut self) -> ResultType<()> {
        parse_header(self.iter.next(), &mut self.map)
    }
}

impl<'r, T: 'r + Read> Iterator for RecordIter<'r, T> {
    type Item = io::Result<Record>;

    fn next(&mut self) -> Option<io::Result<Record>> {
        next(self.iter.next(), &self.map)
    }
}

/// Owning iterator to extract records lazily from a document.
pub struct RecordIntoIter<T: Read> {
    map: RecordFieldIndex,
    iter: csv::StringRecordsIntoIter<T>
}

impl<T: Read> RecordIntoIter<T> {
     /// Create new RecordIntoIter from a reader.
    pub fn new(reader: T, delimiter: u8) -> RecordIntoIter<T> {
        RecordIntoIter {
            map: RecordFieldIndex::new(),
            iter: csv::ReaderBuilder::new()
                .delimiter(delimiter)
                .flexible(false)
                .has_headers(false)
                .from_reader(reader)
                .into_records(),
        }
    }

    /// Parse the header to determine the fields for the map.
    pub fn parse_header(&mut self) -> ResultType<()> {
        parse_header(self.iter.next(), &mut self.map)
    }
}

impl<T: Read> Iterator for RecordIntoIter<T> {
    type Item = io::Result<Record>;

    fn next(&mut self) -> Option<io::Result<Record>> {
        next(self.iter.next(), &self.map)
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    // TODO(ahuszagh)   Implement
}
