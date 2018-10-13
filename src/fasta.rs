/**
 *  FASTA
 *  -----
 *
 *  Trait for FASTA protein serializers and deserializers.
 *
 *  :copyright: (c) 2018 Alex Huszagh.
 *  :license: MIT, see LICENSE.md for more details.
 */

// TRAITS
// ------

/**
 *  \brief Trait that defines FASTA serializers and deserializers.
 *
 *  The `to_fasta` method should return a `String` of the following format,
 *  while the `from_fasta` method should create a struct instance from a
 *  string of the following format.
 *
 *  \format
 *      >sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3
 *      MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA
 *      ENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS
 *      APSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT
 *      ATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV
 *      VDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA
 *      LNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE
 */
pub trait Fasta: Sized {
    /**
     *  \brief Export record to FASTA.
     */
    fn to_fasta(&self) -> Result<String, &str>;

    /**
     *  \brief Import record from FASTA.
     */
    fn from_fasta<'a>(fasta: &str) -> Result<Self, &'a str>;
}

/**
 *  \brief Specialized version of the Fasta trait for collections.
 */
pub trait FastaCollection: Sized {
    /**
     *  \brief Export collection of UniProt records to FASTA.
     *
     *  `to_fasta_strict` requires all records inside the collection
     *  to be valid, or returns an `Err`, while `to_fasta_lenient` will
     *  return as many formatted records as possible, returning an error
     *  only if no records are valid.
     */
     fn to_fasta_strict(&self) -> Result<String, &str>;
     fn to_fasta_lenient(&self) -> Result<String, &str>;

    /**
     *  \brief Import record collection from FASTA.
     *
     *  `from_fasta_strict` requires all records inside the FASTA text
     *  to be valid, or returns an `Err`, while `to_fasta_lenient` will
     *  return as many record structs as possible, returning an error
     *  only if no records are valid.
     */
    fn from_fasta_strict<'a>(fasta: &str) -> Result<Self, &'a str>;
    fn from_fasta_lenient<'a>(fasta: &str) -> Result<Self, &'a str>;
}
