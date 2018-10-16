#[allow(unused_imports)]        // TODO(ahuszagh)       Remove
use std::io::{BufReader, BufWriter, Cursor, Read, Write};
use util::ResultType;

/// Serialize to and from FASTA.
///
/// # Serialized Format
///
/// >sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3
/// MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA
/// ENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS
/// APSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT
/// ATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV
/// VDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA
/// LNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE
pub trait Fasta: Sized {
    /// Export model to FASTA.
    fn to_fasta<T: Write>(&self, writer: &mut BufWriter<T>) -> ResultType<()>;
    //async fn to_fasta_async<T: Write>(&self, writer: &mut BufWriter<T>) -> ResultType<()>;

//    fn to_fasta_buf<T: Write>(&self, writer: &mut BufWriter<T>) -> ResultType<()> {
//        match self.to_fasta() {
//            Err(e) => Err(e),
//            Ok(v)  => {
//                match writer.write_all(v.as_bytes()) {
//                    Err(e) => Err(Box::new(e)),
//                    _      => Ok(()),
//                }
//            }
//        }
//    }
//
//    fn to_fasta_string(&self) -> ResultType<String> {
//        let mut writer = BufWriter::new(Cursor::new(Vec::new()));
//
//        match self.to_fasta_buf(&mut writer) {
//            Err(e)  => Err(e),
//            _       => {
//                match writer.into_inner() {
//                    Err(e)  => Err(Box::new(e)),
//                    Ok(c)   => match String::from_utf8(c.into_inner()) {
//                        Err(e)  => Err(Box::new(e)),
//                        Ok(v)   => Ok(v),
//                    }
//                }
//            },
//        }
//    }

    /// Export model from FASTA.
    fn noop() {}
    // TODO(ahuszagh)       Restore
    //fn from_fasta(fasta: &str) -> ResultType<Self>;
    //async fn from_fasta_async(fasta: &str) -> ResultType<Self>;
}

/// Specialization of the `Fasta` trait for collections.
pub trait FastaCollection: Fasta {
    /// Export collection to FASTA.
    ///
    /// Returns an error if any of the items within the collection
    /// are invalid.
    fn to_fasta_strict(&self) -> ResultType<String>;
    //async fn to_fasta_strict_async(&self) -> ResultType<String>;

    /// Export collection to FASTA.
    ///
    /// Returns an error if none of the items are valid, otherwise,
    /// exports as many items as possible.
    fn to_fasta_lenient(&self) -> ResultType<String>;
    //async fn to_fasta_lenient_async(&self) -> ResultType<String>;

    /// Import collection from FASTA.
    ///
    /// Returns an error if any of the items within the FASTA document
    /// are invalid.
    fn from_fasta_strict(fasta: &str) -> ResultType<Self>;
    //async fn from_fasta_strict_async(fasta: &str) -> ResultType<Self>;

    /// Import collection from FASTA.
    ///
    /// Returns an error if none of the items within the FASTA document
    /// are valid, otherwise, imports as many items as possible.
    fn from_fasta_lenient(fasta: &str) -> ResultType<Self>;
    //async fn from_fasta_lenient_async(fasta: &str) -> ResultType<Self>;
}
