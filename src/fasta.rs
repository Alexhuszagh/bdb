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
     *  \brief Export struct to FASTA.
     */
    fn to_fasta(&self) -> Option<String>;

    /**
     *  \brief Import struct from FASTA.
     */
    fn from_fasta(fasta: &str) -> Option<Self>;
}
