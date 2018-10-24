#[macro_use]
extern crate bencher;
extern crate bdb;

use bencher::{black_box, Bencher};
use bdb::db::uniprot::*;
use bdb::traits::*;

// BENCHES

fn to_csv(bench: &mut Bencher) {
    let gapdh = Record {
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
        sequence: b"MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE".to_vec(),
        taxonomy: String::from("9986"),
        reviewed: true,
    };

    bench.iter(|| { black_box(gapdh.to_csv_string(b'\t')) })
}

fn from_csv(bench: &mut Bencher) {
    let text = "Version (sequence)\tProtein existence\tMass\tLength\tGene names  (primary )\tEntry\tEntry name\tProtein names\tOrganism\tProteomes\tSequence\tOrganism ID\tStatus\n3\tEvidence at protein level\t35,780\t333\tGAPDH\tP46406\tG3P_RABIT\tGlyceraldehyde-3-phosphate dehydrogenase\tOryctolagus cuniculus\tUP000001811\tMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\t9986\treviewed\n";

    bench.iter(|| { black_box(Record::from_csv_string(text, b'\t')) })
}

benchmark_group!(
    benches,
    to_csv,
    from_csv
);
benchmark_main!(benches);
