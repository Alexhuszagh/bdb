//! Client to post queries to the UniProt KB service.

use reqwest::{self, Response};
use std::io::{Cursor, Read};        // TODO: remove
use url;

use util::{ErrorType, ResultType};
use super::csv::{RecordIter, RecordIntoIter};
//use super::record_list::RecordList;

/// Host URL for the UniProt KB domain and path.
const HOST: &str = "https://www.uniprot.org:443/uniprot/";

/// Delimiter for accession number and mnemonic identifiers.
const DELIMITER: &str = " OR ";

/// Return type to iteratively produce records.
type IteratorType = RecordIntoIter<Response>;

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
    // TODO(ahuszagh)   Remove the following debug statements.
    //let text = response.text();
    println!("url = {:?}", url);
    //println!("body = {:?}", text);

    let mut iter = RecordIntoIter::new(response, b'\t');
    iter.parse_header()?;
    Ok(iter)
//    Err(From::from(""))
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn by_id_test() {
        // TODO(ahuszagh)       Restore
        //let record = by_id("P46406").unwrap().next().unwrap().unwrap();
        //println!("{:?}", record);
    }
    // TODO(ahuszagh)
    //      Implement these routines.
}
