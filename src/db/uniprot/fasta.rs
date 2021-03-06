//! Helper utilities for FASTA loading and saving.

use std::io::prelude::*;

use bio::SequenceMass;
use bio::proteins::AverageMass;
use traits::*;
use util::*;
use super::re::*;
use super::record::Record;
use super::record_list::RecordList;

// FASTA ITERATOR

/// Iterator to parse individual FASTA entries from a document.
///
/// Convert a stream to a lazy reader that fetches individual FASTA entries
/// from the document.
pub struct FastaIter<T: BufRead> {
    reader: T,
    buf: Bytes,
    line: Bytes,
}

impl<T: BufRead> FastaIter<T> {
    /// Create new FastaIter from a buffered reader.
    #[inline]
    pub fn new(reader: T) -> Self {
        FastaIter {
            reader: reader,
            buf: Vec::with_capacity(8000),
            line: Vec::with_capacity(8000)
        }
    }
}

impl<T: BufRead> Iterator for FastaIter<T> {
    type Item = Result<Bytes>;

    fn next(&mut self) -> Option<Self::Item> {
        bytes_next_skip_whitespace(b">", &mut self.reader, &mut self.buf, &mut self.line)
    }
}

// SIZE

/// Estimate the size of a FASTA record.
///
/// Used to prevent reallocations during record exportation to string,
/// to minimize costly library calls.
#[inline]
fn estimate_record_size(record: &Record) -> usize {
    // The vocabulary size is actually 20, overestimate to adjust for number export.
    const FASTA_VOCABULARY_SIZE: usize = 40;
    FASTA_VOCABULARY_SIZE +
        record.gene.len() +
        record.id.len() +
        record.mnemonic.len() +
        record.name.len() +
        record.organism.len() +
        record.taxonomy.len() +
        record.sequence.len()
}

/// Estimate the size of a FASTA record list.
#[inline]
fn estimate_list_size(list: &RecordList) -> usize {
    list.iter().fold(0, |sum, x| sum + estimate_record_size(x))
}

// WRITER

/// Export the SwissProt header to FASTA.
pub fn write_swissprot_header<T: Write>(record: &Record, writer: &mut T)
    -> Result<()>
{
    write_alls!(
        writer,
        b">sp|",     record.id.as_bytes(),
        b"|",        record.mnemonic.as_bytes(),
        b" ",        record.name.as_bytes(),
        b" OS=",     record.organism.as_bytes()
    )?;

    // Write the taxonomy ID, if not empty.
    if !record.taxonomy.is_empty() {
        write_alls!(writer, b" OX=", record.taxonomy.as_bytes())?;
    }

    // Write the taxonomy ID, if not empty.
    if !record.gene.is_empty() {
        write_alls!(writer, b" GN=", record.gene.as_bytes())?;
    }

    write_alls!(
        writer,
        b" PE=", to_bytes(&record.protein_evidence)?.as_slice(),
        b" SV=", to_bytes(&record.sequence_version)?.as_slice()
    )?;
    Ok(())
}

/// Export the TrEMBL header to FASTA.
///
/// Don't deduplicate this with SwissProt, they're very different
/// formats and we need to differentiate the two.
pub fn write_trembl_header<T: Write>(record: &Record, writer: &mut T)
    -> Result<()>
{
    write_alls!(
        writer,
        b">tr|",     record.id.as_bytes(),
        b"|",        record.mnemonic.as_bytes(),
        b" ",        record.name.as_bytes(),
        b" OS=",     record.organism.as_bytes()
    )?;

    // Write the taxonomy ID, if not empty.
    if !record.taxonomy.is_empty() {
        write_alls!(writer, b" OX=", record.taxonomy.as_bytes())?;
    }

    // Write the taxonomy ID, if not empty.
    if !record.gene.is_empty() {
        write_alls!(writer, b" GN=", record.gene.as_bytes())?;
    }

    write_alls!(
        writer,
        b" PE=", to_bytes(&record.protein_evidence)?.as_slice(),
        b" SV=", to_bytes(&record.sequence_version)?.as_slice()
    )?;
    Ok(())
}

#[inline(always)]
fn to_fasta<'a, T: Write>(writer: &mut T, record: &'a Record) -> Result<()> {
    record_to_fasta(writer, record)
}

/// Export record to FASTA.
pub fn record_to_fasta<T: Write>(writer: &mut T, record: &Record)
    -> Result<()>
{
    // Write header
    if record.reviewed {
        write_swissprot_header(record, writer)?;
    } else {
        write_trembl_header(record, writer)?;
    }

    // Write SwissProt sequence, formatted at 60 characters.
    // Write the initial, 60 character lines
    const SEQUENCE_LINE_LENGTH: usize = 60;
    let mut bytes = record.sequence.as_slice();
    while bytes.len() > SEQUENCE_LINE_LENGTH {
        let prefix = &bytes[0..SEQUENCE_LINE_LENGTH];
        bytes = &bytes[SEQUENCE_LINE_LENGTH..];
        writer.write_all(b"\n")?;
        writer.write_all(prefix)?;
    }

    // Write the remaining sequence line, if any remainder exists.
    if !bytes.is_empty() {
        writer.write_all(b"\n")?;
        writer.write_all(bytes)?;
    }

    Ok(())
}

// WRITER -- DEFAULT

#[inline(always)]
fn init_cb<T: Write>(writer: &mut T, delimiter: u8)
    -> Result<TextWriterState<T>>
{
    Ok(TextWriterState::new(writer, delimiter))
}

#[inline(always)]
fn export_cb<'a, T: Write>(writer: &mut TextWriterState<T>, record: &'a Record)
    -> Result<()>
{
    writer.export(record, &to_fasta)
}

#[inline(always)]
fn dest_cb<T: Write>(_: &mut TextWriterState<T>)
    -> Result<()>
{
    Ok(())
}

/// Default exporter from a non-owning iterator to FASTA.
#[inline(always)]
pub fn reference_iterator_to_fasta<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}


/// Default exporter from an owning iterator to FASTA.
#[inline(always)]
pub fn value_iterator_to_fasta<Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    value_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to FASTA.
#[inline(always)]
pub fn reference_iterator_to_fasta_strict<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Strict exporter from an owning iterator to FASTA.
#[inline(always)]
pub fn value_iterator_to_fasta_strict<Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    value_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to FASTA.
#[inline(always)]
pub fn reference_iterator_to_fasta_lenient<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Lenient exporter from an owning iterator to FASTA.
#[inline(always)]
pub fn value_iterator_to_fasta_lenient<Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    value_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// READER

/// Import record from SwissProt FASTA.
fn record_header_from_swissprot(header: &str) -> Result<Record> {
    type R = SwissProtHeaderRegex;

    // process the header and match it to the FASTA record
    let captures = none_to_error!(R::extract().captures(&header), InvalidInput);

    // initialize the record with header data
    let pe = capture_as_str(&captures, R::PE_INDEX);
    let sv = capture_as_str(&captures, R::SV_INDEX);
    Ok(Record {
        // Can use unwrap because they were matched in the regex
        // as "\d+" capture groups, they must be deserializeable to int.
        sequence_version: from_string(sv).unwrap(),
        protein_evidence: from_string(pe)?,
        mass: 0,
        length: 0,
        gene: optional_capture_as_string(&captures, R::GENE_INDEX),
        id: capture_as_string(&captures, R::ACCESSION_INDEX),
        mnemonic: capture_as_string(&captures, R::MNEMONIC_INDEX),
        name: capture_as_string(&captures, R::NAME_INDEX),
        organism: capture_as_string(&captures, R::ORGANISM_INDEX),
        taxonomy: optional_capture_as_string(&captures, R::TAXONOMY_INDEX),
        reviewed: true,

        // unused fields in header
        proteome: String::new(),
        sequence: vec![],
    })
}

/// Import record from TrEMBL FASTA.
fn record_header_from_trembl(header: &str) -> Result<Record> {
    type R = TrEMBLHeaderRegex;

    // process the header and match it to the FASTA record
    let captures = none_to_error!(R::extract().captures(&header), InvalidInput);

    // initialize the record with header data
    let pe = capture_as_str(&captures, R::PE_INDEX);
    let sv = capture_as_str(&captures, R::SV_INDEX);
    Ok(Record {
        // Can use unwrap because they were matched in the regex
        // as "\d+" capture groups, they must be deserializeable to int.
        sequence_version: from_string(sv).unwrap(),
        protein_evidence: from_string(pe)?,
        mass: 0,
        length: 0,
        gene: optional_capture_as_string(&captures, R::GENE_INDEX),
        id: capture_as_string(&captures, R::ACCESSION_INDEX),
        mnemonic: capture_as_string(&captures, R::MNEMONIC_INDEX),
        name: capture_as_string(&captures, R::NAME_INDEX),
        organism: capture_as_string(&captures, R::ORGANISM_INDEX),
        taxonomy: optional_capture_as_string(&captures, R::TAXONOMY_INDEX),
        reviewed: false,

        // unused fields in header
        proteome: String::new(),
        sequence: vec![],
    })
}

/// Import record from FASTA.
pub fn record_from_fasta<T: BufRead>(reader: &mut T)
    -> Result<Record>
{
    // Split along lines.
    // First line is the header, rest are the sequences.
    // Short-circuit if the header is `None`.
    let mut lines = reader.lines();
    let header = none_to_error!(lines.next(), InvalidInput)?;

    // Ensure we don't raise an out-of-bounds error on the subsequent slice.
    bool_to_error!(header.len() >= 3, InvalidInput);

    let mut record = match &header[..3] {
        ">sp"   => record_header_from_swissprot(&header)?,
        ">tr"   => record_header_from_trembl(&header)?,
        _       => return Err(From::from(ErrorKind::InvalidFastaFormat)),
    };

    // add sequence data to the FASTA sequence
    for line in lines {
        record.sequence.append(&mut line?.into_bytes());
    }

    // calculate the protein length and mass
    if record.sequence.len() > 0 {
        record.length = record.sequence.len() as u32;
        let mass = AverageMass::total_sequence_mass(record.sequence.as_slice());
        record.mass = mass.round() as u64;
    }

    Ok(record)
}

// READER -- DEFAULT

/// Iterator to lazily load `Record`s from a document.
///
/// Wraps `FastaIter` and converts the text to records.
pub struct FastaRecordIter<T: BufRead> {
    iter: FastaIter<T>
}

impl<T: BufRead> FastaRecordIter<T> {
    /// Create new FastaRecordIter from a buffered reader.
    #[inline]
    pub fn new(reader: T) -> Self {
        FastaRecordIter {
            iter: FastaIter::new(reader)
        }
    }
}

impl<T: BufRead> Iterator for FastaRecordIter<T> {
    type Item = Result<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = match self.iter.next()? {
            Err(e)   => return Some(Err(e)),
            Ok(bytes) => bytes,
        };

        Some(Record::from_fasta_bytes(bytes.as_slice()))
    }
}

/// Create default record iterator from reader.
#[inline(always)]
pub fn iterator_from_fasta<T: BufRead>(reader: T) -> FastaRecordIter<T> {
    FastaRecordIter::new(reader)
}

// READER -- STRICT

/// Iterator to lazily load `Record`s from a document.
///
/// Wraps `FastaIter` and converts the text to records strictly.
pub type FastaRecordStrictIter<T> = StrictIter<Record, FastaRecordIter<T>>;

/// Create default record iterator from reader.
#[inline(always)]
pub fn iterator_from_fasta_strict<T: BufRead>(reader: T) -> FastaRecordStrictIter<T> {
    FastaRecordStrictIter::new(iterator_from_fasta(reader))
}

// READER -- LENIENT

/// Iterator to lazily load `Record`s from a document.
///
/// Wraps `FastaIter` and converts the text to records leniently.
pub type FastaRecordLenientIter<T> = LenientIter<Record, FastaRecordIter<T>>;

/// Create lenient record iterator from reader.
#[inline(always)]
pub fn iterator_from_fasta_lenient<T: BufRead>(reader: T) -> FastaRecordLenientIter<T> {
    FastaRecordLenientIter::new(iterator_from_fasta(reader))
}

// TRAITS

impl Fasta for Record {
    #[inline]
    fn estimate_fasta_size(&self) -> usize {
        estimate_record_size(self)
    }

    #[inline(always)]
    fn to_fasta<T: Write>(&self, writer: &mut T) -> Result<()> {
        record_to_fasta(writer, self)
    }

    fn from_fasta<T: BufRead>(reader: &mut T) -> Result<Self> {
        record_from_fasta(reader)
    }
}

impl Fasta for RecordList {
    #[inline]
    fn estimate_fasta_size(&self) -> usize {
        estimate_list_size(self)
    }

    #[inline(always)]
    fn to_fasta<T: Write>(&self, writer: &mut T) -> Result<()> {
        reference_iterator_to_fasta(writer, self.iter())
    }

    #[inline(always)]
    fn from_fasta<T: BufRead>(reader: &mut T) -> Result<RecordList> {
        iterator_from_fasta(reader).collect()
    }
}

impl FastaCollection for RecordList {
    #[inline(always)]
    fn to_fasta_strict<T: Write>(&self, writer: &mut T) -> Result<()> {
        reference_iterator_to_fasta_strict(writer, self.iter())
    }

    #[inline(always)]
    fn to_fasta_lenient<T: Write>(&self, writer: &mut T) -> Result<()> {
        reference_iterator_to_fasta_lenient(writer, self.iter())
    }

    #[inline(always)]
    fn from_fasta_strict<T: BufRead>(reader: &mut T) -> Result<RecordList> {
        iterator_from_fasta_strict(reader).collect()
    }

    #[inline(always)]
    fn from_fasta_lenient<T: BufRead>(reader: &mut T) -> Result<RecordList> {
        Ok(iterator_from_fasta_lenient(reader).filter_map(Result::ok).collect())
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use bencher;
    use std::fs::File;
    use std::io::{BufReader, Cursor};
    use std::path::PathBuf;
    use test::testdata_dir;
    use super::*;
    use super::super::test::*;

    #[test]
    fn fasta_iter_test() {
        // Check iterator over data.
        let s = b">tr\nXX\n>sp\nXX\nXX\n>tr\n".to_vec();
        let i = FastaIter::new(Cursor::new(s));
        let r: Result<Vec<Bytes>> = i.collect();
        assert_eq!(r.unwrap(), &[b">tr\nXX\n".to_vec(), b">sp\nXX\nXX\n".to_vec(), b">tr\n".to_vec()]);

        // Check iterator over empty string.
        let s = b"".to_vec();
        let i = FastaIter::new(Cursor::new(s));
        let r: Result<Vec<Bytes>> = i.collect();
        assert_eq!(r.unwrap(), Vec::<Bytes>::new());
    }

    #[test]
    fn estimate_size_test() {
        let g = gapdh();
        let b = bsa();
        let v = vec![gapdh(), bsa()];
        assert_eq!(estimate_record_size(&g), 458);
        assert_eq!(estimate_record_size(&b), 693);
        assert_eq!(estimate_list_size(&v), 1151);
    }

    #[test]
    fn iterator_to_fasta_test() {
        let v = vec![gapdh(), bsa()];
        let u = vec![gapdh(), bsa(), Record::new()];

        // reference -- default
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_fasta(&mut w, v.iter()).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_FASTA);

        // value -- default
        let mut w = Cursor::new(vec![]);
        value_iterator_to_fasta(&mut w, iterator_by_value!(v.iter())).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_FASTA);

        // reference -- strict
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_fasta_strict(&mut w, v.iter()).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_FASTA);

        let mut w = Cursor::new(vec![]);
        let r = reference_iterator_to_fasta_strict(&mut w, u.iter());
        assert!(r.is_err());

        // value -- strict
        let mut w = Cursor::new(vec![]);
        value_iterator_to_fasta_strict(&mut w, iterator_by_value!(v.iter())).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_FASTA);

        let mut w = Cursor::new(vec![]);
        let r = value_iterator_to_fasta_strict(&mut w, iterator_by_value!(u.iter()));
        assert!(r.is_err());

        // reference -- lenient
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_fasta_lenient(&mut w, v.iter()).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_FASTA);

        let mut w = Cursor::new(vec![]);
        reference_iterator_to_fasta_lenient(&mut w, u.iter()).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_FASTA);

        // value -- lenient
        let mut w = Cursor::new(vec![]);
        value_iterator_to_fasta_lenient(&mut w, iterator_by_value!(v.iter())).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_FASTA);

        let mut w = Cursor::new(vec![]);
        value_iterator_to_fasta_lenient(&mut w, iterator_by_value!(u.iter())).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_FASTA);
    }

    #[test]
    fn iterator_from_fasta_test() {
        // VALID
        let text = GAPDH_BSA_FASTA;
        let expected = vec![gapdh(), bsa()];

        // record iterator -- default
        let iter = FastaRecordIter::new(Cursor::new(text));
        let v: Result<RecordList> = iter.collect();
        incomplete_list_eq(&expected, &v.unwrap());

        // Compile check only
        iterator_from_fasta(&mut Cursor::new(text));

        // record iterator -- strict
        let iter = iterator_from_fasta_strict(Cursor::new(text));
        let v: Result<RecordList> = iter.collect();
        incomplete_list_eq(&expected, &v.unwrap());

        // Compile check only
        iterator_from_fasta_strict(&mut Cursor::new(text));

        // record iterator -- lenient
        let iter = iterator_from_fasta_lenient(Cursor::new(text));
        let v: Result<RecordList> = iter.collect();
        incomplete_list_eq(&expected, &v.unwrap());

        // Compile check only
        iterator_from_fasta_lenient(&mut Cursor::new(text));

        // INVALID
        let text = GAPDH_EMPTY_FASTA;
        let expected1 = vec![gapdh(), Record::new()];
        let expected2 = vec![gapdh()];

        // record iterator -- default
        let iter = iterator_from_fasta(Cursor::new(text));
        let v: Result<RecordList> = iter.collect();
        let v = v.unwrap();
        assert_eq!(expected1.len(), v.len());
        incomplete_eq(&expected1[0], &v[0]);
        assert_eq!(expected1[1], v[1]);

        // record iterator -- strict
        let iter = iterator_from_fasta_strict(Cursor::new(text));
        let v: Result<RecordList> = iter.collect();
        assert!(v.is_err());

        // record iterator -- lenient
        let iter = iterator_from_fasta_lenient(Cursor::new(text));
        let v: Result<RecordList> = iter.collect();
        incomplete_list_eq(&expected2, &v.unwrap());
    }

    fn fasta_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("uniprot/fasta");
        dir
    }

    #[test]
    #[ignore]
    fn human_fasta_test() {
        let mut path = fasta_dir();
        path.push("human.fasta");
        let reader = BufReader::new(File::open(path).unwrap());
        let iter = FastaRecordIter::new(reader);

        // do nothing, just check it parses.
        for item in iter {
            bencher::black_box(item).unwrap();
        }
    }
}
