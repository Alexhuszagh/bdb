//! Model for UniProt protein definitions.

use super::evidence::ProteinEvidence;

/// Model for a single record from a UniProt KB query.
///
/// Record including core query fields for a given UniProt identifier.
/// The query fields are defined [here](http://www.uniprot.org/help/query-fields).
///
/// # Advanced
///
/// The following is a mapping of the UniProt form-encoded keys, struct
/// field names, and UniProt displayed column names.
/// Despite the name correspondence, the information may not be a
/// identical in one format or another, for example,
/// [`protein_evidence`] is an enumeration, while in a displayed
/// column it's a string, and in FASTA it's a numerical identifier.
/// [`ProteinEvidence.ProteinLevel`] is the same as `"Evidence at protein
/// level"` which is the same as `1`.
///
/// | Field Name           | Form-Encoded Key     | Displayed Column       |
/// |----------------------|----------------------|------------------------|
/// | [`sequence_version`] | version(sequence)    | Sequence version       |
/// | [`protein_evidence`] | existence            | Protein existence      |
/// | [`mass`]             | mass                 | Mass                   |
/// | [`length`]           | length               | Length                 |
/// | [`gene`]             | genes(PREFERRED)     | Gene names  (primary ) |
/// | [`id`]               | id                   | Entry                  |
/// | [`mnemonic`]         | entry name           | Entry name             |
/// | [`name`]             | protein names        | Protein names          |
/// | [`organism`]         | organism             | Organism               |
/// | [`proteome`]         | proteome             | Proteomes              |
/// | [`sequence`]         | sequence             | Sequence               |
/// | [`taxonomy`]         | organism-id          | Organism ID            |
///
/// [`sequence_version`]: struct.Record.html#structfield.sequence_version
/// [`protein_evidence`]: struct.Record.html#structfield.protein_evidence
/// [`mass`]: struct.Record.html#structfield.mass
/// [`length`]: struct.Record.html#structfield.length
/// [`gene`]: struct.Record.html#structfield.gene
/// [`id`]: struct.Record.html#structfield.id
/// [`mnemonic`]: struct.Record.html#structfield.mnemonic
/// [`name`]: struct.Record.html#structfield.name
/// [`organism`]: struct.Record.html#structfield.organism
/// [`proteome`]: struct.Record.html#structfield.proteome
/// [`sequence`]: struct.Record.html#structfield.sequence
/// [`taxonomy`]: struct.Record.html#structfield.taxonomy
/// [`ProteinEvidence.ProteinLevel`]: enum.ProteinEvidence.html#variant.ProteinLevel

// Enumerated values for Record fields.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RecordField {
    SequenceVersion,
    ProteinEvidence,
    Mass,
    Length,
    Gene,
    Id,
    Mnemonic,
    Name,
    Organism,
    Proteome,
    Sequence,
    Taxonomy,
}

// Extra information hidden from the documentation, for developers.
//  Notes:
//      `sequence_version`:
//          Simple integer in all variants.
//
//      `protein_evidence
//          Enumerated value, which appears as a string or integer, with
//          the mapping defined in `ProteinEvidence` and
//          `ProteinEvidence::verbose()`.
//
//      `mass`:
//          Simple integer in all variants.
//
//      `length`:
//          Simple integer in all variants.
//
//      `gene`:
//          TODO(ahuszagh) [I believe this frequently gives more than
//          one gene name, confirm with the unannotated human proteome.
//          If so, designate a regex for filtering from external queries.]
//
//      `id`:
//          Accession number as a string.
//
//      `mnemonic`:
//          Mnemonic identifier as a string.
//
//      `name`:
//          Name for the protein (ex. Glyceraldehyde-3-phosphate
//          dehydrogenase). However, UniProt frequently spits out
//          more than one possible protein name, with each subsequent
//          name enclosed in parentheses (ex. "Glyceraldehyde-3-phosphate
//          dehydrogenase (GAPDH) (EC 1.2.1.12) (Peptidyl-cysteine
//          S-nitrosylase GAPDH) (EC 2.6.99.-)").
//
//      `organism`:
//          Species name (with an optional common name in parentheses).
//          BDB considers the common name superfluous, and therefore
//          removes it from all records fetched from external queries.
//          Strain information, which is also enclosed in parentheses,
//          however, should not be removed.
//
//      `proteome`:
//          Proteomes include a proteome identifier and an optional
//          proteome location, for example, "UP000001811: Unplaced",
//          "UP000001114: Chromosome", and "UP000001811" are all valid
//          values. We discard the location, and solely store the proteome
//          identifier.
//
//      `sequence`:
//          Aminoacid sequence of the protein, as a string.
//
//      `taxonomy`:
//          Numerical identifier for the species, described by "name".
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    /// Numerical identifier for protein version.
    ///
    /// Value starts from 1, and is incremented for each revision of the protein.
    pub sequence_version: u8,
    /// Enumeration for the strength of evidence for the protein existence.
    pub protein_evidence: ProteinEvidence,
    /// Mass of the protein.
    pub mass: u64,
    /// Protein sequence length.
    pub length: u32,
    /// HGNC Gene name.
    pub gene: String,
    /// Accession number (randomly assigned identifier).
    pub id: String,
    /// Entry name (readable identifier).
    pub mnemonic: String,
    /// Protein name.
    pub name: String,
    /// Readable organism name.
    pub organism: String,
    /// UniProt proteome identifier.
    pub proteome: String,
    /// Protein aminoacid sequence.
    pub sequence: String,
    /// Taxonomic identifier.
    pub taxonomy: String,
}


impl Record {
    /// Create new, empty UniProt record.
    #[inline]
    pub fn new() -> Self {
        Record {
            sequence_version: 0,
            protein_evidence: ProteinEvidence::Unknown,
            mass: 0,
            length: 0,
            gene: String::new(),
            id: String::new(),
            mnemonic: String::new(),
            name: String::new(),
            organism: String::new(),
            proteome: String::new(),
            sequence: String::new(),
            taxonomy: String::new(),
        }
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use traits::*;
    use super::*;
    use super::super::test::{bsa, gapdh, incomplete_eq};

    #[test]
    fn debug_record() {
        let text = format!("{:?}", gapdh());
        assert_eq!(text, "Record { sequence_version: 3, protein_evidence: ProteinLevel, mass: 35780, length: 333, gene: \"GAPDH\", id: \"P46406\", mnemonic: \"G3P_RABIT\", name: \"Glyceraldehyde-3-phosphate dehydrogenase\", organism: \"Oryctolagus cuniculus\", proteome: \"UP000001811\", sequence: \"MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\", taxonomy: \"9986\" }");

        let text = format!("{:?}", bsa());
        assert_eq!(text, "Record { sequence_version: 4, protein_evidence: ProteinLevel, mass: 69293, length: 607, gene: \"ALB\", id: \"P02769\", mnemonic: \"ALBU_BOVIN\", name: \"Serum albumin\", organism: \"Bos taurus\", proteome: \"UP000009136\", sequence: \"MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\", taxonomy: \"9913\" }");
    }

    #[test]
    fn equality_record() {
        let x = gapdh();
        let y = gapdh();
        let z = bsa();
        assert_eq!(x, y);
        assert_ne!(x, z);
        assert_ne!(y, z);
    }

    #[test]
    fn properties_record() {
        // test various permutations that can lead to
        // invalid or incomplete identifications
        let g1 = gapdh();
        let mut g2 = g1.clone();
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g1.estimate_fasta_size(), 434);
        assert_eq!(g2.estimate_fasta_size(), 434);

        // check keeping the protein valid but make it incomplete
        g2.proteome = String::new();
        assert!(g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 434);
        g2.proteome = g1.proteome.clone();

        g2.taxonomy = String::new();
        assert!(g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 434);
        g2.taxonomy = g1.taxonomy.clone();

        // check replacing items with valid, but different data
        g2.sequence_version = 1;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 434);
        g2.sequence_version = g1.sequence_version;

        g2.protein_evidence = ProteinEvidence::Inferred;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 434);
        g2.protein_evidence = g1.protein_evidence;

        g2.mass = 64234;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 434);
        g2.mass = g1.mass;

        g2.sequence = String::from(&g2.sequence[0..200]);
        g2.length = 200;
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 301);
        g2.sequence = g1.sequence.clone();
        g2.length = g1.length;

        g2.gene = String::from("HIST1H1A");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 437);
        g2.gene = g1.gene.clone();

        g2.id = String::from("A0A022YWF9");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 438);
        g2.id = g1.id.clone();

        g2.id = String::from("A2BC19");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 434);
        g2.id = g1.id.clone();

        g2.mnemonic = String::from("H11_HUMAN");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 434);
        g2.mnemonic = g1.mnemonic.clone();

        g2.name = String::from("Histone H1.1");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 406);
        g2.name = g1.name.clone();

        g2.organism = String::from("Homo sapiens");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 425);
        g2.organism = g1.organism.clone();

        g2.proteome = String::from("UP000005640");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 434);
        g2.proteome = g1.proteome.clone();

        g2.taxonomy = String::from("9606");
        assert!(g2.is_valid());
        assert!(g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 434);
        g2.taxonomy = g1.taxonomy.clone();

        // check replacing items with invalid data
        g2.sequence_version = 0;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 434);
        g2.sequence_version = g1.sequence_version;

        g2.protein_evidence = ProteinEvidence::Unknown;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 434);
        g2.protein_evidence = g1.protein_evidence;

        g2.mass = 0;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 434);
        g2.mass = g1.mass;

        g2.length = 334;
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 434);
        g2.length = g1.length;

        g2.gene = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 429);
        g2.gene = g1.gene.clone();

        g2.id = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 428);
        g2.id = g1.id.clone();

        g2.mnemonic = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 425);
        g2.mnemonic = g1.mnemonic.clone();

        g2.name = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 394);
        g2.name = g1.name.clone();

        g2.organism = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 413);
        g2.organism = g1.organism.clone();

        g2.sequence = String::new();
        assert!(!g2.is_valid());
        assert!(!g2.is_complete());
        assert_eq!(g2.estimate_fasta_size(), 101);
        g2.sequence = g1.sequence.clone();
    }

    #[test]
    fn fasta_record() {
        // gapdh
        let p = gapdh();
        let x = p.to_fasta_string().unwrap();
        assert_eq!(x, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE");
        let y = Record::from_fasta_string(&x).unwrap();
        incomplete_eq(&p, &y);

        // bsa
        let p = bsa();
        let x = p.to_fasta_string().unwrap();
        assert_eq!(x, ">sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4\nMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPF\nDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEP\nERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYY\nANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVA\nRLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKE\nCCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRR\nHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEK\nLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLIL\nNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLP\nDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVV\nSTQTALA");
        let y = Record::from_fasta_string(&x).unwrap();
        incomplete_eq(&p, &y);

        // empty
        let p = Record::new();
        let x = p.to_fasta_string().unwrap();
        assert_eq!(x, ">sp||  OS= GN= PE=5 SV=0");
        let y = Record::from_fasta_string(&x).unwrap();
        assert_eq!(p, y);
    }

    #[test]
    fn csv_record() {
        // gapdh
        let p = gapdh();
        let x = p.to_csv_string(b'\t').unwrap();
        // TODO(ahuszagh)
        //      This is going to change, due to thousands separators...
        assert_eq!(x, "Sequence version\tProtein existence\tMass\tLength\tGene names  (primary )\tEntry\tEntry name\tProtein names\tOrganism\tProteomes\tSequence\tOrganism ID\n3\tEvidence at protein level\t35780\t333\tGAPDH\tP46406\tG3P_RABIT\tGlyceraldehyde-3-phosphate dehydrogenase\tOryctolagus cuniculus\tUP000001811\tMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\t9986\n");
        let x = p.to_csv_string(b',').unwrap();
        assert_eq!(x, "Sequence version,Protein existence,Mass,Length,Gene names  (primary ),Entry,Entry name,Protein names,Organism,Proteomes,Sequence,Organism ID\n3,Evidence at protein level,35780,333,GAPDH,P46406,G3P_RABIT,Glyceraldehyde-3-phosphate dehydrogenase,Oryctolagus cuniculus,UP000001811,MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE,9986\n");
        let y = Record::from_csv_string(&x, b',').unwrap();
        assert_eq!(p, y);

        // bsa
        let p = bsa();
        let x = p.to_csv_string(b'\t').unwrap();
        assert_eq!(x, "Sequence version\tProtein existence\tMass\tLength\tGene names  (primary )\tEntry\tEntry name\tProtein names\tOrganism\tProteomes\tSequence\tOrganism ID\n4\tEvidence at protein level\t69293\t607\tALB\tP02769\tALBU_BOVIN\tSerum albumin\tBos taurus\tUP000009136\tMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\t9913\n");
        let x = p.to_csv_string(b',').unwrap();
        assert_eq!(x, "Sequence version,Protein existence,Mass,Length,Gene names  (primary ),Entry,Entry name,Protein names,Organism,Proteomes,Sequence,Organism ID\n4,Evidence at protein level,69293,607,ALB,P02769,ALBU_BOVIN,Serum albumin,Bos taurus,UP000009136,MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA,9913\n");
        let y = Record::from_csv_string(&x, b',').unwrap();
        assert_eq!(p, y);

        // empty
        let p = Record::new();
        let x = p.to_csv_string(b'\t').unwrap();
        assert_eq!(x, "Sequence version\tProtein existence\tMass\tLength\tGene names  (primary )\tEntry\tEntry name\tProtein names\tOrganism\tProteomes\tSequence\tOrganism ID\n\t\t\t\t\t\t\t\t\t\t\t\n");
        let x = p.to_csv_string(b',').unwrap();
        assert_eq!(x, "Sequence version,Protein existence,Mass,Length,Gene names  (primary ),Entry,Entry name,Protein names,Organism,Proteomes,Sequence,Organism ID\n,,,,,,,,,,,\n");
        let y = Record::from_csv_string(&x, b',').unwrap();
        assert_eq!(p, y);
    }
}
