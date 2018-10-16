//! Model for UniProt protein collections.

use traits::*;
use util::{ErrorType, ResultType};
use super::record::{Record};

/// UniProt record collection type.
pub type RecordList = Vec<Record>;

impl Valid for RecordList {
    fn is_valid(&self) -> bool {
        self.iter().all(|ref x| x.is_valid())
    }
}

impl Complete for RecordList {
    fn is_complete(&self) -> bool {
        self.iter().all(|ref x| x.is_complete())
    }
}

// TODO(ahuszagh)
//      Restore after BufWriter is used...
//impl Fasta for RecordList {
//    fn to_fasta(&self) -> ResultType<String> {
//        self.to_fasta_lenient()
//    }
//
//    fn from_fasta(fasta: &str) -> ResultType<RecordList> {
//        RecordList::from_fasta_lenient(fasta)
//    }
//}
//
//impl FastaCollection for RecordList {
//    fn to_fasta_strict(&self) -> ResultType<String> {
//        match self.is_empty() {
//            true    => Ok(String::new()),
//            false   => {
//                // construct FASTA records from elements
//                let mut v: Vec<String> = vec![];
//                for record in self {
//                    v.push(record.to_fasta()?);
//                }
//
//                Ok(v.join("\n\n"))
//            }
//        }
//    }
//
//    fn to_fasta_lenient(&self) -> ResultType<String> {
//        match self.is_empty() {
//            true    => Ok(String::new()),
//            false   => {
//                // construct FASTA records from elements
//                let mut v: Vec<String> = vec![];
//                let mut e: ErrorType = From::from("");
//                for record in self {
//                    match record.to_fasta() {
//                        Err(_e)     => e = _e,
//                        Ok(_v)      => v.push(_v),
//                    }
//                }
//
//                match v.is_empty() {
//                    true  => Err(e),
//                    false => Ok(v.join("\n\n"))
//                }
//            }
//        }
//    }
//
//    fn from_fasta_strict(fasta: &str) -> ResultType<RecordList> {
//        match fasta.is_empty() {
//            true    => Ok(RecordList::new()),
//            false   => {
//                // import records from FASTA
//                let mut v: RecordList = vec![];
//                let records = fasta.split("\n\n");
//                for record in records {
//                    v.push(Record::from_fasta(record)?);
//                }
//
//                Ok(v)
//            }
//        }
//    }
//
//    fn from_fasta_lenient(fasta: &str) -> ResultType<RecordList> {
//        match fasta.is_empty() {
//            true    => Ok(RecordList::new()),
//            false   => {
//                // import records from FASTA
//                let mut v: RecordList = vec![];
//                let mut e: ErrorType = From::from("");
//                let records = fasta.split("\n\n");
//                for record in records {
//                    match Record::from_fasta(record) {
//                        Err(_e)     => e = _e,
//                        Ok(_v)      => v.push(_v),
//                    }
//                }
//
//                match v.is_empty() {
//                    true  => Err(e),
//                    false => Ok(v)
//                }
//            }
//        }
//    }
//}


// TESTS
// -----

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;
    use super::super::record::*;

    /// Create a record for the standard protein GAPDH.
    fn gapdh() -> Record {
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
    fn bsa() -> Record {
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

    // LIST

    #[test]
    fn debug_list() {
        let l = format!("{:?}", vec![gapdh(), bsa()]);
        assert_eq!(l, "[Record { sequence_version: 3, protein_evidence: ProteinLevel, mass: 35780, length: 333, gene: \"GAPDH\", id: \"P46406\", mnemonic: \"G3P_RABIT\", name: \"Glyceraldehyde-3-phosphate dehydrogenase\", organism: \"Oryctolagus cuniculus\", proteome: \"UP000001811\", sequence: \"MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\", taxonomy: \"9986\" }, Record { sequence_version: 4, protein_evidence: ProteinLevel, mass: 69293, length: 607, gene: \"ALB\", id: \"P02769\", mnemonic: \"ALBU_BOVIN\", name: \"Serum albumin\", organism: \"Bos taurus\", proteome: \"UP000009136\", sequence: \"MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\", taxonomy: \"9913\" }]");
    }

    #[test]
    fn equality_list() {
        let x = vec![gapdh(), bsa()];
        let y = vec![gapdh(), bsa()];
        let z = vec![gapdh(), gapdh()];
        assert_eq!(x, y);
        assert_ne!(x, z);
        assert_ne!(y, z);
    }

    #[test]
    fn properties_list() {
        // initial check
        let x = vec![gapdh(), Record::new()];
        let mut y = vec![gapdh(), bsa()];
        assert!(!x.is_valid());
        assert!(!x.is_complete());
        assert!(y.is_valid());
        assert!(y.is_complete());

        // remove a necessary qualifier for complete
        y[1].proteome = String::new();
        assert!(y.is_valid());
        assert!(!y.is_complete());

        // remove a necessary qualifier for valid
        y[1].sequence_version = 0;
        assert!(!y.is_valid());
        assert!(!y.is_complete());
    }

    #[test]
    fn serde_list() {
        let x = serde_json::to_string(&vec![gapdh(), bsa()]).unwrap();
        assert_eq!(x, "[{\"sequence_version\":3,\"protein_evidence\":1,\"mass\":35780,\"length\":333,\"gene\":\"GAPDH\",\"id\":\"P46406\",\"mnemonic\":\"G3P_RABIT\",\"name\":\"Glyceraldehyde-3-phosphate dehydrogenase\",\"organism\":\"Oryctolagus cuniculus\",\"proteome\":\"UP000001811\",\"sequence\":\"MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\",\"taxonomy\":\"9986\"},{\"sequence_version\":4,\"protein_evidence\":1,\"mass\":69293,\"length\":607,\"gene\":\"ALB\",\"id\":\"P02769\",\"mnemonic\":\"ALBU_BOVIN\",\"name\":\"Serum albumin\",\"organism\":\"Bos taurus\",\"proteome\":\"UP000009136\",\"sequence\":\"MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\",\"taxonomy\":\"9913\"}]");
    }

// TODO(ahuszagh)
//      Restore
//    #[test]
//    fn fasta_list() {
//        let v: RecordList = vec![gapdh(), bsa()];
//
//        // to_fasta (valid, 2 items)
//        let x = v.to_fasta().unwrap();
//        assert_eq!(x, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\n\n>sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4\nMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPF\nDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEP\nERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYY\nANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVA\nRLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKE\nCCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRR\nHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEK\nLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLIL\nNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLP\nDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVV\nSTQTALA");
//        assert_eq!(x, v.to_fasta_strict().unwrap());
//        assert_eq!(x, v.to_fasta_lenient().unwrap());
//
//        // from_fasta (valid, 2 items)
//        let y = RecordList::from_fasta(&x).unwrap();
//        assert_eq!(y, RecordList::from_fasta_strict(&x).unwrap());
//        assert_eq!(y, RecordList::from_fasta_lenient(&x).unwrap());
//        assert_eq!(y[0].id, "P46406");
//        assert_eq!(y[1].id, "P02769");
//
//        // completeness check
//        for i in 0..2 {
//            assert!(v[i].is_valid());
//            assert!(v[i].is_complete());
//            assert!(y[i].is_valid());
//            assert!(!y[i].is_complete());
//        }
//
//        // to_fasta (empty)
//        let v: RecordList = vec![];
//        let x = v.to_fasta().unwrap();
//        assert_eq!(x, "");
//        assert_eq!(x, v.to_fasta_strict().unwrap());
//        assert_eq!(x, v.to_fasta_lenient().unwrap());
//
//        // from_fasta (empty)
//        let y = RecordList::from_fasta(&x).unwrap();
//        assert_eq!(y, RecordList::from_fasta_strict(&x).unwrap());
//        assert_eq!(y, RecordList::from_fasta_lenient(&x).unwrap());
//        assert_eq!(y.len(), 0);
//
//        // to_fasta (1 empty)
//        let v: RecordList = vec![Record::new()];
//        let x = v.to_fasta().unwrap();
//        assert_eq!(x, ">sp||  OS= GN= PE=5 SV=0");
//
//        // to_fasta (1 valid, 1 empty)
//        let v: RecordList = vec![gapdh(), Record::new()];
//        let x = v.to_fasta().unwrap();
//        assert_eq!(x, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\n\n>sp||  OS= GN= PE=5 SV=0");
//    }
}
