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
    fn to_fasta(&self) -> ResultType<String>;

    /// Export model from FASTA.
    fn from_fasta(fasta: &str) -> ResultType<Self>;
}

/// Specialization of the `Fasta` trait for collections.
pub trait FastaCollection: Fasta {
    /// Export collection to FASTA.
    ///
    /// Returns an error if any of the items within the collection
    /// are invalid.
    fn to_fasta_strict(&self) -> ResultType<String>;

    /// Export collection to FASTA.
    ///
    /// Returns an error if none of the items are valid, otherwise,
    /// exports as many items as possible.
    fn to_fasta_lenient(&self) -> ResultType<String>;

    /// Import collection from FASTA.
    ///
    /// Returns an error if any of the items within the FASTA document
    /// are invalid.
    fn from_fasta_strict(fasta: &str) -> ResultType<Self>;

    /// Import collection from FASTA.
    ///
    /// Returns an error if none of the items within the FASTA document
    /// are valid, otherwise, imports as many items as possible.
    fn from_fasta_lenient(fasta: &str) -> ResultType<Self>;
}
