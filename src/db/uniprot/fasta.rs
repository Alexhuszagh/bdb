//! Helper utilities for FASTA loading and saving.

use std::io::{BufRead, Write};
use std::str as stdstr;

use bio::proteins::{AverageMass, ProteinMass};
use traits::*;
use util::{ErrorType, ResultType};
use super::error::UniProtErrorKind;
use super::evidence::ProteinEvidence;
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
    buf: Vec<u8>,
    line: String,
}

impl<T: BufRead> FastaIter<T> {
    /// Create new FastaIter from a buffered reader.
    pub fn new(reader: T) -> Self {
        FastaIter {
            reader: reader,
            buf: Vec::with_capacity(8000),
            line: String::with_capacity(8000)
        }
    }
}

impl<T: BufRead> Iterator for FastaIter<T> {
    type Item = ResultType<String>;

    fn next(&mut self) -> Option<Self::Item> {
        // Clear our previous inputs, but keep our buffer capacities.
        unsafe {
            self.line.as_mut_vec().set_len(0);
            self.buf.set_len(0);
        }

        // Indefinitely loop over lines.
        loop {
            match self.reader.read_line(&mut self.line) {
                Err(e)      => return Some(Err(From::from(e))),
                Ok(size)    => match size {
                    // Reached EOF
                    0   => break,
                    // Read bytes, process them.
                    _   => unsafe {
                        // Check if we are at the end of a record
                        // marked by only whitespace.
                        if self.line == "\n" || self.line == "\r\n" {
                            // If we don't have any data, ignore repetitive
                            // whitespace, otherwise, end the record.
                            self.line.as_mut_vec().set_len(0);
                            match self.buf.len() {
                                0 => continue,
                                _ => break,
                            }
                        }

                        // Move all elements in `s` to `self.buf`.
                        let v = self.line.as_mut_vec();
                        self.buf.append(v);
                    },
                }
            }
        }

        match self.buf.len() {
            // No record present, at EOF
            0 => None,
            // Data present return
            _ => Some(match stdstr::from_utf8(&self.buf) {
                Err(e)  => Err(From::from(e)),
                Ok(v)   => Ok(String::from(v)),
            })
        }
    }
}

// SIZE

/// Estimate the size of a FASTA record.
///
/// Used to prevent reallocations during record exportation to string,
/// to minimize costly library calls.
#[inline]
pub fn estimate_record_size(record: &Record) -> usize {
    // The vocabulary size is actually 20, overestimate to adjust for number export.
    const FASTA_VOCABULARY_SIZE: usize = 40;
    FASTA_VOCABULARY_SIZE +
        record.gene.len() +
        record.id.len() +
        record.mnemonic.len() +
        record.name.len() +
        record.organism.len() +
        record.sequence.len()
}

/// Estimate the size of a FASTA record list.
#[inline]
pub fn estimate_list_size(list: &RecordList) -> usize {
    list.iter().fold(0, |sum, x| sum + x.estimate_fasta_size())
}

// WRITER STATE

/// Internal struct to store the current writer state.
struct WriterState<'r, T: 'r + Write> {
    writer: &'r mut T,
    /// Whether a record has previously been written successfully.
    success: bool,
    /// Whether an error has occurred loading a record.
    has_errored: bool,
    /// Whether the previous record exported successfully.
    previous: bool,
    /// Current error.
    error: ErrorType,
}

impl<'r, T: 'r + Write> WriterState<'r, T> {
    /// Construct new state from writer.
    #[inline]
    fn new(writer: &'r mut T) -> WriterState<'r, T> {
        WriterState {
            writer: writer,
            success: false,
            has_errored: false,
            previous: false,
            error: From::from("")
        }
    }

    /// Mark success.
    #[inline]
    fn set_success(&mut self) {
        self.previous = true;
        self.success = true;
    }

    /// Mark failure.
    #[inline]
    fn set_error(&mut self, error: ErrorType) {
        self.error = error;
        self.previous = false;
        self.has_errored = true;
    }

    /// Export record to FASTA.
    fn to_fasta(&mut self, record: &Record) -> ResultType<()> {
        // Only write the prefix if the last export worked.
        if self.previous {
            self.writer.write_all(b"\n\n")?;
        }

        match record.to_fasta(self.writer) {
            Err(e)  => self.set_error(e),
            _       => self.set_success(),
        }

        Ok(())
    }

    /// Consume the state and get the result.
    #[inline]
    fn result(self) -> ResultType<()> {
        match self.success || !self.has_errored {
            true    => Ok(()),
            false   => Err(self.error),
        }
    }
}

// WRITER

/// Export record to FASTA.
pub fn record_to_fasta<T: Write>(record: &Record, writer: &mut T)
    -> ResultType<()>
{
    // Write SwissProt header
    write_alls!(
        writer,
        b">sp|",     record.id.as_bytes(),
        b"|",        record.mnemonic.as_bytes(),
        b" ",        record.name.as_bytes(),
        b" OS=",     record.organism.as_bytes(),
        b" GN=",     record.gene.as_bytes(),
        b" PE=",     record.protein_evidence.to_string().as_bytes(),
        b" SV=",     record.sequence_version.to_string().as_bytes()
    )?;

    // Write SwissProt sequence, formatted at 60 characters.
    // Write the initial, 60 character lines
    const SEQUENCE_LINE_LENGTH: usize = 60;
    let mut bytes = record.sequence.as_bytes();
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

/// Default exporter from a non-owning iterator to FASTA.
pub fn reference_iterator_to_fasta<'a, Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    let mut state = WriterState::new(writer);

    // Write all records
    // Error only raised for write error, which should percolate.
    for record in iter {
        state.to_fasta(record)?;
    }

    state.result()
}


/// Default exporter from an owning iterator to FASTA.
pub fn value_iterator_to_fasta<Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    let mut state = WriterState::new(writer);

    // Write all records
    // Error only raised for read or write errors, which should percolate.
    for record in iter {
        state.to_fasta(&record?)?;
    }

    state.result()
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to FASTA.
pub fn reference_iterator_to_fasta_strict<'a, Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    // Write all records, prepending "\n\n" after the first record
    let mut previous = false;
    for record in iter {
        if record.is_valid() {
            if previous {
                writer.write_all(b"\n\n")?;
            }
            record.to_fasta(writer)?;
            previous = true;
        } else {
            return Err(From::from(UniProtErrorKind::InvalidRecord));
        }
    }

    Ok(())
}

/// Strict exporter from an owning iterator to FASTA.
pub fn value_iterator_to_fasta_strict<Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    // Write all records, prepending "\n\n" after the first record
    let mut previous = false;
    for result in iter {
        let record = result?;
        if record.is_valid() {
            if previous {
                writer.write_all(b"\n\n")?;
            }
            record.to_fasta(writer)?;
            previous = true;
        } else {
            return Err(From::from(UniProtErrorKind::InvalidRecord));
        }
    }

    Ok(())
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to FASTA.
pub fn reference_iterator_to_fasta_lenient<'a, Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    let mut state = WriterState::new(writer);

    // Write all records
    // Error only raised for write error, which should percolate.
    for record in iter {
        if record.is_valid() {
            state.to_fasta(record)?;
        }
    }

    Ok(())
}

/// Lenient exporter from an owning iterator to FASTA.
pub fn value_iterator_to_fasta_lenient<Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    let mut state = WriterState::new(writer);

    // Write all records
    // Error only raised for write error, which should percolate.
    for result in iter {
        let record = result?;
        if record.is_valid() {
            state.to_fasta(&record)?;
        }
    }

    Ok(())
}

// READER

/// Import record from FASTA.
pub fn record_from_fasta<T: BufRead>(reader: &mut T)
    -> ResultType<Record>
{
    type R = FastaHeaderRegex;

    // split along lines
    // first line is the header, rest are the sequences
    // short-circuit if the header is None.
    let mut lines = reader.lines();
    let header = match lines.next() {
        None    => return Err(From::from(UniProtErrorKind::InvalidInputData)),
        Some(v) => v?,
    };

    // process the header and match it to the FASTA record
    let captures = match FastaHeaderRegex::extract().captures(&header) {
        None    => return Err(From::from(UniProtErrorKind::InvalidInputData)),
        Some(v) => v,
    };

    // initialize the record with header data
    let pe = capture_as_str(&captures, R::PE_INDEX);
    let sv = capture_as_str(&captures, R::SV_INDEX);
    let mut record = Record {
        // Can use unwrap because they were matched in the regex
        // as "\d+" capture groups, they must be deserializeable to int.
        sequence_version: sv.parse().unwrap(),
        protein_evidence: ProteinEvidence::from_str(pe)?,
        mass: 0,
        length: 0,
        gene: capture_as_string(&captures, R::GENE_INDEX),
        id: capture_as_string(&captures, R::ACCESSION_INDEX),
        mnemonic: capture_as_string(&captures, R::MNEMONIC_INDEX),
        name: capture_as_string(&captures, R::NAME_INDEX),
        organism: capture_as_string(&captures, R::ORGANISM_INDEX),

        // unused fields in header
        proteome: String::new(),
        sequence: String::new(),
        taxonomy: String::new(),
    };

    // add sequence data to the FASTA sequence
    for line in lines {
        record.sequence.push_str(&line?);
    }

    // calculate the protein length and mass
    if record.sequence.len() > 0 {
        record.length = record.sequence.len() as u32;
        let mass = AverageMass::protein_sequence_mass(record.sequence.as_bytes());
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
    type Item = ResultType<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = match self.iter.next()? {
            Err(e)   => return Some(Err(e)),
            Ok(text) => text,

        };

        Some(Record::from_fasta_string(&text))
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
pub struct FastaRecordStrictIter<T: BufRead> {
    iter: FastaIter<T>
}

impl<T: BufRead> FastaRecordStrictIter<T> {
    /// Create new FastaRecordStrictIter from a buffered reader.
    #[inline]
    pub fn new(reader: T) -> Self {
        FastaRecordStrictIter {
            iter: FastaIter::new(reader)
        }
    }
}

impl<T: BufRead> Iterator for FastaRecordStrictIter<T> {
    type Item = ResultType<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = match self.iter.next()? {
            Err(e)   => return Some(Err(e)),
            Ok(text) => text,

        };

        Some(Record::from_fasta_string(&text).and_then(|r| {
            match r.is_valid() {
                true    => Ok(r),
                false   => Err(From::from(UniProtErrorKind::InvalidRecord)),
            }
        }))
    }
}

/// Create default record iterator from reader.
#[inline(always)]
pub fn iterator_from_fasta_strict<T: BufRead>(reader: T) -> FastaRecordStrictIter<T> {
    FastaRecordStrictIter::new(reader)
}

// READER -- LENIENT

/// Iterator to lazily load `Record`s from a document.
///
/// Wraps `FastaIter` and converts the text to records leniently.
pub struct FastaRecordLenientIter<T: BufRead> {
    iter: FastaIter<T>,
}

impl<T: BufRead> FastaRecordLenientIter<T> {
    /// Create new FastaRecordLenientIter from a buffered reader.
    #[inline]
    pub fn new(reader: T) -> Self {
        FastaRecordLenientIter {
            iter: FastaIter::new(reader),
        }
    }
}

impl<T: BufRead> Iterator for FastaRecordLenientIter<T> {
    type Item = ResultType<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let text = match self.iter.next()? {
                Err(e)   => return Some(Err(e)),
                Ok(text) => text,
            };

            match Record::from_fasta_string(&text) {
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
pub fn iterator_from_fasta_lenient<T: BufRead>(reader: T) -> FastaRecordLenientIter<T> {
    FastaRecordLenientIter::new(reader)
}

// TRAITS

impl Fasta for Record {
    #[inline]
    fn estimate_fasta_size(&self) -> usize {
        estimate_record_size(self)
    }

    #[inline(always)]
    fn to_fasta<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        record_to_fasta(self, writer)
    }

    fn from_fasta<T: BufRead>(reader: &mut T) -> ResultType<Self> {
        record_from_fasta(reader)
    }
}

impl Fasta for RecordList {
    #[inline]
    fn estimate_fasta_size(&self) -> usize {
        estimate_list_size(self)
    }

    #[inline(always)]
    fn to_fasta<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        reference_iterator_to_fasta(self.iter(), writer)
    }

    #[inline(always)]
    fn from_fasta<T: BufRead>(reader: &mut T) -> ResultType<RecordList> {
        iterator_from_fasta(reader).collect()
    }
}

impl FastaCollection for RecordList {
    #[inline(always)]
    fn to_fasta_strict<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        reference_iterator_to_fasta_strict(self.iter(), writer)
    }

    #[inline(always)]
    fn to_fasta_lenient<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        reference_iterator_to_fasta_lenient(self.iter(), writer)
    }

    #[inline(always)]
    fn from_fasta_strict<T: BufRead>(reader: &mut T) -> ResultType<RecordList> {
        iterator_from_fasta_strict(reader).collect()
    }

    #[inline(always)]
    fn from_fasta_lenient<T: BufRead>(reader: &mut T) -> ResultType<RecordList> {
        Ok(iterator_from_fasta_lenient(reader).filter_map(Result::ok).collect())
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
    fn fasta_iter_test() {
        // Check iterator over data.
        let s = "Line1\nLine2\n\nRecord2\nLine2\r\n\n\nRecord3\n";
        let i = FastaIter::new(Cursor::new(s));
        let r: ResultType<Vec<String>> = i.collect();
        assert_eq!(r.unwrap(), &["Line1\nLine2\n", "Record2\nLine2\r\n", "Record3\n"]);

        // Check iterator over empty string.
        let s = "";
        let i = FastaIter::new(Cursor::new(s));
        let r: ResultType<Vec<String>> = i.collect();
        assert_eq!(r.unwrap(), Vec::<String>::new());
    }

    #[test]
    fn estimate_size_test() {
        let g = gapdh();
        let b = bsa();
        let v = vec![gapdh(), bsa()];
        assert_eq!(estimate_record_size(&g), 454);
        assert_eq!(estimate_record_size(&b), 689);
        assert_eq!(estimate_list_size(&v), 1143);
    }

    macro_rules! by_value {
        ($x:expr) => ($x.iter().map(|x| { Ok(x.clone()) }))
    }

    #[test]
    fn iterator_to_fasta_test() {
        let v = vec![gapdh(), bsa()];
        let u = vec![gapdh(), bsa(), Record::new()];

        // reference -- default
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_fasta(v.iter(), &mut w).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_FASTA);

        // value -- default
        let mut w = Cursor::new(vec![]);
        value_iterator_to_fasta(by_value!(v), &mut w).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_FASTA);

        // reference -- strict
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_fasta_strict(v.iter(), &mut w).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_FASTA);

        let mut w = Cursor::new(vec![]);
        let r = reference_iterator_to_fasta_strict(u.iter(), &mut w);
        assert!(r.is_err());

        // value -- strict
        let mut w = Cursor::new(vec![]);
        value_iterator_to_fasta_strict(by_value!(v), &mut w).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_FASTA);

        let mut w = Cursor::new(vec![]);
        let r = value_iterator_to_fasta_strict(by_value!(u), &mut w);
        assert!(r.is_err());

        // reference -- lenient
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_fasta_lenient(v.iter(), &mut w).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_FASTA);

        let mut w = Cursor::new(vec![]);
        reference_iterator_to_fasta_lenient(u.iter(), &mut w).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_FASTA);

        // value -- lenient
        let mut w = Cursor::new(vec![]);
        value_iterator_to_fasta_lenient(by_value!(v), &mut w).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_FASTA);

        let mut w = Cursor::new(vec![]);
        value_iterator_to_fasta_lenient(by_value!(u), &mut w).unwrap();
        assert_eq!(String::from_utf8(w.into_inner()).unwrap(), GAPDH_BSA_FASTA);
    }

    #[test]
    fn iterator_from_fasta_test() {
        // VALID
        let text = GAPDH_BSA_FASTA;
        let expected = vec![gapdh(), bsa()];

        // record iterator -- default
        let iter = FastaRecordIter::new(Cursor::new(text));
        let v: ResultType<RecordList> = iter.collect();
        incomplete_list_eq(&expected, &v.unwrap());

        // Compile check only
        iterator_from_fasta(&mut Cursor::new(text));

        // record iterator -- strict
        let iter = FastaRecordStrictIter::new(Cursor::new(text));
        let v: ResultType<RecordList> = iter.collect();
        incomplete_list_eq(&expected, &v.unwrap());

        // Compile check only
        iterator_from_fasta_strict(&mut Cursor::new(text));

        // record iterator -- lenient
        let iter = FastaRecordLenientIter::new(Cursor::new(text));
        let v: ResultType<RecordList> = iter.collect();
        incomplete_list_eq(&expected, &v.unwrap());

        // Compile check only
        iterator_from_fasta_lenient(&mut Cursor::new(text));

        // INVALID
        let text = GAPDH_EMPTY_FASTA;
        let expected1 = vec![gapdh(), Record::new()];
        let expected2 = vec![gapdh()];

        // record iterator -- default
        let iter = iterator_from_fasta(Cursor::new(text));
        let v: ResultType<RecordList> = iter.collect();
        let v = v.unwrap();
        assert_eq!(expected1.len(), v.len());
        incomplete_eq(&expected1[0], &v[0]);
        assert_eq!(expected1[1], v[1]);

        // record iterator -- strict
        let iter = iterator_from_fasta_strict(Cursor::new(text));
        let v: ResultType<RecordList> = iter.collect();
        assert!(v.is_err());

        // record iterator -- lenient
        let iter = iterator_from_fasta_lenient(Cursor::new(text));
        let v: ResultType<RecordList> = iter.collect();
        incomplete_list_eq(&expected2, &v.unwrap());
    }
}
