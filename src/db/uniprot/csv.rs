//! Private implementations for tab-delimited text routines.

use csv;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::str as stdstr;

use bio::SequenceMass;
use bio::proteins::AverageMass;
use traits::*;
use util::*;
use super::evidence::ProteinEvidence;
use super::record::{Record, RecordField};
use super::record_list::RecordList;

// SHARED

/// Header `sequence_version`.
const SEQUENCE_VERSION: &'static [u8] = b"Version (sequence)";

/// Header `protein_evidence`.
const PROTEIN_EVIDENCE: &'static [u8] = b"Protein existence";

/// Header `mass`.
const MASS: &'static [u8] = b"Mass";

/// Header `length`.
const LENGTH: &'static [u8] = b"Length";

/// Header `gene`.
const GENE: &'static [u8] = b"Gene names  (primary )";

/// Header `id`.
const ID: &'static [u8] = b"Entry";

/// Header `mnemonic`.
const MNEMONIC: &'static [u8] = b"Entry name";

/// Header `name`.
const NAME: &'static [u8] = b"Protein names";

/// Header `organism`.
const ORGANISM: &'static [u8] = b"Organism";

/// Header `proteome`.
const PROTEOME: &'static [u8] = b"Proteomes";

/// Header `sequence`.
const SEQUENCE: &'static [u8] = b"Sequence";

/// Header `taxonomy`.
const TAXONOMY: &'static [u8] = b"Organism ID";

/// Header `reviewed`.
const REVIEWED: &'static [u8] = b"Status";

// TO CSV HELPERS

//// Header columns for UniProt CSV export format.
const CSV_HEADER: [&'static [u8]; 13] = [
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
    TAXONOMY,
    REVIEWED
];

/// Convert a record to an array of strings for CSV serialization.
fn to_csv<T: Write>(writer: &mut csv::Writer<T>, record: &Record)
    -> Result<()>
{
    // Export values with the thousands separator.
    let sv = nonzero_to_comma_bytes(&record.sequence_version)?;
    let mass = nonzero_to_comma_bytes(&record.mass)?;
    let length = nonzero_to_comma_bytes(&record.length)?;
    let reviewed: &'static [u8] = match record.reviewed {
        true    => b"reviewed",
        false   => b"unreviewed",
    };
    let array: [&[u8]; 13] = [
        sv.as_slice(),
        record.protein_evidence.verbose_bytes(),
        mass.as_slice(),
        length.as_slice(),
        record.gene.as_bytes(),
        record.id.as_bytes(),
        record.mnemonic.as_bytes(),
        record.name.as_bytes(),
        record.organism.as_bytes(),
        record.proteome.as_bytes(),
        record.sequence.as_slice(),
        record.taxonomy.as_bytes(),
        reviewed,
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
type CsvIterResult = Option<csv::Result<csv::ByteRecord>>;

/// Helper function to parse the header from a record iterator.
fn parse_header(opt: CsvIterResult, map: &mut RecordFieldIndex)
    -> Result<()>
{
    let row = none_to_error!(opt, InvalidInput)?;

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
            REVIEWED            => RecordField::Reviewed,
            _   => continue,
        };
        map.insert(key, index);
    }

    Ok(())
}

/// Specialized macro to load a field from comma-separated text.
macro_rules! load_from_commas {
    ($value:ident, $t:ty) => (match nonzero_from_comma_bytes::<$t>($value) {
        Err(e) => return Some(Err(From::from(e))),
        Ok(v)  => v,
    })
}

/// Specialized macro to handle errors while loading text from UTF-8.
macro_rules! load_as_utf8 {
    // We cannot guarantee the validity of the records! Be safe!
    ($bytes:expr) => (match stdstr::from_utf8($bytes) {
        Err(e)  => return Some(Err(From::from(e))),
        Ok(v)   => String::from(v),
    })
}

/// Specialized macro to load protein evidence.
macro_rules! load_evidence {
    ($bytes:expr) => (match ProteinEvidence::from_verbose_bytes($bytes) {
        Err(e)  => return Some(Err(e)),
        Ok(v)   => v,
    })
}

/// Specialized macro to load protein reviewed status.
macro_rules! load_reviewed {
    ($bytes:expr) => (match $bytes {
        b"reviewed"     => true,
        b"unreviewed"   => false,
        _               => return Some(Err(From::from(ErrorKind::InvalidEnumeration))),
    })
}

/// Helper function to return the next `Record` from the CSV iterator.
fn next(opt: CsvIterResult, map: &RecordFieldIndex)
    -> Option<Result<Record>>
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

        // Match the key and diligently handle errors to percolate up
        match key {
            RecordField::SequenceVersion => record.sequence_version = load_from_commas!(value, u8),
            RecordField::ProteinEvidence => record.protein_evidence = load_evidence!(value),
            RecordField::Mass            => record.mass = load_from_commas!(value, u64),
            RecordField::Length          => record.length = load_from_commas!(value, u32),
            RecordField::Gene            => record.gene = load_as_utf8!(value),
            RecordField::Id              => record.id = load_as_utf8!(value),
            RecordField::Mnemonic        => record.mnemonic = load_as_utf8!(value),
            RecordField::Name            => record.name = load_as_utf8!(value),
            RecordField::Organism        => record.organism = load_as_utf8!(value),
            RecordField::Proteome        => record.proteome = load_as_utf8!(value),
            RecordField::Sequence        => record.sequence = value.to_vec(),
            RecordField::Taxonomy        => record.taxonomy = load_as_utf8!(value),
            RecordField::Reviewed        => record.reviewed = load_reviewed!(value),
        }
    }

    // fix the mass if not present
    if record.mass == 0 && !record.sequence.is_empty() {
        let mass = AverageMass::total_sequence_mass(record.sequence.as_slice());
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
const CSV_HEADER_SIZE: usize = 144;

/// Estimate the size of a CSV row from a record.
#[inline]
fn estimate_record_size(record: &Record) -> usize {
    // The vocabulary size is actually 11, overestimate to adjust for
    // number export and enumeration exports.
    const CSV_VOCABULARY_SIZE: usize = 61;
    CSV_VOCABULARY_SIZE +
        record.gene.len() +
        record.id.len() +
        record.mnemonic.len() +
        record.name.len() +
        record.organism.len() +
        record.taxonomy.len() +
        record.sequence.len()
}

/// Estimate the size of a CSV export from list.
#[inline]
fn estimate_list_size(list: &RecordList) -> usize {
    list.iter().fold(0, |sum, x| sum + estimate_record_size(x))
}

// WRITER

/// Export record to CSV.
pub fn record_to_csv<T: Write>(writer: &mut T, record: &Record, delimiter: u8)
    -> Result<()>
{
    let mut writer = new_writer(writer, delimiter);
    writer.write_record(&CSV_HEADER)?;
    to_csv(&mut writer, record)?;
    Ok(())
}

// WRITER -- DEFAULT

#[inline(always)]
fn init_cb<T: Write>(writer: T, delimiter: u8)
    -> Result<csv::Writer<T>>
{
    let mut writer = new_writer(writer, delimiter);
    writer.write_record(&CSV_HEADER)?;
    Ok(writer)
}

#[inline(always)]
fn export_cb<'a, T: Write>(writer: &mut csv::Writer<T>, record: &'a Record)
    -> Result<()>
{
    to_csv(writer, record)
}

#[inline(always)]
fn dest_cb<T: Write>(_: &mut csv::Writer<T>)
    -> Result<()>
{
    Ok(())
}

/// Default export from a non-owning iterator to CSV.
#[inline(always)]
pub fn reference_iterator_to_csv<'a, Iter, T>(writer: &mut T, iter: Iter, delimiter: u8)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export(writer, iter, delimiter, &init_cb, &export_cb, &dest_cb)
}

/// Default exporter from an owning iterator to FASTA.
#[inline(always)]
pub fn value_iterator_to_csv<Iter, T>(writer: &mut T, iter: Iter, delimiter: u8)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    value_iterator_export(writer, iter, delimiter, &init_cb, &export_cb, &dest_cb)
}

// WRITER -- STRICT

/// Strict export from a non-owning iterator to CSV.
#[inline(always)]
pub fn reference_iterator_to_csv_strict<'a, Iter, T>(writer: &mut T, iter: Iter, delimiter: u8)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_strict(writer, iter, delimiter, &init_cb, &export_cb, &dest_cb)
}

/// Strict exporter from an owning iterator to FASTA.
#[inline(always)]
pub fn value_iterator_to_csv_strict<Iter, T>(writer: &mut T, iter: Iter, delimiter: u8)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    value_iterator_export_strict(writer, iter, delimiter, &init_cb, &export_cb, &dest_cb)
}

// WRITER -- LENIENT

/// Lenient export from a non-owning iterator to CSV.
#[inline(always)]
pub fn reference_iterator_to_csv_lenient<'a, Iter, T>(writer: &mut T, iter: Iter, delimiter: u8)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_lenient(writer, iter, delimiter, &init_cb, &export_cb, &dest_cb)
}

/// Lenient exporter from an owning iterator to FASTA.
#[inline(always)]
pub fn value_iterator_to_csv_lenient<Iter, T>(writer: &mut T, iter: Iter, delimiter: u8)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    value_iterator_export_lenient(writer, iter, delimiter, &init_cb, &export_cb, &dest_cb)
}

// READER

/// Import record from CSV.
#[inline(always)]
pub fn record_from_csv<T: Read>(reader: &mut T, delimiter: u8)
    -> Result<Record>
{
    Ok(none_to_error!(iterator_from_csv(reader, delimiter).next(), InvalidInput)?)
}

// READER -- DEFAULT

/// Iterator to lazily load `Record`s from a document.
pub struct CsvRecordIter<T: Read> {
    map: RecordFieldIndex,
    iter: csv::ByteRecordsIntoIter<T>,
    has_map: bool,
}

impl<T: Read> CsvRecordIter<T> {
     /// Create new CsvRecordIter from a reader.
    #[inline]
    pub fn new(reader: T, delimiter: u8) -> Self {
        CsvRecordIter {
            map: RecordFieldIndex::new(),
            iter: new_reader(reader, delimiter).into_byte_records(),
            has_map: false,
        }
    }

    /// Parse the header to determine the fields for the map.
    #[inline]
    fn parse_header(&mut self) -> Result<()> {
        // Do not set `has_map` until the headers are parsed.
        parse_header(self.iter.next(), &mut self.map)?;
        self.has_map = true;
        Ok(())
    }
}

impl<T: Read> Iterator for CsvRecordIter<T> {
    type Item = Result<Record>;

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
pub type CsvRecordStrictIter<T> = StrictIter<Record, CsvRecordIter<T>>;

/// Create strict record iterator from reader.
#[inline(always)]
pub fn iterator_from_csv_strict<T: Read>(reader: T, delimiter: u8) -> CsvRecordStrictIter<T> {
    CsvRecordStrictIter::new(iterator_from_csv(reader, delimiter))
}

// READER -- LENIENT

/// Iterator to lazily load `Record`s from a document.
pub type CsvRecordLenientIter<T> = LenientIter<Record, CsvRecordIter<T>>;

/// Create lenient record iterator from reader.
#[inline(always)]
pub fn iterator_from_csv_lenient<T: Read>(reader: T, delimiter: u8) -> CsvRecordLenientIter<T> {
    CsvRecordLenientIter::new(iterator_from_csv(reader, delimiter))
}

// TRAITS

impl Csv for Record {
    #[inline(always)]
    fn estimate_csv_size(&self) -> usize {
        CSV_HEADER_SIZE + estimate_record_size(self)
    }

    #[inline(always)]
    fn to_csv<T: Write>(&self, writer: &mut T, delimiter: u8) -> Result<()> {
        record_to_csv(writer, self, delimiter)
    }

    #[inline(always)]
    fn from_csv<T: Read>(reader: &mut T, delimiter: u8) -> Result<Self> {
        record_from_csv(reader, delimiter)
    }
}

impl Csv for RecordList {
    #[inline(always)]
    fn estimate_csv_size(&self) -> usize {
        CSV_HEADER_SIZE + estimate_list_size(self)
    }

    #[inline(always)]
    fn to_csv<T: Write>(&self, writer: &mut T, delimiter: u8) -> Result<()> {
        reference_iterator_to_csv(writer, self.iter(), delimiter)
    }

    #[inline(always)]
    fn from_csv<T: Read>(reader: &mut T, delimiter: u8) -> Result<RecordList> {
        iterator_from_csv(reader, delimiter).collect()
    }
}

impl CsvCollection for RecordList {
    #[inline(always)]
    fn to_csv_strict<T: Write>(&self, writer: &mut T, delimiter: u8) -> Result<()> {
        reference_iterator_to_csv_strict(writer, self.iter(), delimiter)
    }

    #[inline(always)]
    fn to_csv_lenient<T: Write>(&self, writer: &mut T, delimiter: u8) -> Result<()> {
        reference_iterator_to_csv_lenient(writer, self.iter(), delimiter)
    }

    #[inline(always)]
    fn from_csv_strict<T: Read>(reader: &mut T, delimiter: u8) -> Result<RecordList> {
        iterator_from_csv_strict(reader, delimiter).collect()
    }

    #[inline(always)]
    fn from_csv_lenient<T: Read>(reader: &mut T, delimiter: u8) -> Result<RecordList> {
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
        assert_eq!(estimate_record_size(&g), 479);
        assert_eq!(estimate_record_size(&b), 714);
        assert_eq!(estimate_list_size(&v), 1193);
    }

    #[test]
    fn iterator_to_csv_test() {
        let v = vec![gapdh(), bsa()];
        let u = vec![gapdh(), bsa(), Record::new()];

        // reference -- default
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_csv(&mut w, v.iter(), b'\t').unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_CSV_TAB);

        // value -- default
        let mut w = Cursor::new(vec![]);
        value_iterator_to_csv(&mut w, iterator_by_value!(v.iter()), b'\t').unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_CSV_TAB);

        // reference -- strict
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_csv_strict(&mut w, v.iter(), b'\t').unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_CSV_TAB);

        let mut w = Cursor::new(vec![]);
        let r = reference_iterator_to_csv_strict(&mut w, u.iter(), b'\t');
        assert!(r.is_err());

        // value -- strict
        let mut w = Cursor::new(vec![]);
        value_iterator_to_csv_strict(&mut w, iterator_by_value!(v.iter()), b'\t').unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_CSV_TAB);

        let mut w = Cursor::new(vec![]);
        let r = value_iterator_to_csv_strict(&mut w, iterator_by_value!(u.iter()), b'\t');
        assert!(r.is_err());

        // reference -- lenient
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_csv_lenient(&mut w, v.iter(), b'\t').unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_CSV_TAB);

        let mut w = Cursor::new(vec![]);
        reference_iterator_to_csv_lenient(&mut w, u.iter(), b'\t').unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_CSV_TAB);

        // value -- lenient
        let mut w = Cursor::new(vec![]);
        value_iterator_to_csv_lenient(&mut w, iterator_by_value!(v.iter()), b'\t').unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_CSV_TAB);

        let mut w = Cursor::new(vec![]);
        value_iterator_to_csv_lenient(&mut w, iterator_by_value!(u.iter()), b'\t').unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_CSV_TAB);
    }

    #[test]
    fn iterator_from_csv_test() {
        // VALID
        let text = GAPDH_BSA_CSV_TAB;
        let expected = vec![gapdh(), bsa()];

        // record iterator -- default
        let iter = CsvRecordIter::new(Cursor::new(text), b'\t');
        let v: Result<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());

        // Compile check only
        iterator_from_csv(&mut Cursor::new(text), b'\t');

        // record iterator -- strict
        let iter = iterator_from_csv_strict(Cursor::new(text), b'\t');
        let v: Result<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());

        // Compile check only
        iterator_from_csv_strict(&mut Cursor::new(text), b'\t');

        // record iterator -- lenient
        let iter = iterator_from_csv_lenient(Cursor::new(text), b'\t');
        let v: Result<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());

        // Compile check only
        iterator_from_csv_lenient(&mut Cursor::new(text), b'\t');

        // INVALID
        let text = GAPDH_EMPTY_CSV_TAB;
        let expected1 = vec![gapdh(), Record::new()];
        let expected2 = vec![gapdh()];

        // record iterator -- default
        let iter = iterator_from_csv(Cursor::new(text), b'\t');
        let v: Result<RecordList> = iter.collect();
        let v = v.unwrap();
        assert_eq!(expected1, v);

        // record iterator -- strict
        let iter = iterator_from_csv_strict(Cursor::new(text), b'\t');
        let v: Result<RecordList> = iter.collect();
        assert!(v.is_err());

        // record iterator -- lenient
        let iter = iterator_from_csv_lenient(Cursor::new(text), b'\t');
        let v: Result<RecordList> = iter.collect();
        assert_eq!(expected2, v.unwrap());
    }
}
