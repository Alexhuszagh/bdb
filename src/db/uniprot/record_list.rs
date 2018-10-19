//! Model for UniProt protein collections.

use std::io::{BufRead, Write};

use traits::*;
use util::{ErrorType, ResultType};
use super::fasta::FastaIter;
use super::error::UniProtErrorKind;
use super::record::Record;

/// UniProt record collection type.
pub type RecordList = Vec<Record>;

// FASTA
// -----

/// Internal struct to store the current writer state.
struct WriterState<'r, T: 'r + Write> {
    writer: &'r mut T,
    /// Whether a record has previously been written successfully.
    success: bool,
    /// Whether the previous record exported successfully.
    previous: bool,
    /// Current error.
    error: ErrorType,
}

impl<'r, T: 'r + Write> WriterState<'r, T> {
    /// Construct new state from writer.
    #[inline]
    fn new(writer: &'r mut T) -> WriterState<'r, T> {
        WriterState {
            writer: writer,
            success: false,
            previous: false,
            error: From::from("")
        }
    }
}

/// Write record (non-throwing) to FASTA.
#[inline]
fn record_to_fasta<'r, T>(state: &mut WriterState<'r, T>, record: &Record) -> ResultType<()>
    where T: 'r + Write
{
    // Only write the prefix if the last export worked.
    if state.previous {
        state.writer.write_all(b"\n\n")?;
    }

    match record.to_fasta(state.writer) {
        Err(e) => {
            state.error = e;
            state.previous = false;
        },
        _       => {
            state.previous = true;
            state.success = true;
        },
    }

    Ok(())
}

/// Internal struct to store the current writer state.
struct ReaderState<'r, T: 'r + BufRead> {
    reader: &'r mut T,
    /// Whether an error has occurred loading a record.
    has_errored: bool,
    /// Current output.
    list: RecordList,
    /// Current error.
    error: ErrorType,
}


impl<'r, T: 'r + BufRead> ReaderState<'r, T> {
    /// Construct new state from reader.
    #[inline]
    fn new(reader: &'r mut T) -> ReaderState<'r, T> {
        ReaderState {
            reader: reader,
            has_errored: false,
            list: RecordList::new(),
            error: From::from("")
        }
    }
}

#[inline]
fn record_from_fasta<T>(reader: &mut T, callback: fn(Record, &mut RecordList) -> bool)
    -> ResultType<RecordList>
    where T: BufRead
{
    let mut state = ReaderState::new(reader);
    for result in FastaIter::new(&mut state.reader) {
        match result {
            Err(e)      => state.error = e,
            Ok(text)    => {
                match Record::from_fasta_string(&text) {
                    Err(e)  => state.error = e,
                    Ok(r)   => {
                        if !callback(r, &mut state.list) {
                            state.has_errored = true;
                            state.error = From::from(UniProtErrorKind::InvalidRecord);
                        }
                    },
                }
            }
        }
    }

    // Check if the list is empty or the error has been set.
    match state.list.is_empty() && state.has_errored {
        true    => Err(state.error),
        false   => Ok(state.list)
    }
}


impl Fasta for RecordList {
    #[inline]
    fn estimate_fasta_size(&self) -> usize {
        self.iter().fold(0, |sum, x| sum + x.estimate_fasta_size())
    }

    fn to_fasta<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        match self.is_empty() {
            true    => Ok(()),
            false   => {
                let mut state = WriterState::new(writer);

                // Write all records
                for record in self {
                    // Error only raised for write error, which should percolate.
                    record_to_fasta(&mut state, record)?;
                }

                match state.success {
                    true    => Ok(()),
                    false   => Err(state.error),
                }
            }
        }
    }

    fn from_fasta<T: BufRead>(reader: &mut T) -> ResultType<RecordList> {
        record_from_fasta(reader, | record: Record, list: &mut RecordList | -> bool {
            list.push(record);
            true
        })
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
                        // Check if the record is valid, and error
                        if !v.is_valid() {
                            return Err(From::from(UniProtErrorKind::InvalidRecord));
                        }
                        v.to_fasta(writer)?;

                        // Write the remaining records, prepending "\n\n"
                        for record in iter {
                            if !record.is_valid() {
                                return Err(From::from(UniProtErrorKind::InvalidRecord));
                            }
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
                let mut state = WriterState::new(writer);

                // Write all records
                for record in self {
                    if record.is_valid() {
                        // Error only raised for write error, which should percolate.
                        record_to_fasta(&mut state, record)?;
                    }
                }

                match state.success {
                    true    => Ok(()),
                    false   => Err(state.error),
                }
            }
        }
    }

    fn from_fasta_strict<T: BufRead>(reader: &mut T) -> ResultType<RecordList> {
        let mut list = RecordList::new();
        for result in FastaIter::new(reader) {
            let record = Record::from_fasta_string(&result?)?;
            if record.is_valid() {
                list.push(record);
            } else {
                return Err(From::from(UniProtErrorKind::InvalidRecord));
            }
        }

        Ok(list)
    }

    fn from_fasta_lenient<T: BufRead>(reader: &mut T) -> ResultType<RecordList> {
        record_from_fasta(reader, | record: Record, list: &mut RecordList | -> bool {
            if record.is_valid() {
                list.push(record);
                true
            } else {
                false
            }
        })
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;
    use super::super::test::{bsa, gapdh, incomplete_eq};

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

        let mut buf: Vec<u8> = vec![];
        assert!(v.to_fasta_strict(&mut Cursor::new(&mut buf)).is_err());
        assert!(v.to_fasta_lenient(&mut Cursor::new(&mut buf)).is_err());

        // from_fasta (1 empty)
        let y = RecordList::from_fasta_string(&x).unwrap();
        assert!(RecordList::from_fasta_strict(&mut Cursor::new(&x)).is_err());
        assert!(RecordList::from_fasta_lenient(&mut Cursor::new(&x)).is_err());
        assert_eq!(v, y);

        // to_fasta (1 valid, 1 empty)
        let v: RecordList = vec![gapdh(), Record::new()];
        let x = v.to_fasta_string().unwrap();
        assert_eq!(x, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\n\n>sp||  OS= GN= PE=5 SV=0");

        let mut buf: Vec<u8> = vec![];
        assert!(v.to_fasta_strict(&mut Cursor::new(&mut buf)).is_err());
        v.to_fasta_lenient(&mut Cursor::new(&mut buf)).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE");

        // from_fasta (1 valid, 1 empty)
        let y = RecordList::from_fasta_string(&x).unwrap();
        assert!(RecordList::from_fasta_strict(&mut Cursor::new(&x)).is_err());
        let z = RecordList::from_fasta_lenient(&mut Cursor::new(&x)).unwrap();
        incomplete_eq(&v[0], &y[0]);
        incomplete_eq(&v[0], &z[0]);
        assert_eq!(v[1], y[1]);
        assert_eq!(z.len(), 1);
    }

    #[test]
    fn csv_list() {
        let v: RecordList = vec![gapdh(), bsa()];

        // to_fasta (valid, 2 items)
        let x = v.to_csv_string(b'\t').unwrap();
        assert_eq!(x, "Sequence version\tProtein existence\tMass\tLength\tGene names  (primary )\tEntry\tEntry name\tProtein names\tOrganism\tProteomes\tSequence\tOrganism ID\n3\tEvidence at protein level\t35780\t333\tGAPDH\tP46406\tG3P_RABIT\tGlyceraldehyde-3-phosphate dehydrogenase\tOryctolagus cuniculus\tUP000001811\tMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\t9986\n4\tEvidence at protein level\t69293\t607\tALB\tP02769\tALBU_BOVIN\tSerum albumin\tBos taurus\tUP000009136\tMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\t9913\n");

//        let mut buf: Vec<u8> = vec![];
//        v.to_csv_strict(&mut Cursor::new(&mut buf)).unwrap();
//        assert_eq!(String::from_utf8(buf).unwrap(), ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\n\n>sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4\nMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPF\nDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEP\nERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYY\nANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVA\nRLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKE\nCCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRR\nHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEK\nLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLIL\nNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLP\nDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVV\nSTQTALA");
//
//        let mut buf: Vec<u8> = vec![];
//        v.to_csv_lenient(&mut Cursor::new(&mut buf)).unwrap();
//        assert_eq!(String::from_utf8(buf).unwrap(), ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\n\n>sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4\nMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPF\nDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEP\nERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYY\nANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVA\nRLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKE\nCCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRR\nHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEK\nLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLIL\nNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLP\nDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVV\nSTQTALA");
//
//        // from_csv (valid, 2 items)
//        let y = RecordList::from_csv_string(&x).unwrap();
//        assert_eq!(y, RecordList::from_csv_strict(&mut Cursor::new(&x)).unwrap());
//        assert_eq!(y, RecordList::from_csv_lenient(&mut Cursor::new(&x)).unwrap());
//
//        // completeness check
//        for i in 0..2 {
//            incomplete_eq(&v[i], &y[i]);
//        }
//
//        // to_csv (empty)
//        let v: RecordList = vec![];
//        let x = v.to_csv_string().unwrap();
//        assert_eq!(x, "");
//
//        let mut buf: Vec<u8> = vec![];
//        v.to_csv_strict(&mut Cursor::new(&mut buf)).unwrap();
//        assert_eq!(String::from_utf8(buf).unwrap(), "");
//
//        let mut buf: Vec<u8> = vec![];
//        v.to_csv_lenient(&mut Cursor::new(&mut buf)).unwrap();
//        assert_eq!(String::from_utf8(buf).unwrap(), "");
//
//        // from_csv (empty)
//        let y = RecordList::from_csv_string(&x).unwrap();
//        assert_eq!(y, RecordList::from_csv_strict(&mut Cursor::new(&x)).unwrap());
//        assert_eq!(y, RecordList::from_csv_lenient(&mut Cursor::new(&x)).unwrap());
//        assert_eq!(y.len(), 0);
//
//        // to_csv (1 empty)
//        let v: RecordList = vec![Record::new()];
//        let x = v.to_csv_string().unwrap();
//        assert_eq!(x, ">sp||  OS= GN= PE=5 SV=0");
//
//        // to_csv (1 valid, 1 empty)
//        let v: RecordList = vec![gapdh(), Record::new()];
//        let x = v.to_csv_string().unwrap();
//        assert_eq!(x, ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\n\n>sp||  OS= GN= PE=5 SV=0");
    }
}
