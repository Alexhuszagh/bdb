//! Helper utilities for MGF loading and saving.

use std::io::prelude::*;
use std::str as stdstr;

use traits::*;
use util::*;
use super::fullms_mgf::*;
use super::msconvert_mgf::*;
use super::pava_mgf::*;
use super::pwiz_mgf::*;
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
    buf: Buffer,
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
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        text_next!(&mut self.reader, &mut self.buf, &mut self.line, unsafe {
            if self.line == "\n" || self.line == "\r\n" || self.line.starts_with("MASS=") {
                // Ignore whitespace and lines with "Mass".
                self.line.as_mut_vec().set_len(0);
                continue;
            } else if self.buf.len() > 0 && self.line.starts_with(self.start) {
                // Create result from existing buffer,
                // clear the existing buffer, and add
                // the current line to a new buffer.
                let result = buffer_to_string!(self.buf);
                self.buf.append(self.line.as_mut_vec());
                return result;
            } else {
                // Move the line to the buffer.
                self.buf.append(self.line.as_mut_vec());
            }
        })
    }
}

// SIZE

/// Estimate the size of an MGF record.
#[inline(always)]
fn estimate_record_size(record: &Record, kind: MgfKind) -> usize {
    match kind {
        MgfKind::MsConvert => estimate_msconvert_mgf_record_size(record),
        MgfKind::Pava => estimate_pava_mgf_record_size(record),
        MgfKind::Pwiz => estimate_pwiz_mgf_record_size(record),
        MgfKind::FullMs => estimate_fullms_mgf_record_size(record),
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
    -> Result<()>
{
    match kind {
        MgfKind::MsConvert => record_to_msconvert_mgf(writer, record),
        MgfKind::Pava => record_to_pava_mgf(writer, record),
        MgfKind::Pwiz => record_to_pwiz_mgf(writer, record),
        MgfKind::FullMs => record_to_fullms_mgf(writer, record),
    }
}

// WRITER -- DEFAULT

/// Default exporter from a non-owning iterator to MGF.
#[inline(always)]
pub fn reference_iterator_to_mgf<'a, Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    match kind {
        MgfKind::MsConvert => reference_iterator_to_msconvert_mgf(writer, iter),
        MgfKind::Pava => reference_iterator_to_pava_mgf(writer, iter),
        MgfKind::Pwiz => reference_iterator_to_pwiz_mgf(writer, iter),
        MgfKind::FullMs => reference_iterator_to_fullms_mgf(writer, iter),
    }
}

/// Default exporter from an owning iterator to MGF.
#[inline(always)]
pub fn value_iterator_to_mgf<Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    match kind {
        MgfKind::MsConvert => value_iterator_to_msconvert_mgf(writer, iter),
        MgfKind::Pava => value_iterator_to_pava_mgf(writer, iter),
        MgfKind::Pwiz => value_iterator_to_pwiz_mgf(writer, iter),
        MgfKind::FullMs => value_iterator_to_fullms_mgf(writer, iter),
    }
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to MGF.
#[inline(always)]
pub fn reference_iterator_to_mgf_strict<'a, Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    match kind {
        MgfKind::MsConvert => reference_iterator_to_msconvert_mgf_strict(writer, iter),
        MgfKind::Pava => reference_iterator_to_pava_mgf_strict(writer, iter),
        MgfKind::Pwiz => reference_iterator_to_pwiz_mgf_strict(writer, iter),
        MgfKind::FullMs => reference_iterator_to_fullms_mgf_strict(writer, iter),
    }
}

/// Strict exporter from an owning iterator to MGF.
#[inline(always)]
pub fn value_iterator_to_mgf_strict<Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    match kind {
        MgfKind::MsConvert => value_iterator_to_msconvert_mgf_strict(writer, iter),
        MgfKind::Pava => value_iterator_to_pava_mgf_strict(writer, iter),
        MgfKind::Pwiz => value_iterator_to_pwiz_mgf_strict(writer, iter),
        MgfKind::FullMs => value_iterator_to_fullms_mgf_strict(writer, iter),
    }
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to MGF.
#[inline(always)]
pub fn reference_iterator_to_mgf_lenient<'a, Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    match kind {
        MgfKind::MsConvert => reference_iterator_to_msconvert_mgf_lenient(writer, iter),
        MgfKind::Pava => reference_iterator_to_pava_mgf_lenient(writer, iter),
        MgfKind::Pwiz => reference_iterator_to_pwiz_mgf_lenient(writer, iter),
        MgfKind::FullMs => reference_iterator_to_fullms_mgf_lenient(writer, iter),
    }
}

/// Lenient exporter from an owning iterator to MGF.
#[inline(always)]
pub fn value_iterator_to_mgf_lenient<Iter, T>(writer: &mut T, iter: Iter, kind: MgfKind)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    match kind {
        MgfKind::MsConvert => value_iterator_to_msconvert_mgf_lenient(writer, iter),
        MgfKind::Pava => value_iterator_to_pava_mgf_lenient(writer, iter),
        MgfKind::Pwiz => value_iterator_to_pwiz_mgf_lenient(writer, iter),
        MgfKind::FullMs => value_iterator_to_fullms_mgf_lenient(writer, iter),
    }
}

// READER

/// Import record from MGF.
pub fn record_from_mgf<T: BufRead>(reader: &mut T, kind: MgfKind)
    -> Result<Record>
{
    match kind {
        MgfKind::MsConvert => record_from_msconvert_mgf(reader),
        MgfKind::Pava => record_from_pava_mgf(reader),
        MgfKind::Pwiz => record_from_pwiz_mgf(reader),
        MgfKind::FullMs => record_from_fullms_mgf(reader),
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
    type Item = Result<Record>;

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
        MgfKind::Pwiz => iterator_from_pwiz_mgf(reader),
        MgfKind::FullMs => iterator_from_fullms_mgf(reader),
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
    fn to_mgf<T: Write>(&self, writer: &mut T, kind: MgfKind) -> Result<()> {
        record_to_mgf(writer, self, kind)
    }

    #[inline(always)]
    fn from_mgf<T: BufRead>(reader: &mut T, kind: MgfKind) -> Result<Self> {
        record_from_mgf(reader, kind)
    }
}

impl Mgf for RecordList {
    #[inline]
    fn estimate_mgf_size(&self, kind: MgfKind) -> usize {
        estimate_list_size(self, kind)
    }

    #[inline(always)]
    fn to_mgf<T: Write>(&self, writer: &mut T, kind: MgfKind) -> Result<()> {
        reference_iterator_to_mgf(writer, self.iter(), kind)
    }

    #[inline(always)]
    fn from_mgf<T: BufRead>(reader: &mut T, kind: MgfKind) -> Result<Self> {
        iterator_from_mgf(reader, kind).collect()
    }
}

impl MgfCollection for RecordList {
    #[inline(always)]
    fn to_mgf_strict<T: Write>(&self, writer: &mut T, kind: MgfKind) -> Result<()> {
        reference_iterator_to_mgf_strict(writer, self.iter(), kind)
    }

    #[inline(always)]
    fn to_mgf_lenient<T: Write>(&self, writer: &mut T, kind: MgfKind) -> Result<()> {
        reference_iterator_to_mgf_lenient(writer, self.iter(), kind)
    }

    #[inline(always)]
    fn from_mgf_strict<T: BufRead>(reader: &mut T, kind: MgfKind) -> Result<RecordList> {
        iterator_from_mgf_strict(reader, kind).collect()
    }

    #[inline(always)]
    fn from_mgf_lenient<T: BufRead>(reader: &mut T, kind: MgfKind) -> Result<RecordList> {
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
        let s = "BEGIN IONS\nT=A\nEND IONS\nBEGIN IONS\nT=B\nEND IONS\n";
        let i = MgfIter::new(Cursor::new(s), "BEGIN IONS");
        let r: Result<Vec<String>> = i.collect();
        assert_eq!(r.unwrap(), &["BEGIN IONS\nT=A\nEND IONS\n", "BEGIN IONS\nT=B\nEND IONS\n"]);

        // Check iterator over empty string.
        let s = "";
        let i = MgfIter::new(Cursor::new(s), "BEGIN IONS");
        let r: Result<Vec<String>> = i.collect();
        assert_eq!(r.unwrap(), Vec::<String>::new());

        // Check iterator over different delimiter.
        let s = "Scan#: 2182\n\n\nScan#: 2191\n\n\n";
        let i = MgfIter::new(Cursor::new(s), "Scan#: ");
        let r: Result<Vec<String>> = i.collect();
        assert_eq!(r.unwrap(), &["Scan#: 2182\n", "Scan#: 2191\n"]);

        // Check iterator with mass.
        let s = "MASS=Mono\nBEGIN IONS\nT=A\nEND IONS\nBEGIN IONS\nT=B\nEND IONS\n";
        let i = MgfIter::new(Cursor::new(s), "BEGIN IONS");
        let r: Result<Vec<String>> = i.collect();
        assert_eq!(r.unwrap(), &["BEGIN IONS\nT=A\nEND IONS\n", "BEGIN IONS\nT=B\nEND IONS\n"]);
    }

    #[test]
    fn estimate_size_test() {
        let s = mgf_33450();
        let e = mgf_empty();
        let v = vec![mgf_33450(), mgf_empty()];

        // MSConvert
        let kind = MgfKind::MsConvert;
        assert_eq!(estimate_record_size(&s, kind), 1987);
        assert_eq!(estimate_record_size(&e, kind), 262);
        assert_eq!(estimate_list_size(&v, kind), 2249);

        // Pava
        let kind = MgfKind::Pava;
        assert_eq!(estimate_record_size(&s, kind), 1856);
        assert_eq!(estimate_record_size(&e, kind), 131);
        assert_eq!(estimate_list_size(&v, kind), 1987);

        // Pwiz
        let kind = MgfKind::Pwiz;
        assert_eq!(estimate_record_size(&s, kind), 1906);
        assert_eq!(estimate_record_size(&e, kind), 181);
        assert_eq!(estimate_list_size(&v, kind), 2087);
    }

    fn iterator_to_mgf_test(kind: MgfKind, expected: &str) {
        let v = vec![mgf_33450()];
        let u = vec![mgf_33450(), mgf_empty()];

        // reference -- default
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_mgf(&mut w, v.iter(), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), expected);

        // value -- default
        let mut w = Cursor::new(vec![]);
        value_iterator_to_mgf(&mut w, iterator_by_value!(v.iter()), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), expected);

        // reference -- strict
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_mgf_strict(&mut w, v.iter(), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), expected);

        let mut w = Cursor::new(vec![]);
        let r = reference_iterator_to_mgf_strict(&mut w, u.iter(), kind);
        assert!(r.is_err());

        // value -- strict
        let mut w = Cursor::new(vec![]);
        value_iterator_to_mgf_strict(&mut w, iterator_by_value!(v.iter()), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), expected);

        let mut w = Cursor::new(vec![]);
        let r = value_iterator_to_mgf_strict(&mut w, iterator_by_value!(u.iter()), kind);
        assert!(r.is_err());

        // reference -- lenient
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_mgf_lenient(&mut w, v.iter(), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), expected);

        let mut w = Cursor::new(vec![]);
        reference_iterator_to_mgf_lenient(&mut w, u.iter(), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), expected);

        // value -- lenient
        let mut w = Cursor::new(vec![]);
        value_iterator_to_mgf_lenient(&mut w, iterator_by_value!(v.iter()), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), expected);

        let mut w = Cursor::new(vec![]);
        value_iterator_to_mgf_lenient(&mut w, iterator_by_value!(u.iter()), kind).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), expected);
    }

    fn iterator_from_mgf_test_valid(kind: MgfKind, input: &str, expected: RecordList) {
        // record iterator -- default
        let iter = iterator_from_mgf(Cursor::new(input), kind);
        let v: Result<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());

        // record iterator -- strict
        let iter = iterator_from_mgf_strict(Cursor::new(input), kind);
        let v: Result<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());

        // record iterator -- lenient
        let iter = iterator_from_mgf_lenient(Cursor::new(input), kind);
        let v: Result<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());
    }

    fn iterator_from_mgf_test_invalid(kind: MgfKind, input: &str, expected: RecordList) {
        // record iterator -- default
        let iter = iterator_from_mgf(Cursor::new(input), kind);
        let v: Result<RecordList> = iter.collect();
        assert_eq!(expected, v.unwrap());

        // record iterator -- strict
        let iter = iterator_from_mgf_strict(Cursor::new(input), kind);
        let v: Result<RecordList> = iter.collect();
        assert!(v.is_err());

        // record iterator -- lenient
        let iter = iterator_from_mgf_lenient(Cursor::new(input), kind);
        let v: Result<RecordList> = iter.collect();
        assert_eq!(v.unwrap().len(), 0);
    }

    // FULLMS

    #[test]
    fn iterator_to_fullms_mgf_test() {
        iterator_to_mgf_test(MgfKind::FullMs, FULLMS_33450_MGF)
    }

    #[test]
    fn iterator_from_fullms_mgf_test() {
        iterator_from_mgf_test_valid(MgfKind::FullMs, FULLMS_33450_MGF, vec![fullms_mgf_33450()]);
        iterator_from_mgf_test_invalid(MgfKind::FullMs, FULLMS_EMPTY_MGF, vec![fullms_mgf_empty()]);
    }

    // MSCONVERT

    #[test]
    fn iterator_to_msconvert_mgf_test() {
        iterator_to_mgf_test(MgfKind::MsConvert, MSCONVERT_33450_MGF)
    }

    #[test]
    fn iterator_from_msconvert_mgf_test() {
        iterator_from_mgf_test_valid(MgfKind::MsConvert, MSCONVERT_33450_MGF, vec![mgf_33450()]);
        iterator_from_mgf_test_invalid(MgfKind::MsConvert, MSCONVERT_EMPTY_MGF, vec![mgf_empty()]);
    }

    // PAVA

    #[test]
    fn iterator_to_pava_mgf_test() {
        iterator_to_mgf_test(MgfKind::Pava, PAVA_33450_MGF)
    }

    #[test]
    fn iterator_from_pava_mgf_test() {
        iterator_from_mgf_test_valid(MgfKind::Pava, PAVA_33450_MGF, vec![mgf_33450()]);
        iterator_from_mgf_test_invalid(MgfKind::Pava, PAVA_EMPTY_MGF, vec![mgf_empty()]);
    }

    // PWIZ

    #[test]
    fn iterator_to_pwiz_mgf_test() {
        iterator_to_mgf_test(MgfKind::Pwiz, PWIZ_33450_MGF)
    }

    #[test]
    fn iterator_from_pwiz_mgf_test() {
        iterator_from_mgf_test_valid(MgfKind::Pwiz, PWIZ_33450_MGF, vec![mgf_33450()]);
        iterator_from_mgf_test_invalid(MgfKind::Pwiz, PWIZ_EMPTY_MGF, vec![mgf_empty()]);
    }

    // FILE

    fn mgf_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("mass_spectra/mgf");
        dir
    }

    fn mgf_file_test(path: PathBuf, kind: MgfKind) {
        let reader = BufReader::new(File::open(path).unwrap());
        let iter = iterator_from_mgf(reader, kind);

        // do nothing, just check it parses.
        for item in iter {
            bencher::black_box(item).unwrap();
        }
    }

    #[test]
    #[ignore]
    fn fullms_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_fullms.txt");
        mgf_file_test(path, MgfKind::FullMs);
    }

    #[test]
    #[ignore]
    fn msconvert_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_msconvert_ms2.txt");
        mgf_file_test(path, MgfKind::MsConvert);
    }

    #[test]
    #[ignore]
    fn pava_ms2_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_pava_ms2.txt");
        mgf_file_test(path, MgfKind::Pava);
    }

    #[test]
    #[ignore]
    fn pava_ms3_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_pava_ms3.txt");
        mgf_file_test(path, MgfKind::Pava);
    }

    #[test]
    #[ignore]
    fn pava_ms3_20170411_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_pava_ms3_20170411.txt");
        mgf_file_test(path, MgfKind::Pava);
    }

    #[test]
    #[ignore]
    fn pwiz_ms2_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_pwiz_ms2.txt");
        mgf_file_test(path, MgfKind::Pwiz);
    }

    #[test]
    #[ignore]
    fn pwiz_ms3_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_pwiz_ms3.txt");
        mgf_file_test(path, MgfKind::Pwiz);
    }
}
