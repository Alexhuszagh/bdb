/**
 *  UniProt
 *  -------
 *
 *  Record definitions for the UniProt KB service.
 *
 *  :copyright: (c) 2018 Alex Huszagh.
 *  :license: MIT, see LICENSE.md for more details.
 */

use ::ref_slice::ref_slice;

use tbt::{Tbt};       // TbtCollection
//use text::{Text, TextCollection};

// UNIPROT
// -------

impl Tbt for Record {
    /**
     *  \brief Export UniProt record to TBT.
     */
    fn to_tbt(&self) -> ResultType<String> {
        _slice_to_tbt(ref_slice(&self))
    }

    /**
     *  \brief Import UniProt record from a TBT row.
     */
    fn from_tbt(text: &str) -> ResultType<Record> {
        // TODO(ahuszagh) Implement...
        // 1. Need to find only the first 2 lines.
        // 2. Need to call the deserializer.
        // 3. Need to yank just the first item.

        //_text_to_list(text)[0];
        let _text = text;
        Err(From::from(""))
    }
}

impl Tbt for RecordList {
    /**
     *  \brief Export UniProt records to TBT.
     */
    fn to_tbt(&self) -> ResultType<String> {
        _slice_to_tbt(&self[..])
    }

    /**
     *  \brief Import UniProt records from TBT.
     */
    fn from_tbt(text: &str) -> ResultType<RecordList> {
        // TODO(ahuszagh) Implement...
        // 1. Need to call the deserializer.
        // 2. Return values.

        //_text_to_list(text)[0];
        let _text = text;
        Err(From::from(""))
    }
}

// PRIVATE
// -------

// RECORD(S) TO TBT

/**
 *  \brief Export the header columns to TBT.
 */
fn _header_to_row() -> Vec<&'static str> {
    vec![
        "Sequence version",         // sequence_version
        "Protein existence",        // protein_evidence
        "Mass",                     // mass
        "Length",                   // length
        "Gene names  (primary )",   // gene
        "Entry",                    // id
        "Entry name",               // mnemonic
        "Protein names",            // name
        "Organism",                 // organism
        "Proteomes",                // proteome
        "Sequence",                 // sequence
        "Organism ID",              // taxonomy
    ]
}

/**
 *  \brief Convert a record to vector of strings to serialize into TBT.
 */
fn _record_to_row(record: &Record) -> Vec<String> {
    vec![
        nonzero_to_string!(record.sequence_version),
        String::from(match record.protein_evidence {
            ProteinEvidence::Unknown    => "",
            _                           => protein_evidence_verbose(record.protein_evidence),
        }),
        nonzero_to_string!(record.mass),
        nonzero_to_string!(record.length),
        record.gene.clone(),
        record.id.clone(),
        record.mnemonic.clone(),
        record.name.clone(),
        record.organism.clone(),
        record.proteome.clone(),
        record.sequence.clone(),
        record.taxonomy.clone(),
    ]
}


/**
 *  \brief Convert a slice of records into TBT.
 */
fn _slice_to_tbt(records: &[Record]) -> ResultType<String> {
    // Create our custom writer.
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::Necessary)
        .flexible(false)
        .from_writer(vec![]);

    // Serialize the header to TBT.
    writer.write_record(&_header_to_row())?;

    // Serialize each row to TBT.
    for record in records {
        writer.write_record(&_record_to_row(&record))?;
    }

    // Return a string from the writer bytes.
    Ok(String::from_utf8(writer.into_inner()?)?)
}

// RECORD(S) FROM TBT


// TODO(ahuszagh)
//      Likely remove
//      Need to implement other logic for conversion from TBT.
///**
// *  \brief Convert tab-delimited text records to a UniProt record list.
// */
//#[allow(unused_variables)]
//fn _text_to_list<'a>(text: &str) -> Result<RecordList, &'a str> {
//    // TODO(ahuszagh)
//    //  Implement the slice to text code.
//
//    Err("Not yet implemented...")
//}

// CONNECTION
// ----------

/**
 *  \brief Module to fetch records using the Uniprot KB service.
 */
pub mod fetch {

//    // CONSTANTS
//    // ---------
//
//    const HOST: &str = "https://www.uniprot.org:443/uniprot/";
//
//    // ALIAS
//    // -----
//
//    use std::error::Error;
//    use reqwest;
//    use url::form_urlencoded;
//
//    use alias::ResultType;
//    use tbt::{Tbt};
//
//    use super::RecordList;
//
//    // API
//    // ---

//    /**
//     *  \brief Request UniProt records by accession number.
//     *
//     *  \param ids      Single accession number (eg. P46406)
//     */
//    pub fn by_id(id: &str) -> ResultType<RecordList> {
//        _by_id(id)
//    }
//
//    /**
//     *  \brief Request UniProt records by accession numbers.
//     *
//     *  \param ids      Slice of accession numbers (eg. [P46406])
//     */
//    pub fn by_id_list(ids: &[&str]) -> ResultType<RecordList> {
//        _by_id(&ids.join(" OR "))
//    }
//
//    /**
//     *  \brief Request UniProt records by mnemonic.
//     *
//     *  \param ids      Single mnemonic (eg. G3P_RABBIT)
//     */
//    pub fn by_mnemonic(mnemonic: &str) -> ResultType<RecordList> {
//        _by_mnemonic(mnemonic)
//    }
//
//    /**
//     *  \brief Request UniProt records by mnemonics.
//     *
//     *  \param ids      Slice of mnemonics (eg. [G3P_RABBIT])
//     */
//    pub fn by_mnemonic_list(ids: &[&str]) -> ResultType<RecordList> {
//        _by_mnemonic(&ids.join(" OR "))
//    }

    // PRIVATE
    // -------

//    // Helper function for calling the UniProt KB service.
//    #[allow(unused_variables)]
//    fn _call(query: &str) -> ResultType<RecordList> {
//        // create our url with form-encoded parameters
//        let params = form_urlencoded::Serializer::new(String::new())
//            .append_pair("sort", "score")
//            .append_pair("desc", "")
//            .append_pair("fil", "")
//            .append_pair("force", "no")
//            .append_pair("format", "tab")
//            .append_pair("query", query)
//            .append_pair("columns", "version(sequence),existence,mass,length,genes(PREFERRED),id,entry name,protein names,organism,proteome,sequence,organism-id")
//            .finish();
//        let url = format!("{}?{}", HOST, params);
//        let body = _url_to_body(&url)?;
//        // TODO(ahuszagh)   Remove the following debug statements.
//        println!("url = {:?}", url);
//        println!("body = {:?}", body);
//
//        RecordList::from_tbt(&body)
//    }
//
//    // Helper functions to convert URL to UniProt body.
//    fn _url_to_body_impl(url: &str) -> Result<String, reqwest::Error> {
//        reqwest::get(url)?.text()
//    }
//
//    fn _url_to_body(url: &str) -> ResultType<String> {
//        _url_to_body_impl(url).map_err(|e| {
//            Box::new(e) as Box<Error>
//        })
//    }
//
//    // Helper function for requesting by accession number.
//    fn _by_id(id: &str) -> ResultType<RecordList> {
//        _call(&format!("id:{}", id))
//    }
//
//    // Helper function for requesting by mnemonic.
//    fn _by_mnemonic(mnemonic: &str)-> ResultType<RecordList> {
//        _call(&format!("mnemonic:{}", mnemonic))
//    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    // RECORD

    #[test]
    fn tbt_gapdh() {
        let g = gapdh();

        // to_tbt
        let x = g.to_tbt().unwrap();
        assert_eq!(x, "Sequence version\tProtein existence\tMass\tLength\tGene names  (primary )\tEntry\tEntry name\tProtein names\tOrganism\tProteomes\tSequence\tOrganism ID\n3\tEvidence at protein level\t35780\t333\tGAPDH\tP46406\tG3P_RABIT\tGlyceraldehyde-3-phosphate dehydrogenase\tOryctolagus cuniculus\tUP000001811\tMVKVGVNGFGRIGRLVTRAAFNSGKVDVVAINDPFIDLHYMVYMFQYDSTHGKFHGTVKAENGKLVINGKAITIFQERDPANIKWGDAGAEYVVESTGVFTTMEKAGAHLKGGAKRVIISAPSADAPMFVMGVNHEKYDNSLKIVSNASCTTNCLAPLAKVIHDHFGIVEGLMTTVHAITATQKTVDGPSGKLWRDGRGAAQNIIPASTGAAKAVGKVIPELNGKLTGMAFRVPTPNVSVVDLTCRLEKAAKYDDIKKVVKQASEGPLKGILGYTEDQVVSCDFNSATHSSTFDAGAGIALNDHFVKLISWYDNEFGYSNRVVDLMVHMASKE\t9986\n");

        // TODO(ahuszagh) Implement deserializer
    }

    #[test]
    fn tbt_bsa() {
        let b = bsa();

        // to_tbt
        let x = b.to_tbt().unwrap();
        assert_eq!(x, "Sequence version\tProtein existence\tMass\tLength\tGene names  (primary )\tEntry\tEntry name\tProtein names\tOrganism\tProteomes\tSequence\tOrganism ID\n4\tEvidence at protein level\t69293\t607\tALB\tP02769\tALBU_BOVIN\tSerum albumin\tBos taurus\tUP000009136\tMKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA\t9913\n");

        // TODO(ahuszagh) Implement deserializer
    }

    // LIST

    #[test]
    fn tbt_list() {
        // TODO(ahuszagh) Implement the TBT serializer test for lists.
    }

    // FETCH
    // TODO(ahuzagh)
    //      Need to implement the fetch tests here.

    use super::fetch;

    #[test]
    fn by_id() {
        fetch::by_id("P46406");
        // TODO(ahuszagh) implement
    }

    // by_id_list
    // by_mnemonic
    // by_mnemonic_list
}
