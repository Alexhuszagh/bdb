//! Helper utilities for XML loading and saving.
//!
//! This module, especially the implementation of the reader, is quite
//! difficult to understand, due to the low-level optimizations and the
//! SAX-like API present for the pull XML parser. The module is copiously
//! commented to try to facilitate maintainability.

use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Writer};
use std::io::{BufRead, Write};

use traits::*;
use util::{ErrorKind, ResultType, XmlReader};
use super::record::Record;
use super::record_list::RecordList;

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
        Err(e)  => Err(From::from(ErrorKind::Xml(e))),
        _       => Ok(()),
    }
}

/// Write an XML event.
#[inline(always)]
fn write_event<'a, T: Write>(writer: &mut Writer<T>, event: Event<'a>) -> ResultType<()>
{
    match writer.write_event(event) {
        Err(e)  => Err(From::from(ErrorKind::Xml(e))),
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

// XML RECORD ITER

/// Macro to quickly return None or an Error inside an Option<Result<>>;
macro_rules! try_opterr {
    ($e:expr) => ({
         match $e? {
            Err(e)  => return Some(Err(e)),
            _ => (),
        }
    });
}

pub struct XmlRecordIter<T: BufRead> {
    reader: XmlReader<T>,
}

impl<T: BufRead> XmlRecordIter<T> {
    /// Create new XmlRecordIter from a buffered reader.
    #[inline]
    pub fn new(reader: T) -> Self {
        XmlRecordIter {
            reader: XmlReader::new(reader),
        }
    }

    /// Enter the entry element.
    #[inline]
    fn enter_entry(&mut self) -> Option<ResultType<()>> {
        try_opterr!(self.reader.seek_start(b"entry", 1));
        Some(Ok(()))
    }

    /// Leave the entry element.
    #[inline]
    fn leave_entry(&mut self) -> Option<ResultType<()>> {
        try_opterr!(self.reader.seek_end(b"entry", 1));
        Some(Ok(()))
    }

    /// Read the accession number.
    #[inline]
    fn read_accession(&mut self, record: &mut Record) -> Option<ResultType<()>> {
        try_opterr!(self.reader.seek_start(b"accession", 2));

        match self.reader.read_text(b"accession") {
            Err(e)  => return Some(Err(e)),
            Ok(v)   => record.id = v,
        }

        Some(Ok(()))
    }

    /// Read the mnemonic identifier.
    #[inline]
    fn read_mnemonic(&mut self, record: &mut Record) -> Option<ResultType<()>> {
        try_opterr!(self.reader.seek_start(b"name", 2));

        match self.reader.read_text(b"name") {
            Err(e)  => return Some(Err(e)),
            Ok(v)   => record.mnemonic = v,
        }

        Some(Ok(()))
    }

    /// Read the protein name.
    #[inline]
    fn read_protein(&mut self, record: &mut Record) -> Option<ResultType<()>> {
        // Ensure we get to the recommendedName, since "alternativeName"
        // also has the same attributes.
        try_opterr!(self.reader.seek_start(b"recommendedName", 3));

        // Read the protein name
        try_opterr!(self.reader.seek_start(b"fullName", 4));
        match self.reader.read_text(b"fullName") {
            Err(e)  => return Some(Err(e)),
            Ok(v)   => record.name = v,
        }

        try_opterr!(self.reader.seek_end(b"recommendedName", 3));

        Some(Ok(()))
    }

    /// Read the text from the name element.
    #[inline]
    fn read_gene_impl(&mut self, record: &mut Record) -> Option<ResultType<()>> {
        match self.reader.read_text(b"name") {
            Err(e)  => return Some(Err(e)),
            Ok(v)   => record.gene = v,
        }

        Some(Ok(()))
    }

    /// Read the gene name.
    #[inline]
    fn read_gene(&mut self, record: &mut Record) -> Option<ResultType<()>> {
        try_opterr!(self.reader.seek_start(b"gene", 2));

        //  Gene XML format.
        //      <gene>
        //      <name type="primary">GAPDH</name>
        //      <name type="synonym">GAPD</name>
        //      </gene>

        // Callback to determine if we're reading the primary gene name.
        fn is_gene<'a>(event: BytesStart<'a>, _: &mut Record)
            -> Option<ResultType<bool>>
        {
            for result in event.attributes() {
                let attribute = match result {
                    Err(e) => return Some(Err(From::from(ErrorKind::Xml(e)))),
                    Ok(v)  => v,
                };
                if attribute.key == b"type" && &*attribute.value == b"primary" {
                    return Some(Ok(true));
                }
            }
            Some(Ok(false))
        }

        // Here we invoke the actual callback iteratively until we find the element.
        loop {
            match self.reader.seek_start_callback(b"name", 3, record, is_gene)? {
                Err(e)  => return Some(Err(e)),
                Ok(v)   => {
                    if v {
                        try_opterr!(self.read_gene_impl(record));
                        return Some(Ok(()));
                    }
                }
            }
        }
    }

    /// Read the taxonomy.
    #[inline]
    fn read_taxonomy(&mut self, record: &mut Record) -> Option<ResultType<()>> {
        // Callback to parse the taxonomy information.
        fn parse_taxonomy<'a>(event: BytesStart<'a>, record: &mut Record)
            -> Option<ResultType<bool>>
        {
            for result in event.attributes() {
                let attribute = match result {
                    Err(e) => return Some(Err(From::from(ErrorKind::Xml(e)))),
                    Ok(v)  => v,
                };
                if attribute.key == b"type" && &*attribute.value != b"NCBI Taxonomy" {
                    // If the dbReference type is not NCBI Taxonomy, quit early
                    return Some(Ok(false));
                } else if attribute.key == b"id" {
                    // Parse the taxonomic identifier.
                    record.taxonomy = match String::from_utf8(attribute.value.to_vec()) {
                        Err(e) => return Some(Err(From::from(ErrorKind::FromUtf8(e)))),
                        Ok(v)  => v,
                    };
                    return Some(Ok(true));
                }
            }
            Some(Ok(false))
        }

        // Invoker our callback
        Some(match self.reader.seek_start_callback(b"dbReference", 3, record, parse_taxonomy)? {
            Err(e)  => Err(e),
            Ok(_)   => Ok(()),
        })
    }

    /// Read the text from the name element.
    #[inline]
    fn read_organism_impl(&mut self, record: &mut Record) -> Option<ResultType<()>> {
        match self.reader.read_text(b"name") {
            Err(e)  => return Some(Err(e)),
            Ok(v)   => record.organism = v,
        }

        Some(Ok(()))
    }

    /// Read the organism name.
    #[inline]
    fn read_organism(&mut self, record: &mut Record) -> Option<ResultType<()>> {
        try_opterr!(self.reader.seek_start(b"organism", 2));

        //  Organism XML format.
        //        <organism>
        //        <name type="scientific">Oryctolagus cuniculus</name>
        //        <name type="common">Rabbit</name>
        //        <dbReference type="NCBI Taxonomy" id="9986"/>
        //        ...
        //        </organism>

        // Callback to determine if we're reading the scientific name.
        fn is_organism<'a>(event: BytesStart<'a>, _: &mut Record)
            -> Option<ResultType<bool>>
        {
            for result in event.attributes() {
                let attribute = match result {
                    Err(e) => return Some(Err(From::from(ErrorKind::Xml(e)))),
                    Ok(v)  => v,
                };
                if attribute.key == b"type" && &*attribute.value == b"scientific" {
                    return Some(Ok(true));
                }
            }
            Some(Ok(false))
        }

        // Here we invoke the actual callback iteratively until we find the element.
        loop {
            match self.reader.seek_start_callback(b"name", 3, record, is_organism)? {
                Err(e)  => return Some(Err(e)),
                Ok(v)   => {
                    if v {
                        try_opterr!(self.read_organism_impl(record));
                        return self.read_taxonomy(record)
                    }
                }
            }
        }
    }

    /// Parse the UniProt record.
    fn parse_record(&mut self) -> Option<ResultType<Record>> {
        let mut record = Record::new();

        try_opterr!(self.read_accession(&mut record));
        try_opterr!(self.read_mnemonic(&mut record));
        try_opterr!(self.read_protein(&mut record));
        try_opterr!(self.read_gene(&mut record));
        try_opterr!(self.read_organism(&mut record));

        // TODO(ahuszagh)...
        //      Add more calls here...
        //      Need:
        //          sequence_version
        //          protein_evidence
        //          mass
        //          length
        //          proteome
        //          sequence

        Some(Ok(record))
    }
}

impl<T: BufRead> Iterator for XmlRecordIter<T> {
    type Item = ResultType<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        // Enter the entry, which stores our position for the entry element.
        try_opterr!(self.enter_entry());
        let record = self.parse_record()?;

        // Exit the entry, so we're ready for the next iteration.
        match self.leave_entry() {
            None    => return Some(Err(From::from(ErrorKind::UnexpectedEof))),
            Some(v) => match v {
                Err(e)  => return Some(Err(e)),
                _  => (),
            },
        }

        Some(record)
    }
}

// READER

/// Import record data from XML.
#[allow(dead_code)]
fn iterator_from_uniprot<T: BufRead>(reader: T)
    -> XmlRecordIter<T>
{
    XmlRecordIter::new(reader)
}

/// Import record from XML.
#[allow(unused_mut)]
pub fn record_from_xml<T: BufRead>(reader: &mut T)
    -> ResultType<Record>
{
    match iterator_from_uniprot(reader).next() {
        None    => Err(From::from(ErrorKind::UnexpectedEof)),
        Some(v) => v
    }
}

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
            return Err(From::from(ErrorKind::InvalidRecord));
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
            return Err(From::from(ErrorKind::InvalidRecord));
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
    use std::fs::File;
    use std::io::{BufReader};
    use std::path::PathBuf;
    use test::testdata_dir;
    use super::*;

    fn xml_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("uniprot/xml");
        dir
    }

    #[test]
    //#[ignore]     // TODO(ahuszagh)    Restore
    fn gapdh_test() {
        let mut path = xml_dir();
        path.push("P46406.xml");
        let mut reader = BufReader::new(File::open(path).unwrap());

        let record = record_from_xml(&mut reader);
        panic!("At the disco! {:?}", record);

//        let expected = vec!["A0A2U8RNL1", "P02769", "P46406", "Q53FP0"];
//        let v = RecordList::from_csv(&mut reader, b'\t').unwrap();
//        let actual: Vec<String> = v.iter().map(|r| r.id.clone()).collect();
//        assert_eq!(expected, actual);
    }

    // TODO(ahuszagh)
    //  Implement...
}
