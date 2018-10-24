#[macro_use]
extern crate bencher;
extern crate bdb;

use bencher::{black_box, Bencher};
use bdb::db::uniprot::low_level::*;

// HELPERS

#[inline(always)]
fn accession_regex_impl() -> bool {
    type T = AccessionRegex;
    T::validate().is_match("A2BC19") &&
    T::validate().is_match("P12345") &&
    T::validate().is_match("A0A022YWF9") &&
    !T::validate().is_match("2BC19") &&
    !T::validate().is_match("A2BC1") &&
    !T::validate().is_match("0A022YWF9") &&
    !T::validate().is_match("A0A022YWF")
}

#[inline(always)]
fn mnemonic_regex_impl() -> bool {
    type T = MnemonicRegex;
    T::validate().is_match("G3P_RABIT") &&
    T::validate().is_match("1433B_HUMAN") &&
    T::validate().is_match("ENO_ACTSZ") &&
    T::validate().is_match("A0A024R832_HUMAN") &&
    !T::validate().is_match("G3P_RABITX") &&
    !T::validate().is_match("1433B_HUMANX") &&
    !T::validate().is_match("A0A024R832_HUMANX")
}

#[inline(always)]
fn gene_regex_impl() -> bool {
    type T = GeneRegex;
    T::validate().is_match("ND3") &&
    T::validate().is_match("KIF5B-RET(NM_020975)_K15;R12") &&
    T::validate().is_match("TRA@") &&
    T::validate().is_match("HLA-DRB5") &&
    T::validate().is_match("NOD2/CARD15") &&
    T::validate().is_match("Hosa(Biaka)-T2R50") &&
    T::validate().is_match("cytb") &&
    T::validate().is_match("dopamine D4 receptor/ DRD4") &&
    !T::validate().is_match("ND3[") &&
    !T::validate().is_match("ND3`")
}

#[inline(always)]
fn aminoacid_regex_impl() -> bool {
    type T = AminoacidRegex;
    T::validate().is_match(b"SAMPLER") &&
    T::validate().is_match(b"sampler") &&
    T::validate().is_match(b"sAmpLer") &&
    T::validate().is_match(b"USAMPLER") &&
    !T::validate().is_match(b"ORANGE") &&
    !T::validate().is_match(b"oRANGE")
}

#[inline(always)]
fn proteome_regex_impl() -> bool {
    type T = ProteomeRegex;
    T::validate().is_match("UP000001811") &&
    T::validate().is_match("UP000001114") &&
    !T::validate().is_match("UX000001811") &&
    !T::validate().is_match("UPX00001114") &&
    !T::validate().is_match("UP0000018113") &&
    !T::validate().is_match("UP0000011144") &&
    T::validate().is_match("UP000001811: Unplaced") &&
    T::validate().is_match("UP000001114: Chromosome")
}

#[inline(always)]
fn taxonomy_regex_impl() -> bool {
    type T = TaxonomyRegex;
    T::validate().is_match("9606") &&
    T::validate().is_match("731") &&
    !T::validate().is_match("965X") &&
    !T::validate().is_match("965 ") &&
    !T::validate().is_match(" 965") &&
    !T::validate().is_match("X965")
}

#[inline(always)]
fn swissprot_header_regex_impl() -> bool {
    type T = SwissProtHeaderRegex;
    T::validate().is_match(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3") &&
    T::validate().is_match(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3\n") &&
    T::validate().is_match(">sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4") &&
    T::validate().is_match(">sp|P02769|ALBU_BOVIN Serum albumin OS=Bos taurus GN=ALB PE=1 SV=4\n") &&
    T::validate().is_match(">sp|Q9N2K0|ENH1_HUMAN HERV-H_2q24.3 provirus ancestral Env polyprotein OS=Homo sapiens OX=9606 PE=2 SV=1") &&
    T::validate().is_match(">sp|Q6ZN92|DUTL_HUMAN Putative inactive deoxyuridine 5\'-triphosphate nucleotidohydrolase-like protein FLJ16323 OS=Homo sapiens OX=9606 PE=5 SV=1") &&
    !T::validate().is_match(">up|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3") &&
    !T::validate().is_match(">sp|PX6406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3") &&
    !T::validate().is_match(">sp|P46406|G3P_RABITS Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=3") &&
    !T::validate().is_match(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1X SV=3") &&
    !T::validate().is_match(">sp|P46406|G3P_RABIT Glyceraldehyde-3-phosphate dehydrogenase OS=Oryctolagus cuniculus GN=GAPDH PE=1 SV=X3")
}

#[inline(always)]
fn trembl_header_regex_impl() -> bool {
    type T = TrEMBLHeaderRegex;
    T::validate().is_match(">tr|A0A2U8RNL1|A0A2U8RNL1_HUMAN MHC class II antigen (Fragment) OS=Homo sapiens OX=9606 GN=DPB1 PE=4 SV=1") &&
    T::validate().is_match(">tr|O14861|O14861_HUMAN Zinc finger protein (Fragment) OS=Homo sapiens OX=9606 PE=2 SV=1") &&
    T::validate().is_match(">tr|Q53FP0|Q53FP0_HUMAN Pyridoxine 5\'-phosphate oxidase variant (Fragment) OS=Homo sapiens OX=9606 PE=2 SV=1") &&
    T::validate().is_match(">tr|B7ZKX2|B7ZKX2_HUMAN Uncharacterized protein OS=Homo sapiens OX=9606 PE=2 SV=1") &&
    T::validate().is_match(">tr|Q59FB0|Q59FB0_HUMAN PREDICTED: KRAB domain only 2 variant (Fragment) OS=Homo sapiens OX=9606 PE=2 SV=1") &&
    !T::validate().is_match(">ur|A0A2U8RNL1|A0A2U8RNL1_HUMAN MHC class II antigen (Fragment) OS=Homo sapiens OX=9606 GN=DPB1 PE=4 SV=1") &&
    !T::validate().is_match(">tr|AXA2U8RNL1|A0A2U8RNL1_HUMAN MHC class II antigen (Fragment) OS=Homo sapiens OX=9606 GN=DPB1 PE=4 SV=1") &&
    !T::validate().is_match(">tr|A0A2U8RNL1|A0A2U8RNL1_HUMANS MHC class II antigen (Fragment) OS=Homo sapiens OX=9606 GN=DPB1 PE=4 SV=1") &&
    !T::validate().is_match(">tr|A0A2U8RNL1|A0A2U8RNL1_HUMAN MHC class II antigen (Fragment) OS=Homo sapiens OX=9606 GN=DPB1 PE=4X SV=1") &&
    !T::validate().is_match(">tr|A0A2U8RNL1|A0A2U8RNL1_HUMAN MHC class II antigen (Fragment) OS=Homo sapiens OX=9606 GN=DPB1 PE=4 SV=X1")
}

// BENCHES

fn accession_regex(bench: &mut Bencher) {
    bench.iter(|| { black_box(accession_regex_impl()) })
}

fn mnemonic_regex(bench: &mut Bencher) {
    bench.iter(|| { black_box(mnemonic_regex_impl()) })
}

fn gene_regex(bench: &mut Bencher) {
    bench.iter(|| { black_box(gene_regex_impl()) })
}

fn aminoacid_regex(bench: &mut Bencher) {
    bench.iter(|| { black_box(aminoacid_regex_impl()) })
}

fn proteome_regex(bench: &mut Bencher) {
    bench.iter(|| { black_box(proteome_regex_impl()) })
}

fn taxonomy_regex(bench: &mut Bencher) {
    bench.iter(|| { black_box(taxonomy_regex_impl()) })
}

fn swissprot_header_regex(bench: &mut Bencher) {
    bench.iter(|| { black_box(swissprot_header_regex_impl()) })
}

fn trembl_header_regex(bench: &mut Bencher) {
    bench.iter(|| { black_box(trembl_header_regex_impl()) })
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
