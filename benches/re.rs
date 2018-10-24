#[macro_use]
extern crate bencher;
extern crate bdb;

use bencher::{black_box, Bencher};
use bdb::db::uniprot::low_level::*;

// HELPERS

#[inline(always)]
fn accession_regex_impl() -> bool {
    AccessionRegex::validate().is_match("A2BC19") &&
    AccessionRegex::validate().is_match("P12345") &&
    AccessionRegex::validate().is_match("A0A022YWF9")
}

// BENCHES

fn accession_regex(bench: &mut Bencher) {
    bench.iter(|| {
        black_box(accession_regex_impl())
    })
}

fn mnemonic_regex(bench: &mut Bencher) {
    // TODO(ahuszagh)   Implement...
    bench.iter(|| {})
}

fn gene_regex(bench: &mut Bencher) {
    // TODO(ahuszagh)   Implement...
    bench.iter(|| {})
}

fn aminoacid_regex(bench: &mut Bencher) {
    // TODO(ahuszagh)   Implement...
    bench.iter(|| {})
}

fn proteome_regex(bench: &mut Bencher) {
    // TODO(ahuszagh)   Implement...
    bench.iter(|| {})
}

fn taxonomy_regex(bench: &mut Bencher) {
    // TODO(ahuszagh)   Implement...
    bench.iter(|| {})
}

fn swissprot_header_regex(bench: &mut Bencher) {
    // TODO(ahuszagh)   Implement...
    bench.iter(|| {})
}

fn trembl_header_regex(bench: &mut Bencher) {
    // TODO(ahuszagh)   Implement...
    bench.iter(|| {})
}


benchmark_group!(
    benches,
    accession_regex,
    mnemonic_regex,
    gene_regex,
    aminoacid_regex,
    proteome_regex,
    taxonomy_regex,
    swissprot_header_regex,
    trembl_header_regex
);
benchmark_main!(benches);
