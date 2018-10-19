//! Private implementations for tab-delimited text routines.

use csv;
use std::collections::BTreeMap;
use std::io::{Read, Write};

use bio::proteins::{AverageMass, ProteinMass};
use traits::Csv;
use util::ResultType;
use super::error::UniProtErrorKind;
use super::evidence::ProteinEvidence;
use super::record::{Record, RecordField};
use super::record_list::RecordList;

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

// TO CSV HELPERS

//// Header columns for UniProt CSV export format.
pub static CSV_HEADER: [&'static str; 12] = [
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
pub fn item_to_csv<T: Write>(writer: &mut csv::Writer<&mut T>, record: &Record)
    -> ResultType<()>
{
    let sv = nonzero_to_string!(record.sequence_version);
    let mass = nonzero_to_string!(record.mass);
    let length = nonzero_to_string!(record.length);
    // TODO(ahuszagh)
    //  Avoid copying most of the strings...
    let array: [&str; 12] = [
        &sv,
        record.protein_evidence.verbose(),
        // TODO(ahuszagh)
        //  All numbers should have the thousands separator....
        &mass,
        &length,
        &record.gene,
        &record.id,
        &record.mnemonic,
        &record.name,
        &record.organism,
        &record.proteome,
        &record.sequence,
        &record.taxonomy,
    ];

    match writer.write_record(&array) {
        Err(e)  => Err(From::from(e)),
        _       => Ok(())
    }
}


/// Create CSV writer.
// TODO(ahuszagh)
//      Change the `new_writer`
#[inline(always)]
pub fn csv_writer<T: Write>(writer: &mut T, delimiter: u8)
    -> csv::Writer<&mut T>
{
    csv::WriterBuilder::new()
        .delimiter(delimiter)
        .quote_style(csv::QuoteStyle::Necessary)
        .flexible(false)
        .from_writer(writer)
}

// RECORD ITERATOR

/// Type for the record field index.
type RecordFieldIndex = BTreeMap<RecordField, usize>;

/// Return type for the CSV `next()`.
type CsvIterResult = Option<csv::Result<csv::StringRecord>>;

/// Helper function to parse the header from a record iterator.
fn parse_header(opt: CsvIterResult, map: &mut RecordFieldIndex)
    -> ResultType<()>
{
    let row = match opt {
        None    => return Err(From::from(UniProtErrorKind::InvalidInputData)),
        Some(v) => v?,
    };
    // TODO(ahuszagh)       Remove debug statements
    println!("{:?}", row);

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
fn next(opt: CsvIterResult, map: &RecordFieldIndex)
    -> Option<ResultType<Record>>
{
    // Get the next record, and short-circuit if None or an Error.
    let row = match opt? {
        Err(e)  => return Some(Err(From::from(e))),
        Ok(v)   => v,
    };
    // TODO(ahuszagh)       // Remove
    println!("{:?}", row);

    let mut record = Record::new();
    for (key, index) in map.iter() {
        // We know the index is valid, since flexible is false.
        // Just unwrap().
        let value = row.get(*index).expect("Invalid index, dead code...");

        // match the key and diligently handle errors to percolate up
        match key {
            RecordField::SequenceVersion => {
                match nonzero_from_string!(value, u8) {
                    Err(e)  => return Some(Err(From::from(e))),
                    Ok(v)   => record.sequence_version = v,
                }
            },

            RecordField::ProteinEvidence => {
                match ProteinEvidence::from_verbose(value) {
                    Err(e)  => return Some(Err(e)),
                    Ok(v)   => record.protein_evidence = v,
                }
            }

            // TODO(ahuszagh)
            //  Need to strip any ","s
            RecordField::Mass => {
                match nonzero_from_string!(value, u64) {
                    Err(e)  => return Some(Err(From::from(e))),
                    Ok(v)   => record.mass = v,
                }
            },

            // TODO(ahuszagh)
            //  Need to strip any ","s
            RecordField::Length => {
                match nonzero_from_string!(value, u32) {
                    Err(e)  => return Some(Err(From::from(e))),
                    Ok(v)   => record.length = v,
                }
            },
            RecordField::Gene     => record.gene = String::from(value),
            RecordField::Id       => record.id = String::from(value),
            RecordField::Mnemonic => record.mnemonic = String::from(value),
            RecordField::Name     => record.name = String::from(value),
            RecordField::Organism => record.organism = String::from(value),
            RecordField::Proteome => record.proteome = String::from(value),
            RecordField::Sequence => record.sequence = String::from(value),
            RecordField::Taxonomy => record.taxonomy = String::from(value),
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
    type Item = ResultType<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        next(self.iter.next(), &self.map)
    }
}

/// Owning iterator to extract records lazily from a document.
#[allow(dead_code)] // TODO(ahuszagh)       Remove
pub struct RecordIntoIter<T: Read> {
    map: RecordFieldIndex,
    iter: csv::StringRecordsIntoIter<T>
}

impl<T: Read> RecordIntoIter<T> {
     /// Create new RecordIntoIter from a reader.
    #[allow(dead_code)] // TODO(ahuszagh)       Remove
    pub fn new(reader: T, delimiter: u8) -> Self {
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
    #[allow(dead_code)] // TODO(ahuszagh)       Remove
    pub fn parse_header(&mut self) -> ResultType<()> {
        parse_header(self.iter.next(), &mut self.map)
    }
}

impl<T: Read> Iterator for RecordIntoIter<T> {
    type Item = ResultType<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        next(self.iter.next(), &self.map)
    }
}

// SIZE

/// Estimated size of the CSV header.
const CSV_HEADER_SIZE: usize = 142;

/// Estimate the size of a  CSV row from a record.
pub fn estimate_record_size(record: &Record) -> usize {
    // Number of "\t" delimiters per row.
    const CSV_VOCABULARY_SIZE: usize = 11;
    CSV_VOCABULARY_SIZE +
        record.gene.len() +
        record.id.len() +
        record.mnemonic.len() +
        record.name.len() +
        record.organism.len() +
        record.sequence.len()
}

// WRITER

/// Export record to FASTA.
pub fn record_to_csv<T: Write>(record: &Record, writer: &mut T, delimiter: u8)
    -> ResultType<()>
{
    let mut writer = csv_writer(writer, delimiter);
    writer.write_record(&CSV_HEADER)?;
    item_to_csv(&mut writer, record)?;
    Ok(())
}

// READER

/// Import record from CSV.
pub fn csv_to_record<T: Read>(reader: &mut T, delimiter: u8)
    -> ResultType<Record>
{
    let mut iter = RecordIter::new(reader, delimiter);
    iter.parse_header()?;
    match iter.next() {
        None    => Err(From::from(UniProtErrorKind::InvalidInputData)),
        Some(v) => Ok(v?)
    }
}

// TRAITS

impl Csv for Record {
    #[inline]
    fn estimate_csv_size(&self) -> usize {
        CSV_HEADER_SIZE + estimate_record_size(self)
    }

    fn to_csv<T: Write>(&self, writer: &mut T, delimiter: u8) -> ResultType<()> {
        record_to_csv(self, writer, delimiter)
    }

    fn from_csv<T: Read>(reader: &mut T, delimiter: u8) -> ResultType<Self> {
        csv_to_record(reader, delimiter)
    }
}

impl Csv for RecordList {
    #[inline]
    fn estimate_csv_size(&self) -> usize {
        CSV_HEADER_SIZE + self.iter().fold(0, |sum, x| sum + estimate_record_size(x))
    }

    fn to_csv<T: Write>(&self, writer: &mut T, delimiter: u8) -> ResultType<()> {
        // TODO(ahuszagh)
        //  Simplify using traits
        let mut writer = csv_writer(writer, delimiter);
        writer.write_record(&CSV_HEADER)?;
        for record in self {
            item_to_csv(&mut writer, record)?;
        }
        Ok(())
    }

    fn from_csv<T: Read>(reader: &mut T, delimiter: u8) -> ResultType<RecordList> {
        // TODO(ahuszagh)
        //  Simplify using traits
        let mut iter = RecordIter::new(reader, delimiter);
        iter.parse_header()?;
        iter.collect()
    }
}

//impl CsvCollection for RecordList {
//}

// TESTS
// -----

#[cfg(test)]
mod tests {
    // TODO(ahuszagh)   Implement
}
