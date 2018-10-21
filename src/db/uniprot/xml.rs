//! Helper utilities for XML loading and saving.

use quick_xml::events::{BytesDecl, Event};
use quick_xml::{Writer};    // Reader,
use std::io::{Write};

use traits::*;
use util::ResultType;
use super::error::UniProtErrorKind;
use super::record::Record;
use super::record_list::RecordList;

// SHARED

/// Create CSV writer.
#[inline(always)]
fn new_writer<T: Write>(writer: T)
    -> Writer<T>
{
    Writer::new_with_indent(writer, b'\n', 1)
}

/// Write the XML declaration.
#[inline(always)]
fn write_declaration<T: Write>(writer: &mut Writer<T>) -> ResultType<()>
{
    const VERSION: &'static [u8] = b"1.0";
    const ENCODING: &'static [u8] = b"UTF-8";
    let decl = BytesDecl::new(VERSION, Some(ENCODING), None);
    match writer.write_event(Event::Decl(decl)) {
        Err(e)  => Err(From::from(UniProtErrorKind::Xml(e))),
        _       => Ok(()),
    }
}

// SIZE

/// Estimated size of the XML shared properties/attributes.
const XML_SHARED_SIZE: usize = 0;

/// Estimate the size of an XML record.
#[inline]
fn estimate_record_size(record: &Record) -> usize {
    // TODO(ahuszagh)
    //      Need to calculate the actual vocabulary size...
    const XML_RECORD_SIZE: usize = 0;
    XML_RECORD_SIZE +
        record.gene.len() +
        record.id.len() +
        record.mnemonic.len() +
        record.name.len() +
        record.organism.len() +
        record.sequence.len()
}

/// Estimate the size of an XML record list.
#[inline]
fn estimate_list_size(list: &RecordList) -> usize {
    list.iter().fold(0, |sum, x| sum + estimate_record_size(x))
}

// WRITER

/// Export record to XML.
#[allow(unused_variables)]      // TODO(ahuszagh) Remove
pub fn record_to_xml<T: Write>(record: &Record, writer: &mut T)
    -> ResultType<()>
{
    let mut writer = new_writer(writer);
    write_declaration(&mut writer)?;

    // TODO(ahuszagh)
    //  Implement, write the remaining data...
//    writer.write_record(&CSV_HEADER)?;
//    item_to_csv(&mut writer, record)?;
    Ok(())
}

// READER


// TRAITS

impl Xml for Record {
    #[inline(always)]
    fn estimate_xml_size(&self) -> usize {
        XML_SHARED_SIZE + estimate_record_size(self)
    }

    #[inline(always)]
    fn to_xml<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        record_to_xml(self, writer)
    }
}

impl Xml for RecordList {
    #[inline(always)]
    fn estimate_xml_size(&self) -> usize {
        XML_SHARED_SIZE + estimate_list_size(self)
    }

    #[inline(always)]
    fn to_xml<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        let writer = new_writer(writer);
        Ok(())
        //record_to_xml(self, writer, delimiter)
    }
}
