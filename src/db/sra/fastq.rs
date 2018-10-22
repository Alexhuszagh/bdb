//! Helper utilities for FASTQ loading and saving.

use std::io::{BufRead};

use util::{BufferType};

// TODO(ahuszagh)
//  Implement...

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
