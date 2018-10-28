use std::convert::AsRef;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Cursor, Write};
use std::path::Path;

use util::ResultType;

/// Serialize to and from FASTA.
///
/// # Serialized Format
///
/// ```text
/// >sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3
/// MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA
/// ENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS
/// APSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT
/// ATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV
/// VDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA
/// LNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE
/// ```
pub trait Fasta: Sized {
    /// Estimate the size of the resulting FASTA output to avoid reallocations.
    #[inline(always)]
    fn estimate_fasta_size(&self) -> usize {
        0
    }

    /// Export model to FASTA.
    ///
    /// Note that many small writers are made to the writer, so the writer
    /// should be buffered.
    fn to_fasta<T: Write>(&self, writer: &mut T) -> ResultType<()>;

    /// Export model to FASTA string.
    fn to_fasta_string(&self) -> ResultType<String> {
        let capacity = self.estimate_fasta_size();
        let mut writer = Cursor::new(Vec::with_capacity(capacity));

        self.to_fasta(&mut writer)?;
        match String::from_utf8(writer.into_inner()) {
            Err(e)  => Err(Box::new(e)),
            Ok(v)   => Ok(v),
        }
    }

    /// Export model to FASTA output file.
    #[inline]
    fn to_fasta_file<P: AsRef<Path>>(&self, path: P) -> ResultType<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.to_fasta(&mut writer)
    }

    /// Import model from FASTA.
    fn from_fasta<T: BufRead>(reader: &mut T) -> ResultType<Self>;

    /// Import model from FASTA string.
    #[inline]
    fn from_fasta_string(text: &str) -> ResultType<Self> {
        // Rust uses the contents of the immutable &str as the buffer
        // Cursor is then immutable.
        let mut reader = Cursor::new(text);
        Self::from_fasta(&mut reader)
    }

    /// Import model from FASTA file.
    #[inline]
    fn from_fasta_file<P: AsRef<Path>>(path: P) -> ResultType<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Self::from_fasta(&mut reader)
    }
}

/// Specialization of the `Fasta` trait for collections.
pub trait FastaCollection: Fasta {
    /// Export collection to FASTA.
    ///
    /// Returns an error if any of the items within the collection
    /// are invalid.
    ///
    /// Note that many small writers are made to the writer, so the writer
    /// should be buffered.
    fn to_fasta_strict<T: Write>(&self, writer: &mut T) -> ResultType<()>;

    /// Export collection to FASTA.
    ///
    /// Returns only errors due to serialization issues, otherwise,
    /// exports as many items as possible.
    ///
    /// Note that many small writers are made to the writer, so the writer
    /// should be buffered.
    fn to_fasta_lenient<T: Write>(&self, writer: &mut T) -> ResultType<()>;

    /// Import collection from FASTA.
    ///
    /// Returns an error if any of the items within the FASTA document
    /// are invalid.
    fn from_fasta_strict<T: BufRead>(reader: &mut T) -> ResultType<Self>;

    /// Import collection from FASTA.
    ///
    /// Returns only errors due to deserialization errors, otherwise,
    /// imports as many items as possible.
    fn from_fasta_lenient<T: BufRead>(reader: &mut T) -> ResultType<Self>;
}
