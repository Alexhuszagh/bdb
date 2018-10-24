#[macro_use]
extern crate bencher;
extern crate bdb;

use bencher::{black_box, Bencher};
use bdb::db::uniprot::*;
use bdb::traits::*;

// BENCHES

fn to_fasta(bench: &mut Bencher) {
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

    bench.iter(|| { black_box(gapdh.to_fasta_string()) })
}

fn from_fasta(bench: &mut Bencher) {
    let text = ">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus OX=9986 GN=GAPDH PE=1 SV=3\nMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKA\nENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIIS\nAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAIT\nATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSV\nVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIA\nLNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE";

    bench.iter(|| { black_box(Record::from_fasta_string(text)) })
}

benchmark_group!(
    benches,
    to_fasta,
    from_fasta
);
benchmark_main!(benches);
