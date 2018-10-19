//! Shared helper functions for unittest definitions.

#[cfg(test)] use traits::*;
#[cfg(test)] use super::evidence::ProteinEvidence;
#[cfg(test)] use super::record::Record;

/// Create a record for the standard protein GAPDH.
#[cfg(test)]
pub fn gapdh() -> Record {
    Record {
        sequence_version: 3,
        protein_evidence: ProteinEvidence::ProteinLevel,
        mass: 35780,
        length: 333,
        gene: String::from("GAPDH"),
        id: String::from("P46406"),
        mnemonic: String::from("G3P_RABIT"),
        name: String::from("Glyceraldehyde-3-phosphate dehydrogenase"),
        organism: String::from("Oryctolagus cuniculus"),
        proteome: String::from("UP000001811"),
        sequence: String::from("MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE"),
        taxonomy: String::from("9986")
    }
}

/// Create a record for the standard protein BSA.
#[cfg(test)]
pub fn bsa() -> Record {
    Record {
        sequence_version: 4,
        protein_evidence: ProteinEvidence::ProteinLevel,
        mass: 69293,
        length: 607,
        gene: String::from("ALB"),
        id: String::from("P02769"),
        mnemonic: String::from("ALBU_BOVIN"),
        name: String::from("Serum albumin"),
        organism: String::from("Bos taurus"),
        proteome: String::from("UP000009136"),
        sequence: String::from("MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA"),
        taxonomy: String::from("9913")
    }
}


/// Check a record from FASTA with incomplete data is equal to the original.
#[cfg(test)]
pub fn incomplete_eq(x: &Record, y: &Record) {
    assert_eq!(y.sequence_version, x.sequence_version);
    assert_eq!(y.protein_evidence, x.protein_evidence);
    assert_eq!(y.mass, x.mass);
    assert_eq!(y.length, x.length);
    assert_eq!(y.gene, x.gene);
    assert_eq!(y.id, x.id);
    assert_eq!(y.mnemonic, x.mnemonic);
    assert_eq!(y.name, x.name);
    assert_eq!(y.organism, x.organism);
    assert_eq!(y.proteome, "");
    assert_eq!(y.sequence, x.sequence);
    assert_eq!(y.taxonomy, "");

    assert!(x.is_valid());
    assert!(x.is_complete());

    assert!(y.is_valid());
    assert!(!y.is_complete());
}
