//! Helper utilities for FASTQ loading and saving.

use std::io::prelude::*;

use traits::*;
use util::*;
use super::re::*;
use super::record::Record;
use super::record_list::RecordList;

// FASTQ ITERATOR

/// Iterator to parse individual FASTQ entries from a document.
///
/// Convert a stream to a lazy reader that fetches individual FASTQ entries
/// from the document.
pub struct FastqIter<T: BufRead> {
    reader: T,
    buf: BufferType,
    line: String,
}

impl<T: BufRead> FastqIter<T> {
    /// Create new FastqIter from a buffered reader.
    #[inline]
    pub fn new(reader: T) -> Self {
        FastqIter {
            reader: reader,
            buf: Vec::with_capacity(8000),
            line: String::with_capacity(8000)
        }
    }
}

impl<T: BufRead> Iterator for FastqIter<T> {
    type Item = ResultType<String>;

    fn next(&mut self) -> Option<Self::Item> {
        text_next("@", &mut self.reader, &mut self.buf, &mut self.line)
    }
}

// SIZE

/// Estimate the size of a FASTA record.
///
/// Used to prevent reallocations during record exportation to string,
/// to minimize costly library calls.
#[inline]
fn estimate_record_size(record: &Record) -> usize {
    const FASTQ_VOCABULARY_SIZE: usize = 5;
    FASTQ_VOCABULARY_SIZE +
        record.seq_id.len() +
        record.description.len() +
        record.sequence.len() +
        record.quality.len()
}

/// Estimate the size of a FASTA record list.
#[inline]
fn estimate_list_size(list: &RecordList) -> usize {
    list.iter().fold(0, |sum, x| sum + estimate_record_size(x))
}

// WRITER

#[inline(always)]
fn to_fastq<T: Write>(writer: &mut T, record: &Record) -> ResultType<()> {
    record_to_fastq(writer, record)
}

/// Export record to FASTQ.
pub fn record_to_fastq<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    write_alls!(writer, b"@", record.seq_id.as_bytes())?;

    if !record.description.is_empty() {
        write_alls!(writer, b" ", record.description.as_bytes())?;
    }

    write_alls!(
        writer,
        b"\n", record.sequence.as_slice(),
        b"\n+", record.seq_id.as_bytes()
    )?;

    if !record.description.is_empty() {
        write_alls!(writer, b" ", record.description.as_bytes())?;
    }

    write_alls!(writer, record.quality.as_slice())?;

    Ok(())
}

// WRITER -- DEFAULT

#[inline(always)]
fn init_cb<T: Write>(writer: &mut T, delimiter: u8)
    -> ResultType<TextWriterState<T>>
{
    Ok(TextWriterState::new(writer, delimiter))
}

#[inline(always)]
fn export_cb<'a, T: Write>(writer: &mut TextWriterState<T>, record: &'a Record)
    -> ResultType<()>
{
    writer.export(record, &to_fastq)
}

#[inline(always)]
fn dest_cb<T: Write>(_: &mut TextWriterState<T>)
    -> ResultType<()>
{
    Ok(())
}

/// Default exporter from a non-owning iterator to FASTQ.
#[inline(always)]
pub fn reference_iterator_to_fastq<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}


/// Default exporter from an owning iterator to FASTQ.
#[inline(always)]
pub fn value_iterator_to_fastq<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to FASTQ.
#[inline(always)]
pub fn reference_iterator_to_fastq_strict<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Strict exporter from an owning iterator to FASTQ.
#[inline(always)]
pub fn value_iterator_to_fastq_strict<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to FASTQ.
#[inline(always)]
pub fn reference_iterator_to_fastq_lenient<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Lenient exporter from an owning iterator to FASTQ.
#[inline(always)]
pub fn value_iterator_to_fastq_lenient<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// READER

/// Import record from FASTQ.
#[allow(unused_variables)]
pub fn record_from_fastq<T: BufRead>(reader: &mut T)
    -> ResultType<Record>
{
    // Split along lines.
    // The first line is the first header, short-circuit if it's none.
    let mut lines = reader.lines();
    let header = none_to_error!(lines.next(), InvalidInput)?;

    // process the header and match it to the FASTA record
    let captures = none_to_error!(FastqHeaderRegex::extract().captures(&header), InvalidInput);

    // create the record from the header metadata
    let mut record = Record {
        seq_id: capture_as_string(&captures, FastqHeaderRegex::SEQID_INDEX),
        description: capture_as_string(&captures, FastqHeaderRegex::DESCRIPTION_INDEX),
        length: 0,
        sequence: vec![],
        quality: vec![]
    };

    // get the FASTQ sequence.
    let sequence = none_to_error!(lines.next(), InvalidInput)?;
    record.sequence = sequence.into_bytes();
    record.length = record.sequence.len() as u32;

    // get the header quality line
    let header = none_to_error!(lines.next(), InvalidInput)?;
    bool_to_error!(header.starts_with('+'), InvalidInput);

    // get the FASTQ quality scores
    let quality = none_to_error!(lines.next(), InvalidInput)?;
    record.quality = quality.into_bytes();
    bool_to_error!(record.quality.len() as u32 == record.length, InvalidRecord);

    Ok(record)
}

// READER -- DEFAULT

/// Iterator to lazily load `Record`s from a document.
///
/// Wraps `FastqIter` and converts the text to records.
pub struct FastqRecordIter<T: BufRead> {
    iter: FastqIter<T>
}

impl<T: BufRead> FastqRecordIter<T> {
    /// Create new FastqRecordIter from a buffered reader.
    #[inline]
    pub fn new(reader: T) -> Self {
        FastqRecordIter {
            iter: FastqIter::new(reader)
        }
    }
}

impl<T: BufRead> Iterator for FastqRecordIter<T> {
    type Item = ResultType<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = match self.iter.next()? {
            Err(e)   => return Some(Err(e)),
            Ok(text) => text,

        };

        Some(Record::from_fastq_string(&text))
    }
}

/// Create default record iterator from reader.
#[inline(always)]
pub fn iterator_from_fastq<T: BufRead>(reader: T) -> FastqRecordIter<T> {
    FastqRecordIter::new(reader)
}

// READER -- STRICT

/// Iterator to lazily load `Record`s from a document.
///
/// Wraps `FastqIter` and converts the text to records strictly.
pub type FastqRecordStrictIter<T> = StrictIter<Record, FastqRecordIter<T>>;

/// Create default record iterator from reader.
#[inline(always)]
pub fn iterator_from_fastq_strict<T: BufRead>(reader: T) -> FastqRecordStrictIter<T> {
    FastqRecordStrictIter::new(iterator_from_fastq(reader))
}

// READER -- LENIENT

/// Iterator to lazily load `Record`s from a document.
///
/// Wraps `FastqIter` and converts the text to records leniently.
pub type FastqRecordLenientIter<T> = LenientIter<Record, FastqRecordIter<T>>;

/// Create lenient record iterator from reader.
#[inline(always)]
pub fn iterator_from_fastq_lenient<T: BufRead>(reader: T) -> FastqRecordLenientIter<T> {
    FastqRecordLenientIter::new(iterator_from_fastq(reader))
}

// TRAITS

impl Fastq for Record {
    #[inline]
    fn estimate_fastq_size(&self) -> usize {
        estimate_record_size(self)
    }

    #[inline(always)]
    fn to_fastq<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        record_to_fastq(writer, self)
    }

    fn from_fastq<T: BufRead>(reader: &mut T) -> ResultType<Self> {
        record_from_fastq(reader)
    }
}

impl Fastq for RecordList {
    #[inline]
    fn estimate_fastq_size(&self) -> usize {
        estimate_list_size(self)
    }

    #[inline(always)]
    fn to_fastq<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        reference_iterator_to_fastq(writer, self.iter())
    }

    #[inline(always)]
    fn from_fastq<T: BufRead>(reader: &mut T) -> ResultType<RecordList> {
        iterator_from_fastq(reader).collect()
    }
}

impl FastqCollection for RecordList {
    #[inline(always)]
    fn to_fastq_strict<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        reference_iterator_to_fastq_strict(writer, self.iter())
    }

    #[inline(always)]
    fn to_fastq_lenient<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        reference_iterator_to_fastq_lenient(writer, self.iter())
    }

    #[inline(always)]
    fn from_fastq_strict<T: BufRead>(reader: &mut T) -> ResultType<RecordList> {
        iterator_from_fastq_strict(reader).collect()
    }

    #[inline(always)]
    fn from_fastq_lenient<T: BufRead>(reader: &mut T) -> ResultType<RecordList> {
        Ok(iterator_from_fastq_lenient(reader).filter_map(Result::ok).collect())
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use std::io::{Cursor};
    use super::*;
    //use super::super::test::*;

    #[test]
    fn fastq_iter_test() {
        // Check iterator over data.

        let s = "@tag desc\nCATTAG\n+tag desc\n;;;;;;\n@tag1 desc1\nTAGCAT\n+tag1 desc1\n;;;;;;";
        let i = FastqIter::new(Cursor::new(s));
        let r: ResultType<Vec<String>> = i.collect();
        assert_eq!(r.unwrap(), &["@tag desc\nCATTAG\n+tag desc\n;;;;;;\n", "@tag1 desc1\nTAGCAT\n+tag1 desc1\n;;;;;;"]);

        // Check iterator over empty string.
        let s = "";
        let i = FastqIter::new(Cursor::new(s));
        let r: ResultType<Vec<String>> = i.collect();
        assert_eq!(r.unwrap(), Vec::<String>::new());
    }

    // TODO(ahuszagh)
    //  Implement the unittests.
}
