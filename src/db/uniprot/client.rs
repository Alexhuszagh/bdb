//! Client to post queries to the UniProt KB service.

use reqwest::{self, Response};
use url;

use util::{ErrorType, ResultType};
use super::csv::CsvRecordIter;

/// Host URL for the UniProt KB domain and path.
const HOST: &str = "https://www.uniprot.org:443/uniprot/";

/// Delimiter for accession number and mnemonic identifiers.
const DELIMITER: &str = " OR ";

/// Return type to iteratively produce records.
type IteratorType = CsvRecordIter<Response>;

/// Request UniProt records by accession number.
///
/// * `ids` - Single accession number (eg. P46406).
#[inline(always)]
pub fn by_id(id: &str) -> ResultType<IteratorType> {
    by_id_impl(id)
}

/// Request UniProt records by accession numbers.
///
/// * `ids` - Slice of accession numbers (eg. [P46406]).
#[inline(always)]
pub fn by_id_list(ids: &[&str]) -> ResultType<IteratorType> {
    by_id_impl(&ids.join(DELIMITER))
}

/// Request UniProt records by mnemonic.
///
/// * `mnemonic` - Single mnemonic (eg. G3P_RABBIT).
#[inline(always)]
pub fn by_mnemonic(mnemonic: &str) -> ResultType<IteratorType> {
    by_mnemonic_impl(mnemonic)
}

/// Request UniProt records by mnemonics.
///
/// * `mnemonics` - Slice of mnemonics (eg. [G3P_RABBIT]).
#[inline(always)]
pub fn by_mnemonic_list(mnemonics: &[&str]) -> ResultType<IteratorType> {
    by_mnemonic_impl(&mnemonics.join(DELIMITER))
}

// PRIVATE
// -------

/// Helper function for requesting by accession number.
#[inline(always)]
fn by_id_impl(param: &str) -> ResultType<IteratorType> {
    call(&format!("id:{}", param))
}

/// Helper function for requesting by mnemonic.
#[inline(always)]
fn by_mnemonic_impl(param: &str) -> ResultType<IteratorType> {
    call(&format!("mnemonic:{}", param))
}

// Helper function for calling the UniProt KB service.
fn call(query: &str) -> ResultType<IteratorType> {
    // create our url with form-encoded parameters
    let params = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("sort", "score")
        .append_pair("desc", "")
        .append_pair("fil", "")
        .append_pair("force", "no")
        .append_pair("format", "tab")
        .append_pair("query", query)
        .append_pair("columns", "version(sequence),existence,mass,length,genes(PREFERRED),id,entry name,protein names,organism,proteome,sequence,organism-id")
        .finish();
    let url = format!("{}?{}", HOST, params);
    let response = reqwest::get(&url).map_err(|e| {
        Box::new(e) as ErrorType
    })?;

    Ok(CsvRecordIter::new(response, b'\t'))
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::evidence::ProteinEvidence;
    use super::super::record::Record;
    use super::super::record_list::RecordList;

    fn check_gapdh(record: &Record) {
        assert_eq!(record.sequence_version, 3);
        assert_eq!(record.protein_evidence, ProteinEvidence::ProteinLevel);
        assert_eq!(record.mass, 35780);
        assert_eq!(record.length, 333);
        assert_eq!(record.gene, "GAPDH");
        assert_eq!(record.id, "P46406");
        assert_eq!(record.mnemonic, "G3P_RABIT");
        assert_eq!(record.name, "Glyceraldehyde-3-phosphate dehydrogenase (GAPDH) (EC 1.2.1.12) (Peptidyl-cysteine S-nitrosylase GAPDH) (EC 2.6.99.-)");
        assert_eq!(record.organism, "Oryctolagus cuniculus (Rabbit)");
        assert_eq!(record.proteome, "UP000001811: Unplaced");
        assert_eq!(record.sequence, "MVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE");
        assert_eq!(record.taxonomy, "9986");
    }

    fn check_bsa(record: &Record) {
        assert_eq!(record.sequence_version, 4);
        assert_eq!(record.protein_evidence, ProteinEvidence::ProteinLevel);
        assert_eq!(record.mass, 69293);
        assert_eq!(record.length, 607);
        assert_eq!(record.gene, "ALB");
        assert_eq!(record.id, "P02769");
        assert_eq!(record.mnemonic, "ALBU_BOVIN");
        assert_eq!(record.name, "Serum albumin (BSA) (allergen Bos d 6)");
        assert_eq!(record.organism, "Bos taurus (Bovine)");
        assert_eq!(record.proteome, "UP000009136: Unplaced");
        assert_eq!(record.sequence, "MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA");
        assert_eq!(record.taxonomy, "9913");
    }

    #[test]
    #[ignore]
    fn by_id_test() {
        let record: Record = by_id("P46406").unwrap().next().unwrap().unwrap();
        check_gapdh(&record);
    }

    #[test]
    #[ignore]
    fn by_id_list_test() {
        let ids = ["P46406", "P02769"];
        let result: ResultType<RecordList> = by_id_list(&ids).unwrap().collect();
        let mut list = result.unwrap();
        list.sort();        // Ensure we get a stable ordering

        // Check properties.
        assert_eq!(list.len(), 2);
        check_gapdh(&list[0]);
        check_bsa(&list[1]);
    }

    #[test]
    #[ignore]
    fn by_mnemonic_test() {
        let record: Record = by_mnemonic("G3P_RABIT").unwrap().next().unwrap().unwrap();
        check_gapdh(&record);
    }

    #[test]
    #[ignore]
    fn by_mnemonic_list_test() {
        let mnemonics = ["G3P_RABIT", "ALBU_BOVIN"];
        let result: ResultType<RecordList> = by_mnemonic_list(&mnemonics).unwrap().collect();
        let mut list = result.unwrap();
        list.sort();        // Ensure we get a stable ordering

        // Check properties.
        assert_eq!(list.len(), 2);
        check_gapdh(&list[0]);
        check_bsa(&list[1]);
    }
}
