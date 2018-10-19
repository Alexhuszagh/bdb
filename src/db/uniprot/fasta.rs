//! Helper utilities for FASTA loading and saving.

use std::io::{BufRead, Write};
use std::str as stdstr;

use bio::proteins::{AverageMass, ProteinMass};
use traits::Fasta;
use util::ResultType;
use super::error::UniProtErrorKind;
use super::evidence::ProteinEvidence;
use super::re::*;
use super::record::Record;

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
pub fn estimate_record_size(record: &Record) -> usize {
    const FASTA_VOCABULARY_SIZE: usize = 20;
    FASTA_VOCABULARY_SIZE +
        record.gene.len() +
        record.id.len() +
        record.mnemonic.len() +
        record.name.len() +
        record.organism.len() +
        record.sequence.len()
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

// TODO(ahuszagh)
//      Need iterator to fasta

// READER

/// Import record from FASTA.
pub fn fasta_to_record<T: BufRead>(reader: &mut T)
    -> ResultType<Record>
{
    type R = FastaHeaderRegex;

    // split along lines
    // first line is the header, rest are the sequences
    // short-circuit if the header is None.
    let mut lines = reader.lines();
    let header = lines.next();
    if header.is_none() {
        return Err(From::from(UniProtErrorKind::InvalidInputData));
    }
    let header = header.unwrap()?;

    // process the header and match it to the FASTA record
    let captures = FastaHeaderRegex::extract().captures(&header);
    if captures.is_none() {
        return Err(From::from(UniProtErrorKind::InvalidInputData));
    }
    let captures = captures.unwrap();

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

// TODO(ahuszagh)
//      Need iterator from fasta

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
        fasta_to_record(reader)
    }
}

// TODO(ahuszagh)
//      Need fasta_to_iterator
//      Need iterator_to_fasta

// TESTS
// -----

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn fasta_iterator() {
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

    // TODO(ahuszagh)
    //      Test record_to_fasta
}
