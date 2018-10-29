use std::convert::AsRef;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Cursor, Write};
use std::path::Path;

use util::Result;

/// Serialize to and from FASTQ.
///
/// # Serialized Format
///
/// ```text
/// @SRR390728.1 1 length=72
/// CATTCTTCACGTAGTTCTCGAGCCTTGGTTTTCAGCGATGGAGAATGACTTTGACAAGCTGAGAGAAGNTNC
/// +SRR390728.1 1 length=72
/// ;;;;;;;;;;;;;;;;;;;;;;;;;;;9;;665142;;;;;;;;;;;;;;;;;;;;;;;;;;;;;96&&&&(
/// ```
pub trait Fastq: Sized {
    /// Estimate the size of the resulting FASTQ output to avoid reallocations.
    #[inline(always)]
    fn estimate_fastq_size(&self) -> usize {
        0
    }

    /// Export model to FASTQ.
    ///
    /// Note that many small writers are made to the writer, so the writer
    /// should be buffered.
    fn to_fastq<T: Write>(&self, writer: &mut T) -> Result<()>;

    /// Export model to FASTQ string.
    fn to_fastq_string(&self) -> Result<String> {
        let capacity = self.estimate_fastq_size();
        let mut writer = Cursor::new(Vec::with_capacity(capacity));

        self.to_fastq(&mut writer)?;
        Ok(String::from_utf8(writer.into_inner())?)
    }

    /// Export model to FASTQ output file.
    #[inline]
    fn to_fastq_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.to_fastq(&mut writer)
    }

    /// Import model from FASTQ.
    fn from_fastq<T: BufRead>(reader: &mut T) -> Result<Self>;

    /// Import model from FASTQ string.
    #[inline]
    fn from_fastq_string(text: &str) -> Result<Self> {
        // Rust uses the contents of the immutable &str as the buffer
        // Cursor is then immutable.
        let mut reader = Cursor::new(text);
        Self::from_fastq(&mut reader)
    }

    /// Import model from FASTQ file.
    #[inline]
    fn from_fastq_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Self::from_fastq(&mut reader)
    }
}

/// Specialization of the `Fastq` trait for collections.
pub trait FastqCollection: Fastq {
    /// Export collection to FASTQ.
    ///
    /// Returns an error if any of the items within the collection
    /// are invalid.
    ///
    /// Note that many small writers are made to the writer, so the writer
    /// should be buffered.
    fn to_fastq_strict<T: Write>(&self, writer: &mut T) -> Result<()>;

    /// Export collection to FASTQ.
    ///
    /// Returns only errors due to serialization issues, otherwise,
    /// exports as many items as possible.
    ///
    /// Note that many small writers are made to the writer, so the writer
    /// should be buffered.
    fn to_fastq_lenient<T: Write>(&self, writer: &mut T) -> Result<()>;

    /// Import collection from FASTQ.
    ///
    /// Returns an error if any of the items within the FASTQ document
    /// are invalid.
    fn from_fastq_strict<T: BufRead>(reader: &mut T) -> Result<Self>;

    /// Import collection from FASTQ.
    ///
    /// Returns only errors due to deserialization errors, otherwise,
    /// imports as many items as possible.
    fn from_fastq_lenient<T: BufRead>(reader: &mut T) -> Result<Self>;
}
