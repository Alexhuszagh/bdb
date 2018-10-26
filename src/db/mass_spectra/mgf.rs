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
#[allow(dead_code)]     // TODO(ahuszagh), Remove
pub struct MgfIter<T: BufRead> {
    reader: T,
    start: &'static str,
    buf: BufferType,
    line: String,
}

impl<T: BufRead> MgfIter<T> {
    /// Create new MgfIter from a buffered reader.
    #[inline]
    #[allow(dead_code)]     // TODO(ahuszagh), Remove
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
// WRITER -- STRICT
// WRITER -- LENIENT

// TODO(ahuszagh)
//  Implement these callbacks...
//#[inline(always)]
//fn init_cb<T: Write>(writer: &mut T, delimiter: u8)
//    -> ResultType<TextWriterState<T>>
//{
//    Ok(TextWriterState::new(writer, delimiter))
//}
//
//#[inline(always)]
//fn export_cb<'a, T: Write>(writer: &mut TextWriterState<T>, record: &'a Record)
//    -> ResultType<()>
//{
//    writer.export(record, &to_fasta)
//}
//
//#[inline(always)]
//fn dest_cb<T: Write>(_: &mut TextWriterState<T>)
//    -> ResultType<()>
//{
//    Ok(())
//}

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
// READER -- STRICT
// READER -- LENIENT

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
        // TODO(ahuszagh)   Implement...
        Ok(())
    }

    #[inline(always)]
    #[allow(unused_variables)]
    fn from_mgf<T: BufRead>(reader: &mut T, kind: MgfKind) -> ResultType<Self> {
        // TODO(ahuszagh)   Implement...
        Ok(RecordList::new())
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    //TODO(ahuszagh)        Implement
}
