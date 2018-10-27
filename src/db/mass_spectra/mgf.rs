//! Helper utilities for MGF loading and saving.

use std::io::prelude::*;

use traits::*;
use util::*;
use super::msconvert_mgf::*;
use super::pava_mgf::*;
use super::record::Record;
use super::record_list::RecordList;

// MGF ITERATOR

/// Iterator to parse individual MGF entries from a document.
///
/// Convert a stream to a lazy reader that fetches individual MGF entries
/// from the document.
pub struct MgfIter<T: BufRead> {
    reader: T,
    start: &'static str,
    buf: BufferType,
    line: String,
}

impl<T: BufRead> MgfIter<T> {
    /// Create new MgfIter from a buffered reader.
    #[inline]
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
        text_next_skip_whitespace(self.start, &mut self.reader, &mut self.buf, &mut self.line)
    }
}

// SIZE

/// Estimate the size of an MGF record.
#[inline(always)]
fn estimate_record_size(record: &Record, kind: MgfKind) -> usize {
    match kind {
        MgfKind::MsConvert => estimate_msconvert_mgf_record_size(record),
        MgfKind::Pava => estimate_pava_mgf_record_size(record),
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
        MgfKind::Pava => record_to_pava_mgf(writer, record),
        // TODO(ahuszagh)
        //  Add more record types.
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
        MgfKind::Pava => reference_iterator_to_pava_mgf(writer, iter),
        // TODO(ahuszagh)
        //  Add more record types.
    }
}

/// Default exporter from an owning iterator to MGF.
#[inline(always)]
pub fn value_iterator_to_mgf<Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    match kind {
        MgfKind::MsConvert => value_iterator_to_msconvert_mgf(writer, iter),
        MgfKind::Pava => value_iterator_to_pava_mgf(writer, iter),
        // TODO(ahuszagh)
        //  Add more record types.
    }
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to MGF.
#[inline(always)]
pub fn reference_iterator_to_mgf_strict<'a, Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    match kind {
        MgfKind::MsConvert => reference_iterator_to_msconvert_mgf_strict(writer, iter),
        MgfKind::Pava => reference_iterator_to_pava_mgf_strict(writer, iter),
        // TODO(ahuszagh)
        //  Add more record types.
    }
}

/// Strict exporter from an owning iterator to MGF.
#[inline(always)]
pub fn value_iterator_to_mgf_strict<Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    match kind {
        MgfKind::MsConvert => value_iterator_to_msconvert_mgf_strict(writer, iter),
        MgfKind::Pava => value_iterator_to_pava_mgf_strict(writer, iter),
        // TODO(ahuszagh)
        //  Add more record types.
    }
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to MGF.
#[inline(always)]
pub fn reference_iterator_to_mgf_lenient<'a, Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    match kind {
        MgfKind::MsConvert => reference_iterator_to_msconvert_mgf_lenient(writer, iter),
        MgfKind::Pava => reference_iterator_to_pava_mgf_lenient(writer, iter),
        // TODO(ahuszagh)
        //  Add more record types.
    }
}

/// Lenient exporter from an owning iterator to MGF.
#[inline(always)]
pub fn value_iterator_to_mgf_lenient<Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    match kind {
        MgfKind::MsConvert => value_iterator_to_msconvert_mgf_lenient(writer, iter),
        MgfKind::Pava => value_iterator_to_pava_mgf_lenient(writer, iter),
        // TODO(ahuszagh)
        //  Add more record types.
    }
}

// READER

/// Import record from MGF.
pub fn record_from_mgf<T: BufRead>(reader: &mut T, kind: MgfKind)
    -> ResultType<Record>
{
    match kind {
        MgfKind::MsConvert => record_from_msconvert_mgf(reader),
        MgfKind::Pava => record_from_pava_mgf(reader),
        // TODO(ahuszagh)
        //  Add more record types.
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
        MgfKind::Pava => iterator_from_pava_mgf(reader),
        // TODO(ahuszagh)
        //  Add more record types.
    }
}

// READER -- STRICT

/// Iterator to lazily load `Record`s from a document.
///
/// Wraps `FastaIter` and converts the text to records strictly.
pub type MgfRecordStrictIter<T> = StrictIter<Record, MgfRecordIter<T>>;

/// Create default record iterator from reader.
#[inline(always)]
pub fn iterator_from_mgf_strict<T: BufRead>(reader: T, kind: MgfKind)
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
pub fn iterator_from_mgf_lenient<T: BufRead>(reader: T, kind: MgfKind)
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
    fn to_mgf<T: Write>(&self, writer: &mut T, kind: MgfKind) -> ResultType<()> {
        reference_iterator_to_mgf(writer, self.iter(), kind)
    }

    #[inline(always)]
    fn from_mgf<T: BufRead>(reader: &mut T, kind: MgfKind) -> ResultType<Self> {
        iterator_from_mgf(reader, kind).collect()
    }
}

impl MgfCollection for RecordList {
    #[inline(always)]
    fn to_mgf_strict<T: Write>(&self, writer: &mut T, kind: MgfKind) -> ResultType<()> {
        reference_iterator_to_mgf_strict(writer, self.iter(), kind)
    }

    #[inline(always)]
    fn to_mgf_lenient<T: Write>(&self, writer: &mut T, kind: MgfKind) -> ResultType<()> {
        reference_iterator_to_mgf_lenient(writer, self.iter(), kind)
    }

    #[inline(always)]
    fn from_mgf_strict<T: BufRead>(reader: &mut T, kind: MgfKind) -> ResultType<RecordList> {
        iterator_from_mgf_strict(reader, kind).collect()
    }

    #[inline(always)]
    fn from_mgf_lenient<T: BufRead>(reader: &mut T, kind: MgfKind) -> ResultType<RecordList> {
        Ok(iterator_from_mgf_lenient(reader, kind).filter_map(Result::ok).collect())
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
    fn mgf_iter_test() {
        // Check iterator over data.
        let s = "BEGIN IONS\nTITLE=A\nEND IONS\nBEGIN IONS\nTITLE=B\nEND IONS\n";
        let i = MgfIter::new(Cursor::new(s), "BEGIN IONS");
        let r: ResultType<Vec<String>> = i.collect();
        assert_eq!(r.unwrap(), &["BEGIN IONS\nTITLE=A\nEND IONS\n", "BEGIN IONS\nTITLE=B\nEND IONS\n"]);

        // Check iterator over empty string.
        let s = "";
        let i = MgfIter::new(Cursor::new(s), "BEGIN IONS");
        let r: ResultType<Vec<String>> = i.collect();
        assert_eq!(r.unwrap(), Vec::<String>::new());
    }

    #[test]
    fn estimate_size_test() {
        // MSConvert
        let kind = MgfKind::MsConvert;
        let s = msconvert_33450();
        let e = msconvert_empty();
        let v = vec![msconvert_33450(), msconvert_empty()];
        assert_eq!(estimate_record_size(&s,kind), 1987);
        assert_eq!(estimate_record_size(&e,kind), 262);
        assert_eq!(estimate_list_size(&v,kind), 2249);
    }

    #[test]
    fn iterator_to_msconvert_mgf_test() {
        let v = vec![msconvert_33450()];
        let u = vec![msconvert_33450(), msconvert_empty()];
        let kind = MgfKind::MsConvert;

        // reference -- default
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_mgf(&mut w, v.iter(), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), MSCONVERT_33450_MGF);

        // value -- default
        let mut w = Cursor::new(vec![]);
        value_iterator_to_mgf(&mut w, iterator_by_value!(v.iter()), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), MSCONVERT_33450_MGF);

        // reference -- strict
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_mgf_strict(&mut w, v.iter(), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), MSCONVERT_33450_MGF);

        let mut w = Cursor::new(vec![]);
        let r = reference_iterator_to_mgf_strict(&mut w, u.iter(), kind);
        assert!(r.is_err());

        // value -- strict
        let mut w = Cursor::new(vec![]);
        value_iterator_to_mgf_strict(&mut w, iterator_by_value!(v.iter()), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), MSCONVERT_33450_MGF);

        let mut w = Cursor::new(vec![]);
        let r = value_iterator_to_mgf_strict(&mut w, iterator_by_value!(u.iter()), kind);
        assert!(r.is_err());

        // reference -- lenient
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_mgf_lenient(&mut w, v.iter(), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), MSCONVERT_33450_MGF);

        let mut w = Cursor::new(vec![]);
        reference_iterator_to_mgf_lenient(&mut w, u.iter(), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), MSCONVERT_33450_MGF);

        // value -- lenient
        let mut w = Cursor::new(vec![]);
        value_iterator_to_mgf_lenient(&mut w, iterator_by_value!(v.iter()), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), MSCONVERT_33450_MGF);

        let mut w = Cursor::new(vec![]);
        value_iterator_to_mgf_lenient(&mut w, iterator_by_value!(u.iter()), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), MSCONVERT_33450_MGF);
    }

    #[test]
    fn iterator_from_msconvert_mgf_test() {
        // VALID
        let text = MSCONVERT_33450_MGF;
        let expected = vec![msconvert_33450()];
        let kind = MgfKind::MsConvert;

        // record iterator -- default
        let iter = iterator_from_mgf(Cursor::new(text), kind);
        let v: ResultType<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());

        // record iterator -- strict
        let iter = iterator_from_mgf_strict(Cursor::new(text), kind);
        let v: ResultType<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());

        // record iterator -- lenient
        let iter = iterator_from_mgf_lenient(Cursor::new(text), kind);
        let v: ResultType<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());

        // INVALID
        let text = MSCONVERT_EMPTY_MGF;
        let expected = vec![msconvert_empty()];

        // record iterator -- default
        let iter = iterator_from_mgf(Cursor::new(text), kind);
        let v: ResultType<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());

        // record iterator -- strict
        let iter = iterator_from_mgf_strict(Cursor::new(text), kind);
        let v: ResultType<RecordList> = iter.collect();
        assert!(v.is_err());

        // record iterator -- lenient
        let iter = iterator_from_mgf_lenient(Cursor::new(text), kind);
        let v: ResultType<RecordList> = iter.collect();
        assert_eq!(v.unwrap().len(), 0);
    }

    fn mgf_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("mass_spectra/mgf");
        dir
    }

    #[test]
    #[ignore]
    fn msconvert_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_msconvert_ms2.txt");

        let reader = BufReader::new(File::open(path).unwrap());
        let iter = iterator_from_mgf(reader, MgfKind::MsConvert);

        // do nothing, just check it parses.
        for item in iter {
            bencher::black_box(item).unwrap();
        }
    }

    //TODO(ahuszagh)
    //  Add more MGF types...
}
