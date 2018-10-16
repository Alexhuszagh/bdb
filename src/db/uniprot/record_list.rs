//! Model for UniProt protein collections.

use std::io::{self, BufRead, Write};
use std::str as stdstr;

use traits::*;
use util::{ErrorType, ResultType};
use super::record::{Record};

/// UniProt record collection type.
pub type RecordList = Vec<Record>;

impl Valid for RecordList {
    #[inline]
    fn is_valid(&self) -> bool {
        self.iter().all(|ref x| x.is_valid())
    }
}

impl Complete for RecordList {
    #[inline]
    fn is_complete(&self) -> bool {
        self.iter().all(|ref x| x.is_complete())
    }
}

/// Iterator to parse individual FASTA entries from a document.
struct FastaIterator<T: BufRead> {
    reader: T,
    buf: Vec<u8>,
    line: String,
}

impl<T: BufRead> FastaIterator<T> {
    /// Create new FastaIterator from a buffered reader.
    pub fn new(reader: T) -> FastaIterator<T> {
        FastaIterator {
            reader: reader,
            buf: Vec::with_capacity(8000),
            line: String::with_capacity(8000)
        }
    }
}


impl<T: BufRead> Iterator for FastaIterator<T> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        // Clear our previous inputs, but keep our buffer capacities.
        unsafe {
            self.line.as_mut_vec().set_len(0);
            self.buf.set_len(0);
        }

        // Indefinitely loop over lines.
        loop {
            match self.reader.read_line(&mut self.line) {
                Err(e)      => return Some(Err(e)),
                Ok(size)    => match size {
                    // Reached EOF
                    0   => break,
                    // Read bytes, process them.
                    _   => unsafe {
                        // Check if we are at the end of a record
                        // marked by only whitespace.
                        if self.line == "\n" || self.line == "\r\n" {
                            // If we don't have any data, ignore repetitive
                            // whitespace, otherwise, end the record.
                            self.line.as_mut_vec().set_len(0);
                            match self.buf.len() {
                                0 => continue,
                                _ => break,
                            }
                        }

                        // Move all elements in `s` to `self.buf`.
                        let v = self.line.as_mut_vec();
                        self.buf.append(v);
                    },
                }
            }
        }

        match self.buf.len() {
            // No record present, at EOF
            0 => None,
            // Data present return
            _ => Some(match stdstr::from_utf8(&self.buf) {
                Err(_e) => Err(From::from(io::ErrorKind::InvalidData)),
                Ok(v)   => Ok(String::from(v))
            })
        }
    }
}


impl Fasta for RecordList {
    #[inline]
    fn estimate_fasta_size(&self) -> usize {
        self.iter().fold(0, |sum, x| sum + x.estimate_fasta_size())
    }

    #[inline(always)]
    fn to_fasta<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        self.to_fasta_lenient(writer)
    }

    #[inline(always)]
    fn from_fasta<T: BufRead>(reader: &mut T) -> ResultType<Self> {
        RecordList::from_fasta_lenient(reader)
    }
}

impl FastaCollection for RecordList {
    fn to_fasta_strict<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        match self.is_empty() {
            true    => Ok(()),
            false   => {
                // Write first record so we can precede all following records
                // with a double LF (join-like functionality)
                let mut iter = self.iter();
                match iter.next() {
                    None    => Ok(()),
                    Some(v) => {
                        v.to_fasta(writer)?;

                        // Write the remaining records, prepending "\n\n"
                        for record in iter {
                            writer.write_all(b"\n\n")?;
                            record.to_fasta(writer)?;
                        }

                        Ok(())
                    }
                }
            }
        }
    }

    fn to_fasta_lenient<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        match self.is_empty() {
            true    => Ok(()),
            false   => {
                // Write first record so we can precede all following records
                // with a double LF (join-like functionality)
                //let mut iter = self.iter();
                let mut e: ErrorType = From::from("");
                // Use a boolean to mark if a record has been written (success).
                // If no entries are present, we short-circuit Ok, otherwise,
                // at least 1 entry must be successfully exported to mark Ok().
                let mut success = false;
                // Use a boolean to track if the previous export was successful.
                let mut previous = false;

                // Write all records
                for record in self {
                    // Only write the prefix if the last export worked.
                    if previous {
                        writer.write_all(b"\n\n")?;
                    }

                    match record.to_fasta(writer) {
                        Err(_e) => {
                            e = _e;
                            previous = false;
                        },
                        _       => {
                            previous = true;
                            success = true;
                        },
                    }
                }

                match success {
                    true    => Ok(()),
                    false   => Err(e),
                }
            }
        }
    }

    fn from_fasta_strict<T: BufRead>(reader: &mut T) -> ResultType<Self> {
        let mut list = RecordList::new();
        for result in FastaIterator::new(reader) {
            list.push(Record::from_fasta_string(&result?)?);
        }

        Ok(list)
    }

    fn from_fasta_lenient<T: BufRead>(reader: &mut T) -> ResultType<Self> {
        let mut list = RecordList::new();
        let mut e: ErrorType = From::from("");
        for result in FastaIterator::new(reader) {
            match result {
                Err(_e)     => e = Box::new(_e),
                Ok(text)    => {
                    match Record::from_fasta_string(&text) {
                        Err(_e) => e = _e,
                        Ok(r)   => list.push(r),
                    }
                }
            }
        }

        // Check if the list is empty or the error has been set.
        match list.is_empty() && e.description() != "" {
            true    => Err(e),
            false   => Ok(list)
        }
    }
}


// TESTS
// -----

#[cfg(test)]
mod tests {
    use serde_json;
    use std::io::Cursor;
    use super::*;
    use super::super::record::*;
    use super::super::test::{bsa, gapdh, incomplete_eq};

    // FASTA ITERATOR

    #[test]
    fn fasta_iterator() {
        // Check iterator over data.
        let s = "Line1\nLine2\n\nRecord2\nLine2\r\n\n\nRecord3\n";
        let i = FastaIterator::new(Cursor::new(s));
        let r: io::Result<Vec<String>> = i.collect();
        assert_eq!(r.unwrap(), &["Line1\nLine2\n", "Record2\nLine2\r\n", "Record3\n"]);

        // Check iterator over empty string.
        let s = "";
        let i = FastaIterator::new(Cursor::new(s));
        let r: io::Result<Vec<String>> = i.collect();
        assert_eq!(r.unwrap(), Vec::<String>::new());
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
        assert_eq!(x.estimate_fasta_size(), 454);
        assert_eq!(y.estimate_fasta_size(), 1103);

        // remove a necessary qualifier for complete
        y[1].proteome = String::new();
        assert!(y.is_valid());
        assert!(!y.is_complete());
        assert_eq!(y.estimate_fasta_size(), 1103);

        // remove a necessary qualifier for valid
        y[1].sequence_version = 0;
        assert!(!y.is_valid());
        assert!(!y.is_complete());
        assert_eq!(y.estimate_fasta_size(), 1103);
    }

    #[test]
    fn serde_list() {
        let x = serde_json::to_string(&vec![gapdh(), bsa()]).unwrap();
        assert_eq!(x, "[{\"sequence_version\":3,\"protein_evidence\":1,\"mass\":35780,\"length\":333,\"gene\":\"GAPDH\",\"id\":\"P46406\",\"mnemonic\":\"G3P_RABIT\",\"name\":\"Glyceraldehyde-3-phosphate dehydrogenase\",\"organism\":\"Oryctolagus cuniculus\",\"proteome\":\"UP000001811\",\"sequence\":\"MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\",\"taxonomy\":\"9986\"},{\"sequence_version\":4,\"protein_evidence\":1,\"mass\":69293,\"length\":607,\"gene\":\"ALB\",\"id\":\"P02769\",\"mnemonic\":\"ALBU_BOVIN\",\"name\":\"Serum albumin\",\"organism\":\"Bos taurus\",\"proteome\":\"UP000009136\",\"sequence\":\"MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\",\"taxonomy\":\"9913\"}]");
    }

    #[test]
    fn fasta_list() {
        let v: RecordList = vec![gapdh(), bsa()];

        // to_fasta (valid, 2 items)
        let x = v.to_fasta_string().unwrap();
        assert_eq!(x, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\n\n>sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4\nMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPF\nDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEP\nERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYY\nANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVA\nRLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKE\nCCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRR\nHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEK\nLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLIL\nNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLP\nDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVV\nSTQTALA");

        let mut buf: Vec<u8> = vec![];
        v.to_fasta_strict(&mut Cursor::new(&mut buf)).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\n\n>sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4\nMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPF\nDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEP\nERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYY\nANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVA\nRLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKE\nCCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRR\nHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEK\nLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLIL\nNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLP\nDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVV\nSTQTALA");

        let mut buf: Vec<u8> = vec![];
        v.to_fasta_lenient(&mut Cursor::new(&mut buf)).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\n\n>sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4\nMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPF\nDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEP\nERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYY\nANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVA\nRLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKE\nCCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRR\nHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEK\nLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLIL\nNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLP\nDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVV\nSTQTALA");

        // from_fasta (valid, 2 items)
        let y = RecordList::from_fasta_string(&x).unwrap();
        assert_eq!(y, RecordList::from_fasta_strict(&mut Cursor::new(&x)).unwrap());
        assert_eq!(y, RecordList::from_fasta_lenient(&mut Cursor::new(&x)).unwrap());

        // completeness check
        for i in 0..2 {
            incomplete_eq(&v[i], &y[i]);
        }

        // to_fasta (empty)
        let v: RecordList = vec![];
        let x = v.to_fasta_string().unwrap();
        assert_eq!(x, "");

        let mut buf: Vec<u8> = vec![];
        v.to_fasta_strict(&mut Cursor::new(&mut buf)).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "");

        let mut buf: Vec<u8> = vec![];
        v.to_fasta_lenient(&mut Cursor::new(&mut buf)).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "");

        // from_fasta (empty)
        let y = RecordList::from_fasta_string(&x).unwrap();
        assert_eq!(y, RecordList::from_fasta_strict(&mut Cursor::new(&x)).unwrap());
        assert_eq!(y, RecordList::from_fasta_lenient(&mut Cursor::new(&x)).unwrap());
        assert_eq!(y.len(), 0);

        // to_fasta (1 empty)
        let v: RecordList = vec![Record::new()];
        let x = v.to_fasta_string().unwrap();
        assert_eq!(x, ">sp||  OS= GN= PE=5 SV=0");

        // to_fasta (1 valid, 1 empty)
        let v: RecordList = vec![gapdh(), Record::new()];
        let x = v.to_fasta_string().unwrap();
        assert_eq!(x, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\n\n>sp||  OS= GN= PE=5 SV=0");
    }
}
