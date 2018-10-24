#[macro_use]
extern crate bencher;
extern crate bdb;

use bencher::{black_box, Bencher};
use bdb::db::uniprot::*;
use bdb::traits::*;

// BENCHES

fn valid(bench: &mut Bencher) {
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

    bench.iter(|| { black_box(gapdh.is_valid()) })
}

fn invalid(bench: &mut Bencher) {
    let empty = Record::new();

    bench.iter(|| { black_box(!empty.is_valid()) })
}

benchmark_group!(
    benches,
    valid,
    invalid
);
benchmark_main!(benches);
