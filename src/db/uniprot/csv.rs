//! Private implementations for tab-delimited text routines.

use csv;
use digit_group::FormatGroup;
use std::collections::BTreeMap;
use std::io::{Read, Write};

use bio::proteins::{AverageMass, ProteinMass};
use traits::*;
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
const CSV_HEADER: [&'static str; 12] = [
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
fn item_to_csv<T: Write>(writer: &mut csv::Writer<&mut T>, record: &Record)
    -> ResultType<()>
{
    // Export values with the thousands separator.
    let sv = nonzero_to_commas!(record.sequence_version);
    let mass = nonzero_to_commas!(record.mass);
    let length = nonzero_to_commas!(record.length);
    let array: [&str; 12] = [
        &sv,
        record.protein_evidence.verbose(),
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
#[inline(always)]
fn new_writer<T: Write>(writer: T, delimiter: u8)
    -> csv::Writer<T>
{
    csv::WriterBuilder::new()
        .delimiter(delimiter)
        .quote_style(csv::QuoteStyle::Necessary)
        .flexible(false)
        .from_writer(writer)
}

/// Create CSV reader.
#[inline(always)]
fn new_reader<T: Read>(reader: T, delimiter: u8)
    -> csv::Reader<T>
{
    csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .flexible(false)
        .from_reader(reader)
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
        None    => return Err(From::from(UniProtErrorKind::InvalidInput)),
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
fn next(opt: CsvIterResult, map: &RecordFieldIndex)
    -> Option<ResultType<Record>>
{
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
            RecordField::SequenceVersion => {
                match nonzero_from_commas!(value, u8) {
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

            RecordField::Mass => {
                match nonzero_from_commas!(value, u64) {
                    Err(e)  => return Some(Err(From::from(e))),
                    Ok(v)   => record.mass = v,
                }
            },

            RecordField::Length => {
                match nonzero_from_commas!(value, u32) {
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

// SIZE

/// Estimated size of the CSV header.
const CSV_HEADER_SIZE: usize = 142;

/// Estimate the size of a CSV row from a record.
#[inline]
pub fn estimate_record_size(record: &Record) -> usize {
    // The vocabulary size is actually 11, overestimate to adjust for number export.
    const CSV_VOCABULARY_SIZE: usize = 31;
    CSV_VOCABULARY_SIZE +
        record.gene.len() +
        record.id.len() +
        record.mnemonic.len() +
        record.name.len() +
        record.organism.len() +
        record.sequence.len()
}

/// Estimate the size of a CSV export from list.
#[inline]
pub fn estimate_list_size(list: &RecordList) -> usize {
    list.iter().fold(0, |sum, x| sum + estimate_record_size(x))
}

// WRITER

/// Export record to CSV.
pub fn record_to_csv<T: Write>(record: &Record, writer: &mut T, delimiter: u8)
    -> ResultType<()>
{
    let mut writer = new_writer(writer, delimiter);
    writer.write_record(&CSV_HEADER)?;
    item_to_csv(&mut writer, record)?;
    Ok(())
}

// WRITER -- DEFAULT

/// Default export from a non-owning iterator to CSV.
pub fn reference_iterator_to_csv<'a, Iter, T>(iter: Iter, writer: &mut T, delimiter: u8)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    let mut writer = new_writer(writer, delimiter);
    writer.write_record(&CSV_HEADER)?;
    for record in iter {
        item_to_csv(&mut writer, record)?;
    }
    Ok(())
}

/// Default exporter from an owning iterator to FASTA.
pub fn value_iterator_to_csv<Iter, T>(iter: Iter, writer: &mut T, delimiter: u8)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    let mut writer = new_writer(writer, delimiter);
    writer.write_record(&CSV_HEADER)?;
    for record in iter {
        item_to_csv(&mut writer, &record?)?;
    }
    Ok(())
}

// WRITER -- STRICT

/// Strict export from a non-owning iterator to CSV.
pub fn reference_iterator_to_csv_strict<'a, Iter, T>(iter: Iter, writer: &mut T, delimiter: u8)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    let mut writer = new_writer(writer, delimiter);
    writer.write_record(&CSV_HEADER)?;
    for record in iter {
        if record.is_valid() {
            item_to_csv(&mut writer, record)?;
        } else {
            return Err(From::from(UniProtErrorKind::InvalidRecord));
        }
    }
    Ok(())
}

/// Strict exporter from an owning iterator to FASTA.
pub fn value_iterator_to_csv_strict<Iter, T>(iter: Iter, writer: &mut T, delimiter: u8)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    let mut writer = new_writer(writer, delimiter);
    writer.write_record(&CSV_HEADER)?;
    for result in iter {
        let record = result?;
        if record.is_valid() {
            item_to_csv(&mut writer, &record)?;
        } else {
            return Err(From::from(UniProtErrorKind::InvalidRecord));
        }
    }
    Ok(())
}

// WRITER -- LENIENT

/// Lenient export from a non-owning iterator to CSV.
pub fn reference_iterator_to_csv_lenient<'a, Iter, T>(iter: Iter, writer: &mut T, delimiter: u8)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    let mut writer = new_writer(writer, delimiter);
    writer.write_record(&CSV_HEADER)?;
    for record in iter {
        if record.is_valid() {
            item_to_csv(&mut writer, record)?;
        }
    }
    Ok(())
}

/// Lenient exporter from an owning iterator to FASTA.
pub fn value_iterator_to_csv_lenient<Iter, T>(iter: Iter, writer: &mut T, delimiter: u8)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    let mut writer = new_writer(writer, delimiter);
    writer.write_record(&CSV_HEADER)?;
    for result in iter {
        let record = result?;
        if record.is_valid() {
            item_to_csv(&mut writer, &record)?;
        }
    }
    Ok(())
}

// READER

/// Import record from CSV.
pub fn record_from_csv<T: Read>(reader: &mut T, delimiter: u8)
    -> ResultType<Record>
{
    let mut iter = CsvRecordIter::new(reader, delimiter);
    match iter.next() {
        None    => Err(From::from(UniProtErrorKind::InvalidInput)),
        Some(v) => Ok(v?)
    }
}

// READER -- DEFAULT

/// Iterator to lazily load `Record`s from a document.
pub struct CsvRecordIter<T: Read> {
    map: RecordFieldIndex,
    iter: csv::StringRecordsIntoIter<T>,
    has_map: bool,
}

impl<T: Read> CsvRecordIter<T> {
     /// Create new CsvRecordIter from a reader.
    #[inline]
    pub fn new(reader: T, delimiter: u8) -> Self {
        CsvRecordIter {
            map: RecordFieldIndex::new(),
            iter: new_reader(reader, delimiter).into_records(),
            has_map: false,
        }
    }

    /// Parse the header to determine the fields for the map.
    #[inline]
    fn parse_header(&mut self) -> ResultType<()> {
        // Do not set `has_map` until the headers are parsed.
        parse_header(self.iter.next(), &mut self.map)?;
        self.has_map = true;
        Ok(())
    }
}

impl<T: Read> Iterator for CsvRecordIter<T> {
    type Item = ResultType<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        // Parse headers if they have not already been parsed
        if !self.has_map {
            match self.parse_header() {
                Err(e) => return Some(Err(e)),
                _      => (),
            }
        }
        next(self.iter.next(), &self.map)
    }
}

/// Create default record iterator from reader.
#[inline(always)]
pub fn iterator_from_csv<T: Read>(reader: T, delimiter: u8) -> CsvRecordIter<T> {
    CsvRecordIter::new(reader, delimiter)
}

// READER -- STRICT

/// Iterator to lazily load `Record`s from a document.
pub struct CsvRecordStrictIter<T: Read> {
    map: RecordFieldIndex,
    iter: csv::StringRecordsIntoIter<T>,
    has_map: bool,
}

impl<T: Read> CsvRecordStrictIter<T> {
     /// Create new CsvRecordStrictIter from a reader.
    #[inline]
    pub fn new(reader: T, delimiter: u8) -> Self {
        CsvRecordStrictIter {
            map: RecordFieldIndex::new(),
            iter: new_reader(reader, delimiter).into_records(),
            has_map: false,
        }
    }

    /// Parse the header to determine the fields for the map.
    #[inline]
    fn parse_header(&mut self) -> ResultType<()> {
        // Do not set `has_map` until the headers are parsed.
        parse_header(self.iter.next(), &mut self.map)?;
        self.has_map = true;
        Ok(())
    }
}

impl<T: Read> Iterator for CsvRecordStrictIter<T> {
    type Item = ResultType<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        // Parse headers if they have not already been parsed
        if !self.has_map {
            match self.parse_header() {
                Err(e) => return Some(Err(e)),
                _      => (),
            }
        }

        match next(self.iter.next(), &self.map)? {
            Err(e)  => Some(Err(e)),
            Ok(r)   => {
                if r.is_valid() {
                    Some(Ok(r))
                } else {
                    Some(Err(From::from(UniProtErrorKind::InvalidRecord)))
                }
            }
        }
    }
}

/// Create strict record iterator from reader.
#[inline(always)]
pub fn iterator_from_csv_strict<T: Read>(reader: T, delimiter: u8) -> CsvRecordStrictIter<T> {
    CsvRecordStrictIter::new(reader, delimiter)
}

// READER -- LENIENT

/// Iterator to lazily load `Record`s from a document.
pub struct CsvRecordLenientIter<T: Read> {
    map: RecordFieldIndex,
    iter: csv::StringRecordsIntoIter<T>,
    has_map: bool,
}

impl<T: Read> CsvRecordLenientIter<T> {
     /// Create new CsvRecordLenientIter from a reader.
    #[inline]
    pub fn new(reader: T, delimiter: u8) -> Self {
        CsvRecordLenientIter {
            map: RecordFieldIndex::new(),
            iter: new_reader(reader, delimiter).into_records(),
            has_map: false,
        }
    }

    /// Parse the header to determine the fields for the map.
    #[inline]
    fn parse_header(&mut self) -> ResultType<()> {
        // Do not set `has_map` until the headers are parsed.
        parse_header(self.iter.next(), &mut self.map)?;
        self.has_map = true;
        Ok(())
    }
}

impl<T: Read> Iterator for CsvRecordLenientIter<T> {
    type Item = ResultType<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        // Parse headers if they have not already been parsed
        if !self.has_map {
            match self.parse_header() {
                Err(e) => return Some(Err(e)),
                _      => (),
            }
        }

        loop {
            match next(self.iter.next(), &self.map)? {
                Err(e)  => return Some(Err(e)),
                Ok(r)   => {
                    if r.is_valid() {
                        return Some(Ok(r));
                    }
                },
            }
        }
    }
}

/// Create lenient record iterator from reader.
#[inline(always)]
pub fn iterator_from_csv_lenient<T: Read>(reader: T, delimiter: u8) -> CsvRecordLenientIter<T> {
    CsvRecordLenientIter::new(reader, delimiter)
}

// TRAITS

impl Csv for Record {
    #[inline(always)]
    fn estimate_csv_size(&self) -> usize {
        CSV_HEADER_SIZE + estimate_record_size(self)
    }

    #[inline(always)]
    fn to_csv<T: Write>(&self, writer: &mut T, delimiter: u8) -> ResultType<()> {
        record_to_csv(self, writer, delimiter)
    }

    #[inline(always)]
    fn from_csv<T: Read>(reader: &mut T, delimiter: u8) -> ResultType<Self> {
        record_from_csv(reader, delimiter)
    }
}

impl Csv for RecordList {
    #[inline(always)]
    fn estimate_csv_size(&self) -> usize {
        CSV_HEADER_SIZE + estimate_list_size(self)
    }

    #[inline(always)]
    fn to_csv<T: Write>(&self, writer: &mut T, delimiter: u8) -> ResultType<()> {
        reference_iterator_to_csv(self.iter(), writer, delimiter)
    }

    #[inline(always)]
    fn from_csv<T: Read>(reader: &mut T, delimiter: u8) -> ResultType<RecordList> {
        iterator_from_csv(reader, delimiter).collect()
    }
}

impl CsvCollection for RecordList {
    #[inline(always)]
    fn to_csv_strict<T: Write>(&self, writer: &mut T, delimiter: u8) -> ResultType<()> {
        reference_iterator_to_csv_strict(self.iter(), writer, delimiter)
    }

    #[inline(always)]
    fn to_csv_lenient<T: Write>(&self, writer: &mut T, delimiter: u8) -> ResultType<()> {
        reference_iterator_to_csv_lenient(self.iter(), writer, delimiter)
    }

    #[inline(always)]
    fn from_csv_strict<T: Read>(reader: &mut T, delimiter: u8) -> ResultType<RecordList> {
        iterator_from_csv_strict(reader, delimiter).collect()
    }

    #[inline(always)]
    fn from_csv_lenient<T: Read>(reader: &mut T, delimiter: u8) -> ResultType<RecordList> {
        Ok(iterator_from_csv_lenient(reader, delimiter).filter_map(Result::ok).collect())
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;
    use super::super::test::*;

    #[test]
    fn estimate_size_test() {
        let g = gapdh();
        let b = bsa();
        let v = vec![gapdh(), bsa()];
        assert_eq!(estimate_record_size(&g), 445);
        assert_eq!(estimate_record_size(&b), 680);
        assert_eq!(estimate_list_size(&v), 1125);
    }

    macro_rules! by_value {
        ($x:expr) => ($x.iter().map(|x| { Ok(x.clone()) }))
    }

    #[test]
    fn iterator_to_csv_test() {
        let v = vec![gapdh(), bsa()];
        let u = vec![gapdh(), bsa(), Record::new()];

        // reference -- default
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_csv(v.iter(), &mut w, b'\t').unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_CSV_TAB);

        // value -- default
        let mut w = Cursor::new(vec![]);
        value_iterator_to_csv(by_value!(v), &mut w, b'\t').unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_CSV_TAB);

        // reference -- strict
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_csv_strict(v.iter(), &mut w, b'\t').unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_CSV_TAB);

        let mut w = Cursor::new(vec![]);
        let r = reference_iterator_to_csv_strict(u.iter(), &mut w, b'\t');
        assert!(r.is_err());

        // value -- strict
        let mut w = Cursor::new(vec![]);
        value_iterator_to_csv_strict(by_value!(v), &mut w, b'\t').unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_CSV_TAB);

        let mut w = Cursor::new(vec![]);
        let r = value_iterator_to_csv_strict(by_value!(u), &mut w, b'\t');
        assert!(r.is_err());

        // reference -- lenient
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_csv_lenient(v.iter(), &mut w, b'\t').unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_CSV_TAB);

        let mut w = Cursor::new(vec![]);
        reference_iterator_to_csv_lenient(u.iter(), &mut w, b'\t').unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_CSV_TAB);

        // value -- lenient
        let mut w = Cursor::new(vec![]);
        value_iterator_to_csv_lenient(by_value!(v), &mut w, b'\t').unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_CSV_TAB);

        let mut w = Cursor::new(vec![]);
        value_iterator_to_csv_lenient(by_value!(u), &mut w, b'\t').unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_CSV_TAB);
    }

    #[test]
    fn iterator_from_csv_test() {
        // VALID
        let text = GAPDH_BSA_CSV_TAB;
        let expected = vec![gapdh(), bsa()];

        // record iterator -- default
        let iter = CsvRecordIter::new(Cursor::new(text), b'\t');
        let v: ResultType<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());

        // Compile check only
        iterator_from_csv(&mut Cursor::new(text), b'\t');

        // record iterator -- strict
        let iter = CsvRecordStrictIter::new(Cursor::new(text), b'\t');
        let v: ResultType<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());

        // Compile check only
        iterator_from_csv_strict(&mut Cursor::new(text), b'\t');

        // record iterator -- lenient
        let iter = CsvRecordLenientIter::new(Cursor::new(text), b'\t');
        let v: ResultType<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());

        // Compile check only
        iterator_from_csv_lenient(&mut Cursor::new(text), b'\t');

        // INVALID
        let text = GAPDH_EMPTY_CSV_TAB;
        let expected1 = vec![gapdh(), Record::new()];
        let expected2 = vec![gapdh()];

        // record iterator -- default
        let iter = iterator_from_csv(Cursor::new(text), b'\t');
        let v: ResultType<RecordList> = iter.collect();
        let v = v.unwrap();
        assert_eq!(expected1, v);

        // record iterator -- strict
        let iter = iterator_from_csv_strict(Cursor::new(text), b'\t');
        let v: ResultType<RecordList> = iter.collect();
        assert!(v.is_err());

        // record iterator -- lenient
        let iter = iterator_from_csv_lenient(Cursor::new(text), b'\t');
        let v: ResultType<RecordList> = iter.collect();
        assert_eq!(expected2, v.unwrap());
    }
}
