//! Client to post queries to the UniProt KB service.

use reqwest;
use url;

use util::{ErrorType, ResultType};
use super::csv::RecordIter;
//use super::record_list::RecordList;

/// Host URL for the UniProt KB domain and path.
const HOST: &str = "https://www.uniprot.org:443/uniprot/";

/// Delimiter for accession number and mnemonic identifiers.
const DELIMITER: &str = " OR ";

// TODO(ahuszagh)
//      Restore the streaming capabilities...
///// Request UniProt records by accession number.
/////
///// * `ids` - Single accession number (eg. P46406).
//#[inline(always)]
//pub fn by_id(id: &str) -> ResultType<RecordIter> {
//    by_id_impl(id)
//}
//
//
///// Request UniProt records by accession numbers.
/////
///// * `ids` - Slice of accession numbers (eg. [P46406]).
//#[inline(always)]
//pub fn by_id_list(ids: &[&str]) -> ResultType<RecordIter> {
//    by_id_impl(&ids.join(DELIMITER))
//}
//
//
///// Request UniProt records by mnemonic.
/////
///// * `mnemonic` - Single mnemonic (eg. G3P_RABBIT).
//#[inline(always)]
//pub fn by_mnemonic(mnemonic: &str) -> ResultType<RecordIter> {
//    by_mnemonic_impl(mnemonic)
//}
//
//
///// Request UniProt records by mnemonics.
/////
///// * `mnemonics` - Slice of mnemonics (eg. [G3P_RABBIT]).
//#[inline(always)]
//pub fn by_mnemonic_list(mnemonics: &[&str]) -> ResultType<RecordIter> {
//    by_mnemonic_impl(&mnemonics.join(DELIMITER))
//}
//
//// PRIVATE
//// -------
//
///// Helper function for requesting by accession number.
//#[inline(always)]
//fn by_id_impl(param: &str) -> ResultType<RecordIter> {
//    call(&format!("id:{}", param))
//}
//
///// Helper function for requesting by mnemonic.
//#[inline(always)]
//fn by_mnemonic_impl(param: &str) -> ResultType<RecordIter> {
//    call(&format!("mnemonic:{}", param))
//}
//
///// Helper function to convert URL to UniProt body.
//#[inline(always)]
//fn url_to_body_impl(url: &str) -> Result<reqwest::Response, reqwest::Error> {
//    reqwest::get(url)?
//}
//
///// Helper function to map `reqwest::Error` to `ErrorType`.
//#[inline(always)]
//fn url_to_body(url: &str) -> ResultType<String> {
//    url_to_body_impl(url).map_err(|e| {
//        Box::new(e) as ErrorType
//    })
//}
//
//// Helper function for calling the UniProt KB service.
//fn call(query: &str) -> ResultType<RecordIter> {
//    // create our url with form-encoded parameters
//    let params = url::form_urlencoded::Serializer::new(String::new())
//        .append_pair("sort", "score")
//        .append_pair("desc", "")
//        .append_pair("fil", "")
//        .append_pair("force", "no")
//        .append_pair("format", "tab")
//        .append_pair("query", query)
//        .append_pair("columns", "version(sequence),existence,mass,length,genes(PREFERRED),id,entry name,protein names,organism,proteome,sequence,organism-id")
//        .finish();
//    let url = format!("{}?{}", HOST, params);
//    // TODO(ahuszagh)
//    //  Can we convert the body to a stream without async??
//    let body = url_to_body(&url)?;
//    // TODO(ahuszagh)   Remove the following debug statements.
//    println!("url = {:?}", url);
//    println!("body = {:?}", body);
//
//    // TODO(ahuszagh)       Remove
//    Ok(RecordList::new())
//    //RecordList::from_csv_string(&body)
//}

// TESTS
// -----

#[cfg(test)]
mod tests {
    // TODO(ahuszagh)
    //      Implement these routines.
}
