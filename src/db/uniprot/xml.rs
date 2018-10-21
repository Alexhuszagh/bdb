//! Helper utilities for XML loading and saving.

use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use std::io::{BufRead, Write};

use traits::*;
use util::ResultType;
use super::error::UniProtErrorKind;
use super::record::Record;
use super::record_list::RecordList;

// SHARED -- READER

/// Create CSV writer.
#[inline(always)]
fn new_reader<T: BufRead>(reader: T)
    -> Reader<T>
{
    Reader::from_reader(reader)
}

// TODO(ahuszagh)
//      Implement.

// SHARED -- WRITER

/// Create CSV writer.
#[inline(always)]
fn new_writer<T: Write>(writer: T)
    -> Writer<T>
{
    Writer::new(writer)
}

/// Create start element.
#[inline(always)]
fn new_start_element(bytes: &'static [u8]) -> BytesStart<'static> {
    BytesStart::borrowed(bytes, bytes.len())
}

/// Create text element.
#[inline(always)]
fn new_text_element<'a>(text: &'a str) -> BytesText<'a> {
    BytesText::from_plain_str(text)
}

/// Create end element.
#[inline(always)]
fn new_end_element(bytes: &'static [u8]) -> BytesEnd<'static> {
    BytesEnd::borrowed(bytes)
}

/// Macro to call `s.push_attribute(x)` for all x.
#[doc(hidden)]
#[macro_export]
macro_rules! push_attributes {
    // Base case, call `push_attribute`
    ($s:ident, $x:expr) => ($s.push_attribute($x));
    // `$x` followed by at least one `$y,`
    ($s:ident, $x:expr, $($y:expr),+) => ({
        $s.push_attribute($x);
        push_attributes!($s, $($y),+)
    });
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

/// Write an XML event.
#[inline(always)]
fn write_event<'a, T: Write>(writer: &mut Writer<T>, event: Event<'a>) -> ResultType<()>
{
    match writer.write_event(event) {
        Err(e)  => Err(From::from(UniProtErrorKind::Xml(e))),
        _       => Ok(()),
    }
}


/// Write the UniProt start element.
#[inline]
fn write_uniprot_start<T: Write>(writer: &mut Writer<T>) -> ResultType<()>
{
    // Attributes
    const XLMNS: (&'static[u8], &'static[u8]) = (b"xlmns", b"http://uniprot.org/uniprot");
    const XSI: (&'static[u8], &'static[u8]) = (b"xmlns:xsi", b"http://www.w3.org/2001/XMLSchema-instance");
    const LOCATION: (&'static[u8], &'static[u8]) = (b"xmlns:schemaLocation", b"http://uniprot.org/uniprot http://www.uniprot.org/support/docs/uniprot.xsd");

    // Create element
    let mut elem = new_start_element(b"uniprot");
    push_attributes!(elem, XLMNS, XSI, LOCATION);

    write_event(writer, Event::Start(elem))
}

/// Write the UniProt end element.
#[inline]
fn write_uniprot_end<T: Write>(writer: &mut Writer<T>) -> ResultType<()>
{
    let elem = new_end_element(b"uniprot");
    write_event(writer, Event::End(elem))
}

/// Write the entry element.
#[inline]
fn write_entry<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    write_entry_start(writer)?;
    write_id(record, writer)?;
    write_mnemonic(record, writer)?;
    write_protein(record, writer)?;
    write_gene(record, writer)?;
    write_organism(record, writer)?;
    write_protein_existence(record, writer)?;
    write_sequence(record, writer)?;

    write_entry_end(writer)
}

/// Write the entry start element.
#[inline]
fn write_entry_start<T: Write>(writer: &mut Writer<T>) -> ResultType<()>
{
    // Attributes
    const DATASET: (&'static[u8], &'static[u8]) = (b"dataset", b"Swiss-Prot");
    const CREATED: (&'static[u8], &'static[u8]) = (b"created", b"1995-11-01");

    // Create element
    let mut elem = new_start_element(b"entry");
    push_attributes!(elem, DATASET, CREATED);

    write_event(writer, Event::Start(elem))
}

/// Write the entry end element.
#[inline]
fn write_entry_end<T: Write>(writer: &mut Writer<T>) -> ResultType<()>
{
    let elem = new_end_element(b"entry");
    write_event(writer, Event::End(elem))
}

/// Write the accession element.
#[inline]
fn write_id<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    let start = new_start_element(b"accession");
    write_event(writer, Event::Start(start))?;

    let text = new_text_element(&record.id);
    write_event(writer, Event::Text(text))?;

    let end = new_end_element(b"accession");
    write_event(writer, Event::End(end))
}

/// Write the mnemonic element.
#[inline]
fn write_mnemonic<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    let start = new_start_element(b"name");
    write_event(writer, Event::Start(start))?;

    let text = new_text_element(&record.mnemonic);
    write_event(writer, Event::Text(text))?;

    let end = new_end_element(b"name");
    write_event(writer, Event::End(end))
}

/// Write the protein element.
#[inline]
fn write_protein<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    let elem = new_start_element(b"protein");
    write_event(writer, Event::Start(elem))?;

    write_recommended_name(record, writer)?;

    let elem = new_end_element(b"protein");
    write_event(writer, Event::End(elem))
}

/// Write the protein element.
#[inline]
fn write_recommended_name<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    let elem = new_start_element(b"recommendedName");
    write_event(writer, Event::Start(elem))?;

    write_full_name(record, writer)?;
    write_gene_name(record, writer)?;

    let elem = new_end_element(b"recommendedName");
    write_event(writer, Event::End(elem))
}

/// Write the name element.
#[inline]
fn write_full_name<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    let elem = new_start_element(b"fullName");
    write_event(writer, Event::Start(elem))?;

    let text = new_text_element(&record.name);
    write_event(writer, Event::Text(text))?;

    let elem = new_end_element(b"fullName");
    write_event(writer, Event::End(elem))
}

/// Write the gene element.
#[inline]
fn write_gene_name<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    let elem = new_start_element(b"shortName");
    write_event(writer, Event::Start(elem))?;

    let text = new_text_element(&record.gene);
    write_event(writer, Event::Text(text))?;

    let elem = new_end_element(b"shortName");
    write_event(writer, Event::End(elem))
}

/// Write the gene information element.
#[inline]
fn write_gene<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    let elem = new_start_element(b"gene");
    write_event(writer, Event::Start(elem))?;

    write_primary_name(record, writer)?;

    let elem = new_end_element(b"gene");
    write_event(writer, Event::End(elem))
}

/// Write the primary gene name element.
#[inline]
fn write_primary_name<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    // Attributes
    const TYPE: (&'static[u8], &'static[u8]) = (b"type", b"primary");

    let mut elem = new_start_element(b"name");
    push_attributes!(elem, TYPE);
    write_event(writer, Event::Start(elem))?;

    let text = new_text_element(&record.gene);
    write_event(writer, Event::Text(text))?;

    let elem = new_end_element(b"name");
    write_event(writer, Event::End(elem))
}

/// Write the organism information element.
#[inline]
fn write_organism<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    let elem = new_start_element(b"organism");
    write_event(writer, Event::Start(elem))?;

    // skip the common name since we can never guess....
    write_scientific_name(record, writer)?;
    write_taxonomy_id(record, writer)?;
    // skip the lineage section since we don't store that info

    let elem = new_end_element(b"organism");
    write_event(writer, Event::End(elem))
}

#[inline]
fn write_scientific_name<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    // Attributes
    const TYPE: (&'static[u8], &'static[u8]) = (b"type", b"scientific");

    let mut elem = new_start_element(b"name");
    push_attributes!(elem, TYPE);
    write_event(writer, Event::Start(elem))?;

    let text = new_text_element(&record.organism);
    write_event(writer, Event::Text(text))?;

    let elem = new_end_element(b"name");
    write_event(writer, Event::End(elem))
}

#[inline]
fn write_taxonomy_id<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    // Attributes
    const TYPE: (&'static[u8], &'static[u8]) = (b"type", b"NCBI Taxonomy");

    let mut elem = new_start_element(b"dbReference");
    let id: (&str, &str) = ("id", &record.taxonomy);
    push_attributes!(elem, TYPE, id);
    write_event(writer, Event::Start(elem))?;

    let elem = new_end_element(b"dbReference");
    write_event(writer, Event::End(elem))
}

#[inline]
fn write_protein_existence<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    let mut elem = new_start_element(b"proteinExistence");
    let typ: (&str, &str) = ("type", record.protein_evidence.xml_verbose());
    push_attributes!(elem, typ);
    write_event(writer, Event::Start(elem))?;

    let elem = new_end_element(b"proteinExistence");
    write_event(writer, Event::End(elem))
}


#[inline]
fn write_sequence<T: Write>(record: &Record, writer: &mut Writer<T>) -> ResultType<()>
{
    let length = record.length.to_string();
    let mass = record.mass.to_string();
    let version = record.sequence_version.to_string();
    let length: (&str, &str) = ("length", &length);
    let mass: (&str, &str) = ("mass", &mass);
    let version: (&str, &str) = ("version", &version);

    let mut elem = new_start_element(b"sequence");
    push_attributes!(elem, length, mass, version);
    write_event(writer, Event::Start(elem))?;

    let text = new_text_element(&record.sequence);
    write_event(writer, Event::Text(text))?;

    let elem = new_end_element(b"sequence");
    write_event(writer, Event::End(elem))
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

// READER

/// Import record from XML.
#[allow(unused_mut, unused_variables)]  // TODO(ahuszagh) Remove
pub fn record_from_xml<T: BufRead>(reader: &mut T)
    -> ResultType<Record>
{
    let mut reader = new_reader(reader);
    Err(From::from(""))
}
// TODO(ahuszagh)
//      Implement.

// READER -- DEFAULT

// READER -- STRICT

// READER -- LENIENT

// WRITER

/// Export record data to XML.
fn item_to_xml<T: Write>(record: &Record, writer: &mut Writer<T>)
    -> ResultType<()>
{
    write_uniprot_start(writer)?;
    write_entry(record, writer)?;
    write_uniprot_end(writer)
}

/// Export record to XML.
pub fn record_to_xml<T: Write>(record: &Record, writer: &mut T)
    -> ResultType<()>
{
    let mut writer = new_writer(writer);
    write_declaration(&mut writer)?;
    item_to_xml(record, &mut writer)
}

// WRITER -- DEFAULT

/// Default exporter from a non-owning iterator to XML.
#[allow(dead_code)]     // TODO(ahuszagh)   Remove
pub fn reference_iterator_to_xml<'a, Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    let mut writer = new_writer(writer);
    write_declaration(&mut writer)?;
    write_uniprot_start(&mut writer)?;

    for record in iter {
        write_entry(record, &mut writer)?;
    }

    write_uniprot_end(&mut writer)
}

/// Default exporter from an owning iterator to XML.
#[allow(dead_code)]     // TODO(ahuszagh)   Remove
pub fn value_iterator_to_xml<Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    let mut writer = new_writer(writer);
    write_declaration(&mut writer)?;
    write_uniprot_start(&mut writer)?;

    for record in iter {
        write_entry(&record?, &mut writer)?;
    }

    write_uniprot_end(&mut writer)
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to XML.
#[allow(dead_code)]     // TODO(ahuszagh)   Remove
pub fn reference_iterator_to_xml_strict<'a, Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    let mut writer = new_writer(writer);
    write_declaration(&mut writer)?;
    write_uniprot_start(&mut writer)?;

    for record in iter {
        if record.is_valid() {
            write_entry(record, &mut writer)?;
        } else {
            return Err(From::from(UniProtErrorKind::InvalidRecord));
        }
    }

    write_uniprot_end(&mut writer)
}

/// Strict exporter from an owning iterator to XML.
#[allow(dead_code)]     // TODO(ahuszagh)   Remove
pub fn value_iterator_to_xml_strict<Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    let mut writer = new_writer(writer);
    write_declaration(&mut writer)?;
    write_uniprot_start(&mut writer)?;

    for result in iter {
        let record = result?;
        if record.is_valid() {
            write_entry(&record, &mut writer)?;
        } else {
            return Err(From::from(UniProtErrorKind::InvalidRecord));
        }
    }

    write_uniprot_end(&mut writer)
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to XML.
#[allow(dead_code)]     // TODO(ahuszagh)   Remove
pub fn reference_iterator_to_xml_lenient<'a, Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    let mut writer = new_writer(writer);
    write_declaration(&mut writer)?;
    write_uniprot_start(&mut writer)?;

    for record in iter {
        if record.is_valid() {
            write_entry(record, &mut writer)?;
        }
    }

    write_uniprot_end(&mut writer)
}

/// Lenient exporter from an owning iterator to XML.
#[allow(dead_code)]     // TODO(ahuszagh)   Remove
pub fn value_iterator_to_xml_lenient<Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    let mut writer = new_writer(writer);
    write_declaration(&mut writer)?;
    write_uniprot_start(&mut writer)?;

    for result in iter {
        let record = result?;
        if record.is_valid() {
            write_entry(&record, &mut writer)?;
        }
    }

    write_uniprot_end(&mut writer)
}

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

    #[inline(always)]
    fn from_xml<T: BufRead>(reader: &mut T) -> ResultType<Self> {
        record_from_xml(reader)
    }
}

impl Xml for RecordList {
    #[inline(always)]
    fn estimate_xml_size(&self) -> usize {
        XML_SHARED_SIZE + estimate_list_size(self)
    }

    #[inline(always)]
    fn to_xml<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        reference_iterator_to_xml(self.iter(), writer)
    }

    #[inline(always)]
    #[allow(unused_mut, unused_variables)]  // TODO(ahuszagh) Remove
    fn from_xml<T: BufRead>(reader: &mut T) -> ResultType<Self> {
        //iterator_from_xml(reader).collect()
        Err(From::from(""))
    }
}


// TESTS
// -----

#[cfg(test)]
mod tests {
    // TODO(ahuszagh)
    //  Implement...
}
