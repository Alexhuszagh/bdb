//! Helper utilities for FASTQ loading and saving.

use std::io::{BufRead};

use util::{BufferType};

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

// TODO(ahuszagh)
//  Implement the iterator....
