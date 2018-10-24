//! Helper utilities for FASTQ loading and saving.

use std::io::prelude::*;
use std::str as stdstr;

use traits::*;
use util::*;
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

    /// Export the buffer to a string without affecting the buffer.
    #[inline]
    fn to_string_impl(&self) -> Option<ResultType<String>> {
        match self.buf.len() {
            0   => None,
            _   => Some(match stdstr::from_utf8(&self.buf) {
                Err(e)  => Err(From::from(e)),
                Ok(v)   => Ok(String::from(v)),
            }),
        }
    }

    /// Export the buffer to a string (or none if the buffer is empty.)
    #[inline]
    fn to_string(&mut self) -> Option<ResultType<String>> {
        let result = self.to_string_impl();
        unsafe { self.buf.set_len(0); }
        result
    }
}

impl<T: BufRead> Iterator for FastqIter<T> {
    type Item = ResultType<String>;

    fn next(&mut self) -> Option<Self::Item> {
        // Indefinitely loop over lines.
        loop {
            match self.reader.read_line(&mut self.line) {
                Err(e)      => return Some(Err(From::from(e))),
                Ok(size)    => match size {
                    // Reached EOF
                    0   => return self.to_string(),
                    // Read bytes, process them.
                    _   => unsafe {
                        // Ignore whitespace.
                        if self.line == "\n" || self.line == "\r\n" {
                            self.line.as_mut_vec().set_len(0);
                            continue;
                        } else if self.buf.len() > 0 && self.line.starts_with("@") {
                            // Create result from existing buffer,
                            // clear the existing buffer, and add
                            // the current line to a new buffer.
                            let result = self.to_string();
                            self.buf.append(self.line.as_mut_vec());
                            return result;
                        } else {
                            // Move the line to the buffer.
                            self.buf.append(self.line.as_mut_vec());
                        }
                    },
                }
            }
        }
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
    record.to_fastq(writer)
}

/// Export record to FASTQ.
#[allow(unused_variables)]
pub fn record_to_fastq<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    Err(From::from(""))
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

// TODO(ahuszagh)
//  Implement the reader....

/// Import record from FASTQ.
pub fn record_from_fastq<T: BufRead>(reader: &mut T)
    -> ResultType<Record>
{
    Err(From::from(""))
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
    FastqRecordStrictIter::new(FastqRecordIter::new(reader))
}

// READER -- LENIENT

/// Iterator to lazily load `Record`s from a document.
///
/// Wraps `FastqIter` and converts the text to records leniently.
pub type FastqRecordLenientIter<T> = LenientIter<Record, FastqRecordIter<T>>;

/// Create lenient record iterator from reader.
#[inline(always)]
pub fn iterator_from_fastq_lenient<T: BufRead>(reader: T) -> FastqRecordLenientIter<T> {
    FastqRecordLenientIter::new(FastqRecordIter::new(reader))
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
