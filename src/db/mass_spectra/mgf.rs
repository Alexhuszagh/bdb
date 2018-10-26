//! Helper utilities for MGF loading and saving.

use std::io::prelude::*;

// TODO(ahuszagh)
//  More imports...
use traits::*;
use util::*;
use super::msconvert_mgf::*;
use super::record::Record;
use super::record_list::RecordList;

// MGF ITERATOR

/// Iterator to parse individual MGF entries from a document.
///
/// Convert a stream to a lazy reader that fetches individual MGF entries
/// from the document.
#[allow(dead_code)]     // TODO(ahuszagh) Remove
pub struct MgfIter<T: BufRead> {
    reader: T,
    start: &'static str,
    buf: BufferType,
    line: String,
}

impl<T: BufRead> MgfIter<T> {
    /// Create new MgfIter from a buffered reader.
    #[inline]
    #[allow(dead_code)]     // TODO(ahuszagh) Remove
    pub fn new(reader: T, start: &'static str) -> Self {
        MgfIter {
            reader: reader,
            start: start,
            buf: Vec::with_capacity(8000),
            line: String::with_capacity(8000)
        }
    }
}

impl<T: BufRead> Iterator for MgfIter<T> {
    type Item = ResultType<String>;

    fn next(&mut self) -> Option<Self::Item> {
        text_next(self.start, &mut self.reader, &mut self.buf, &mut self.line)
    }
}

// SIZE

/// Estimate the size of an MGF record.
#[inline(always)]
fn estimate_record_size(record: &Record, kind: MgfKind) -> usize {
    match kind {
        MgfKind::MsConvert => estimate_msconvert_mgf_record_size(record),
        // TODO(ahuszagh)
        //  Add more record types.
    }
}

/// Estimate the size of an MGF record list.
#[inline]
fn estimate_list_size(list: &RecordList, kind: MgfKind) -> usize {
    list.iter().fold(0, |sum, x| sum + estimate_record_size(x, kind))
}

// WRITER

/// Export record to MGF.
#[inline(always)]
pub fn record_to_mgf<T: Write>(writer: &mut T, record: &Record, kind: MgfKind)
    -> ResultType<()>
{
    match kind {
        MgfKind::MsConvert => record_to_msconvert_mgf(writer, record),
    }
}

// WRITER -- DEFAULT

/// Default exporter from a non-owning iterator to MGF.
#[inline(always)]
pub fn reference_iterator_to_mgf<'a, Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    match kind {
        MgfKind::MsConvert => reference_iterator_to_msconvert_mgf(writer, iter),
    }
}

/// Default exporter from an owning iterator to MGF.
#[inline(always)]
#[allow(dead_code)]
pub fn value_iterator_to_mgf<Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    match kind {
        MgfKind::MsConvert => value_iterator_to_msconvert_mgf(writer, iter),
    }
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to MGF.
#[inline(always)]
#[allow(dead_code)]
pub fn reference_iterator_to_mgf_strict<'a, Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    match kind {
        MgfKind::MsConvert => reference_iterator_to_msconvert_mgf_strict(writer, iter),
    }
}

/// Strict exporter from an owning iterator to MGF.
#[inline(always)]
#[allow(dead_code)]
pub fn value_iterator_to_mgf_strict<Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    match kind {
        MgfKind::MsConvert => value_iterator_to_msconvert_mgf_strict(writer, iter),
    }
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to MGF.
#[inline(always)]
#[allow(dead_code)]
pub fn reference_iterator_to_mgf_lenient<'a, Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    match kind {
        MgfKind::MsConvert => reference_iterator_to_msconvert_mgf_lenient(writer, iter),
    }
}

/// Lenient exporter from an owning iterator to MGF.
#[inline(always)]
#[allow(dead_code)]
pub fn value_iterator_to_mgf_lenient<Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    match kind {
        MgfKind::MsConvert => value_iterator_to_msconvert_mgf_lenient(writer, iter),
    }
}

// READER

/// Import record from MGF.
pub fn record_from_mgf<T: BufRead>(reader: &mut T, kind: MgfKind)
    -> ResultType<Record>
{
    match kind {
        MgfKind::MsConvert => record_from_msconvert_mgf(reader)
    }
}

// READER -- DEFAULT

/// Iterator to lazily load `Record`s from a document.
///
/// Wraps `MgfIter` and converts the text to records.
pub struct MgfRecordIter<T: BufRead> {
    iter: MgfIter<T>,
    kind: MgfKind
}

impl<T: BufRead> MgfRecordIter<T> {
    /// Create new MgfRecordIter from a buffered reader.
    #[inline]
    pub fn new(reader: T, start: &'static str, kind: MgfKind) -> Self {
        MgfRecordIter {
            iter: MgfIter::new(reader, start),
            kind: kind
        }
    }
}

impl<T: BufRead> Iterator for MgfRecordIter<T> {
    type Item = ResultType<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = match self.iter.next()? {
            Err(e)   => return Some(Err(e)),
            Ok(text) => text,

        };

        Some(Record::from_mgf_string(&text, self.kind))
    }
}

/// Create default record iterator from reader.
#[inline(always)]
pub fn iterator_from_mgf<T: BufRead>(reader: T, kind: MgfKind)
    -> MgfRecordIter<T>
{
    match kind {
        MgfKind::MsConvert => iterator_from_msconvert_mgf(reader),
    }
}

// READER -- STRICT

/// Iterator to lazily load `Record`s from a document.
///
/// Wraps `FastaIter` and converts the text to records strictly.
pub type MgfRecordStrictIter<T> = StrictIter<Record, MgfRecordIter<T>>;

/// Create default record iterator from reader.
#[inline(always)]
#[allow(dead_code)]         // TODO(ahuszagh)       Remove
pub fn iterator_from_fasta_strict<T: BufRead>(reader: T, kind: MgfKind)
    -> MgfRecordStrictIter<T>
{
    MgfRecordStrictIter::new(iterator_from_mgf(reader, kind))
}

// READER -- LENIENT

/// Iterator to lazily load `Record`s from a document.
///
/// Wraps `FastaIter` and converts the text to records leniently.
pub type MgfRecordLenientIter<T> = LenientIter<Record, MgfRecordIter<T>>;

/// Create default record iterator from reader.
#[inline(always)]
#[allow(dead_code)]         // TODO(ahuszagh)       Remove
pub fn iterator_from_fasta_lenient<T: BufRead>(reader: T, kind: MgfKind)
    -> MgfRecordLenientIter<T>
{
    MgfRecordLenientIter::new(iterator_from_mgf(reader, kind))
}

// TRAITS

impl Mgf for Record {
    #[inline]
    fn estimate_mgf_size(&self, kind: MgfKind) -> usize {
        estimate_record_size(self, kind)
    }

    #[inline(always)]
    fn to_mgf<T: Write>(&self, writer: &mut T, kind: MgfKind) -> ResultType<()> {
        record_to_mgf(writer, self, kind)
    }

    #[inline(always)]
    fn from_mgf<T: BufRead>(reader: &mut T, kind: MgfKind) -> ResultType<Self> {
        record_from_mgf(reader, kind)
    }
}

impl Mgf for RecordList {
    #[inline]
    fn estimate_mgf_size(&self, kind: MgfKind) -> usize {
        estimate_list_size(self, kind)
    }

    #[inline(always)]
    #[allow(unused_variables)]
    fn to_mgf<T: Write>(&self, writer: &mut T, kind: MgfKind) -> ResultType<()> {
        reference_iterator_to_mgf(writer, self.iter(), kind)
    }

    #[inline(always)]
    #[allow(unused_variables)]
    fn from_mgf<T: BufRead>(reader: &mut T, kind: MgfKind) -> ResultType<Self> {
        iterator_from_mgf(reader, kind).collect()
    }
}

impl MgfCollection for RecordList {
    // TODO(ahuszagh)   Implement...
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    //TODO(ahuszagh)        Implement
}
