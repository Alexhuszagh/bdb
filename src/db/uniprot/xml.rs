//! Helper utilities for XML loading and saving.
//!
//! This module, especially the implementation of the reader, is quite
//! difficult to understand, due to the low-level optimizations and the
//! SAX-like API present for the pull XML parser. The module is copiously
//! commented to try to facilitate maintainability.

use quick_xml::events::BytesStart;
use std::io::{BufRead, Write};
use std::str as stdstr;

use traits::*;
use util::{ErrorKind, ResultType, XmlReader, XmlWriter};
use super::evidence::ProteinEvidence;
use super::record::Record;
use super::record_list::RecordList;

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
#[inline(always)]
pub fn record_from_xml<T: BufRead>(reader: &mut T)
    -> ResultType<Record>
{
    match iterator_from_xml(reader).next() {
        None    => Err(From::from(ErrorKind::UnexpectedEof)),
        Some(v) => v
    }
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

/// Macro to parse an attribute.
macro_rules! parse_attribute {
    ($result:ident) => ({
        match $result {
            Err(e) => return Some(Err(From::from(ErrorKind::Xml(e)))),
            Ok(v)  => v,
        }
    });
}

/// Macro to parse UTF8 from an attribute.
macro_rules! from_utf8 {
    ($attribute:ident) => ({
        match stdstr::from_utf8(&*$attribute.value) {
            Err(e)  => return Some(Err(From::from(ErrorKind::Utf8(e)))),
            Ok(v)   => v,
        }
    });
}

/// Macro to parse an integer from a `str`.
macro_rules! parse_integer {
    ($s:expr) => ({
        match $s.parse() {
            Err(e)  => return Some(Err(From::from(e))),
            Ok(v)   => v,
        }
    });
    ($s:expr, $t:ty) => ({
        match $s.parse::<$t>() {
            Err(e)  => return Some(Err(From::from(e))),
            Ok(v)   => v,
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
        self.reader.seek_start(b"entry", 1)
    }

    /// Leave the entry element.
    #[inline]
    fn leave_entry(&mut self) -> Option<ResultType<()>> {
        self.reader.seek_end(b"entry", 1)
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

        self.reader.seek_end(b"recommendedName", 3)
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
                let attribute = parse_attribute!(result);
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
                        return self.reader.seek_end(b"gene", 2);
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
                let attribute = parse_attribute!(result);
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

        // Invoke our callback
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
                let attribute = parse_attribute!(result);
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
                        try_opterr!(self.read_taxonomy(record));
                        // Leave organism for next element to shine.
                        return self.reader.seek_end(b"organism", 2)
                    }
                }
            }
        }
    }

    /// Read the proteome ID.
    #[inline]
    fn read_proteome(&mut self, record: &mut Record) -> Option<ResultType<()>> {
        //  Proteomes XML format.
        //        <dbReference type="Proteomes" id="UP000001811">

        // Callback to determine if we're reading the proteome reference.
        fn parse_proteome<'a>(event: BytesStart<'a>, record: &mut Record)
            -> Option<ResultType<bool>>
        {
            for result in event.attributes() {
                let attribute = parse_attribute!(result);
                if attribute.key == b"type" && &*attribute.value != b"Proteomes" {
                    return Some(Ok(false));
                } else if attribute.key == b"id" {
                     // Parse the taxonomic identifier.
                    record.proteome = match String::from_utf8(attribute.value.to_vec()) {
                        Err(e) => return Some(Err(From::from(ErrorKind::FromUtf8(e)))),
                        Ok(v)  => v,
                    };
                    return Some(Ok(true));
                }
            }
            Some(Ok(false))
        }

        // Here we invoke the actual callback iteratively until we find the element.
        loop {
            match self.reader.seek_start_callback(b"dbReference", 2, record, parse_proteome)? {
                Err(e)  => return Some(Err(e)),
                Ok(v)   => {
                    if v {
                        return Some(Ok(()));
                    }
                }
            }
        }
    }

    /// Read the protein evidence.
    #[inline]
    fn read_evidence(&mut self, record: &mut Record) -> Option<ResultType<()>> {
        // Callback to parse the protein evidence information.
        fn parse_evidence<'a>(event: BytesStart<'a>, record: &mut Record)
            -> Option<ResultType<bool>>
        {
            for result in event.attributes() {
                let attribute = parse_attribute!(result);
                if attribute.key == b"type" {
                    // Parse the taxonomic identifier.
                    let pe = from_utf8!(attribute);
                    record.protein_evidence = match ProteinEvidence::from_xml_verbose(pe) {
                        Err(e) => return Some(Err(e)),
                        Ok(v)  => v,
                    };
                    return Some(Ok(true));
                }
            }
            Some(Ok(false))
        }

        // Invoke our callback
        Some(match self.reader.seek_start_callback(b"proteinExistence", 2, record, parse_evidence)? {
            Err(e)  => Err(e),
            Ok(_)   => Ok(()),
        })
    }

    // Read the sequence.
    #[inline]
    fn read_sequence(&mut self, record: &mut Record) -> Option<ResultType<()>> {
        // Callback to parse the protein evidence information.
        fn parse_sequence<'a>(event: BytesStart<'a>, record: &mut Record)
            -> Option<ResultType<bool>>
        {
            for result in event.attributes() {
                let attribute = parse_attribute!(result);

                if attribute.key == b"length" {
                    record.length = parse_integer!(from_utf8!(attribute));
                } else if attribute.key == b"mass" {
                    record.mass = parse_integer!(from_utf8!(attribute));
                } else if attribute.key == b"version" {
                    record.sequence_version = parse_integer!(from_utf8!(attribute));
                }
            }
            Some(Ok(true))
        }

        // Invoke our callback
        Some(match self.reader.seek_start_callback(b"sequence", 2, record, parse_sequence)? {
            Err(e)  => Err(e),
            Ok(_)   => {
                match self.reader.read_text(b"sequence") {
                    Err(e)  => Err(e),
                    Ok(v)   => {
                        record.sequence = String::with_capacity(v.len());
                        v.split("\n").for_each(|s| record.sequence += s);
                        Ok(())
                    },
                }
            },
        })
    }

    /// Parse the UniProt record.
    fn parse_record(&mut self) -> Option<ResultType<Record>> {
        let mut record = Record::new();

        try_opterr!(self.read_accession(&mut record));
        try_opterr!(self.read_mnemonic(&mut record));
        try_opterr!(self.read_protein(&mut record));
        try_opterr!(self.read_gene(&mut record));
        try_opterr!(self.read_organism(&mut record));
        try_opterr!(self.read_proteome(&mut record));
        try_opterr!(self.read_evidence(&mut record));
        try_opterr!(self.read_sequence(&mut record));

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

// READER -- DEFAULT

/// Import record data from XML.
#[inline(always)]
fn iterator_from_xml<T: BufRead>(reader: T)
    -> XmlRecordIter<T>
{
    XmlRecordIter::new(reader)
}

// READER -- STRICT

/// Iterator to lazily load `Record`s from a document.
pub struct XmlRecordStrictIter<T: BufRead> {
    iter: XmlRecordIter<T>,
}

impl<T: BufRead> XmlRecordStrictIter<T> {
     /// Create new XmlRecordStrictIter from a reader.
    #[inline]
    pub fn new(reader: T) -> Self {
        XmlRecordStrictIter {
            iter: XmlRecordIter::new(reader),
        }
    }
}

impl<T: BufRead> Iterator for XmlRecordStrictIter<T> {
    type Item = ResultType<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next()? {
            Err(e)  => Some(Err(e)),
            Ok(r)   => {
                if r.is_valid() {
                    Some(Ok(r))
                } else {
                    Some(Err(From::from(ErrorKind::InvalidRecord)))
                }
            }
        }
    }
}

/// Create strict record iterator from reader.
#[inline(always)]
pub fn iterator_from_xml_strict<T: BufRead>(reader: T) -> XmlRecordStrictIter<T> {
    XmlRecordStrictIter::new(reader)
}

// READER -- LENIENT

/// Iterator to lazily load `Record`s from a document.
pub struct XmlRecordLenientIter<T: BufRead> {
    iter: XmlRecordIter<T>,
}

impl<T: BufRead> XmlRecordLenientIter<T> {
     /// Create new XmlRecordLenientIter from a reader.
    #[inline]
    pub fn new(reader: T) -> Self {
        XmlRecordLenientIter {
            iter: XmlRecordIter::new(reader),
        }
    }
}

impl<T: BufRead> Iterator for XmlRecordLenientIter<T> {
    type Item = ResultType<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next()? {
                Err(e)  => return Some(Err(e)),
                Ok(r)   => {
                    if r.is_valid() {
                        return Some(Ok(r));
                    }
                },
            }
        }
    }
}

/// Create lenient record iterator from reader.
#[inline(always)]
pub fn iterator_from_xml_lenient<T: BufRead>(reader: T) -> XmlRecordLenientIter<T> {
    XmlRecordLenientIter::new(reader)
}

// XML UNIPROT WRITER

/// Internal XML writer for UniProt records.
struct XmlUniProtWriter<T: Write> {
    writer: XmlWriter<T>
}

impl<T: Write> XmlUniProtWriter<T> {
    /// Create new XmlUniProtWriter.
    #[inline]
    pub fn new(writer: T) -> Self {
        XmlUniProtWriter {
            writer: XmlWriter::new(writer)
        }
    }

    /// Write the XML declaration.
    #[inline(always)]
    pub fn write_declaration(&mut self) -> ResultType<()> {
        self.writer.write_declaration()
    }

    /// Write the UniProt start element.
    #[inline]
    fn write_uniprot_start(&mut self) -> ResultType<()> {
        self.writer.write_start_element(b"uniprot", &[
            (b"xlmns", b"http://uniprot.org/uniprot"),
            (b"xmlns:xsi", b"http://www.w3.org/2001/XMLSchema-instance"),
            (b"xmlns:schemaLocation", b"http://uniprot.org/uniprot http://www.uniprot.org/support/docs/uniprot.xsd")
        ])
    }

    /// Write the UniProt end element.
    #[inline]
    fn write_uniprot_end(&mut self) -> ResultType<()> {
        self.writer.write_end_element(b"uniprot")
    }

    /// Write the entry start element.
    #[inline]
    fn write_entry_start(&mut self) -> ResultType<()> {
        self.writer.write_start_element(b"entry", &[
            (b"dataset", b"Swiss-Prot"),
            (b"created", b"1995-11-01")
        ])
    }

    /// Write the entry end element.
    #[inline]
    fn write_entry_end(&mut self) -> ResultType<()> {
        self.writer.write_end_element(b"entry")
    }

    /// Write the accession element.
    #[inline]
    fn write_id(&mut self, record: &Record) -> ResultType<()> {
        self.writer.write_text_element(b"accession", &record.id, &[])
    }

    /// Write the mnemonic element.
    #[inline]
    fn write_mnemonic(&mut self, record: &Record) -> ResultType<()> {
        self.writer.write_text_element(b"name", &record.mnemonic, &[])
    }

    /// Write the protein element.
    #[inline]
    fn write_protein(&mut self, record: &Record) -> ResultType<()> {
        self.writer.write_start_element(b"protein", &[])?;
        self.write_recommended_name(record)?;
        self.writer.write_end_element(b"protein")
    }

    /// Write the protein element.
    #[inline]
    fn write_recommended_name(&mut self, record: &Record) -> ResultType<()> {
        self.writer.write_start_element(b"recommendedName", &[])?;
        self.write_full_name(record)?;
        self.write_gene_name(record)?;
        self.writer.write_end_element(b"recommendedName")
    }

    /// Write the name element.
    #[inline]
    fn write_full_name(&mut self, record: &Record) -> ResultType<()> {
        self.writer.write_text_element(b"fullName", &record.name, &[])
    }

    /// Write the gene element.
    #[inline]
    fn write_gene_name(&mut self, record: &Record) -> ResultType<()> {
        self.writer.write_text_element(b"shortName", &record.gene, &[])
    }

    /// Write the gene information element.
    #[inline]
    fn write_gene(&mut self, record: &Record) -> ResultType<()> {
        self.writer.write_start_element(b"gene", &[])?;
        self.write_primary_name(record)?;
        self.writer.write_end_element(b"gene")
    }

    /// Write the primary gene name element.
    #[inline]
    fn write_primary_name(&mut self, record: &Record) -> ResultType<()> {
        self.writer.write_text_element(b"name", &record.gene, &[
            (b"type", b"primary")
        ])
    }

    /// Write the organism information element.
    #[inline]
    fn write_organism(&mut self, record: &Record) -> ResultType<()> {
        self.writer.write_start_element(b"organism", &[])?;
        // Skip the common name since we can never guess....
        self.write_scientific_name(record)?;
        self.write_taxonomy_id(record)?;
        // Skip the lineage section since we don't store that info.
        self.writer.write_end_element(b"organism")
    }

    #[inline]
    fn write_scientific_name(&mut self, record: &Record) -> ResultType<()> {
        self.writer.write_text_element(b"name", &record.organism, &[
            (b"type", b"scientific")
        ])
    }

    #[inline]
    fn write_taxonomy_id(&mut self, record: &Record) -> ResultType<()> {
        self.writer.write_empty_element(b"dbReference", &[
            (b"type", b"NCBI Taxonomy"),
            (b"id", record.taxonomy.as_bytes())
        ])
    }

    #[inline]
    fn write_proteome(&mut self, record: &Record) -> ResultType<()> {
        self.writer.write_start_element(b"dbReference", &[
            (b"type", b"Proteomes"),
            (b"id", record.proteome.as_bytes())
        ])?;
        self.writer.write_empty_element(b"property", &[
            (b"type", b"component"),
            (b"value", b"Genome")
        ])?;

        self.writer.write_end_element(b"dbReference")
    }

    #[inline]
    fn write_protein_existence(&mut self, record: &Record) -> ResultType<()> {
        self.writer.write_empty_element(b"proteinExistence", &[
            (b"type", record.protein_evidence.xml_verbose().as_bytes())
        ])
    }

    #[inline]
    fn write_sequence(&mut self, record: &Record) -> ResultType<()>
    {
        let length = record.length.to_string();
        let mass = record.mass.to_string();
        let version = record.sequence_version.to_string();

        self.writer.write_text_element(b"sequence", &record.sequence, &[
            (b"length", length.as_bytes()),
            (b"mass", mass.as_bytes()),
            (b"version", version.as_bytes())
        ])
    }

    /// Write the entry element.
    #[inline]
    fn write_entry(&mut self, record: &Record) -> ResultType<()> {
        self.write_entry_start()?;
        self.write_id(record)?;
        self.write_mnemonic(record)?;
        self.write_protein(record)?;
        self.write_gene(record)?;
        self.write_organism(record)?;
        self.write_proteome(record)?;
        self.write_protein_existence(record)?;
        self.write_sequence(record)?;

        self.write_entry_end()
    }
}

// WRITER

/// Export record data to XML.
fn item_to_xml<T: Write>(record: &Record, writer: &mut XmlUniProtWriter<T>)
    -> ResultType<()>
{
    writer.write_uniprot_start()?;
    writer.write_entry(record)?;
    writer.write_uniprot_end()
}

/// Export record to XML.
pub fn record_to_xml<T: Write>(record: &Record, writer: &mut T)
    -> ResultType<()>
{
    let mut writer = XmlUniProtWriter::new(writer);
    writer.write_declaration()?;
    item_to_xml(record, &mut writer)
}

// WRITER -- DEFAULT

/// Default exporter from a non-owning iterator to XML.
pub fn reference_iterator_to_xml<'a, Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    let mut writer = XmlUniProtWriter::new(writer);
    writer.write_declaration()?;
    writer.write_uniprot_start()?;

    for record in iter {
        writer.write_entry(record)?;
    }

    writer.write_uniprot_end()
}

/// Default exporter from an owning iterator to XML.
pub fn value_iterator_to_xml<Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    let mut writer = XmlUniProtWriter::new(writer);
    writer.write_declaration()?;
    writer.write_uniprot_start()?;

    for record in iter {
        writer.write_entry(&record?)?;
    }

    writer.write_uniprot_end()
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to XML.
pub fn reference_iterator_to_xml_strict<'a, Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    let mut writer = XmlUniProtWriter::new(writer);
    writer.write_declaration()?;
    writer.write_uniprot_start()?;

    for record in iter {
        if record.is_valid() {
            writer.write_entry(record)?;
        } else {
            return Err(From::from(ErrorKind::InvalidRecord));
        }
    }

    writer.write_uniprot_end()
}

/// Strict exporter from an owning iterator to XML.
pub fn value_iterator_to_xml_strict<Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    let mut writer = XmlUniProtWriter::new(writer);
    writer.write_declaration()?;
    writer.write_uniprot_start()?;

    for result in iter {
        let record = result?;
        if record.is_valid() {
            writer.write_entry(&record)?;
        } else {
            return Err(From::from(ErrorKind::InvalidRecord));
        }
    }

    writer.write_uniprot_end()
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to XML.
pub fn reference_iterator_to_xml_lenient<'a, Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    let mut writer = XmlUniProtWriter::new(writer);
    writer.write_declaration()?;
    writer.write_uniprot_start()?;

    for record in iter {
        if record.is_valid() {
            writer.write_entry(record)?;
        }
    }

    writer.write_uniprot_end()
}

/// Lenient exporter from an owning iterator to XML.
pub fn value_iterator_to_xml_lenient<Iter, T>(iter: Iter, writer: &mut T)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    let mut writer = XmlUniProtWriter::new(writer);
    writer.write_declaration()?;
    writer.write_uniprot_start()?;

    for result in iter {
        let record = result?;
        if record.is_valid() {
            writer.write_entry(&record)?;
        }
    }

    writer.write_uniprot_end()
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
    fn from_xml<T: BufRead>(reader: &mut T) -> ResultType<Self> {
        iterator_from_xml(reader).collect()
    }
}

impl XmlCollection for RecordList {
    #[inline(always)]
    fn to_xml_strict<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        reference_iterator_to_xml_strict(self.iter(), writer)
    }

    #[inline(always)]
    fn to_xml_lenient<T: Write>(&self, writer: &mut T) -> ResultType<()> {
        reference_iterator_to_xml_lenient(self.iter(), writer)
    }

    #[inline(always)]
    fn from_xml_strict<T: BufRead>(reader: &mut T) -> ResultType<Self> {
        iterator_from_xml_strict(reader).collect()
    }

    #[inline(always)]
    fn from_xml_lenient<T: BufRead>(reader: &mut T) -> ResultType<Self> {
        Ok(iterator_from_xml_lenient(reader).filter_map(Result::ok).collect())
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
    use super::super::test::*;

    fn xml_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("uniprot/xml");
        dir
    }

    #[test]
    #[ignore]
    fn gapdh_test() {
        let mut path = xml_dir();
        path.push("P46406.xml");
        let mut reader = BufReader::new(File::open(path).unwrap());

        let p = gapdh();
        let record = record_from_xml(&mut reader).unwrap();
        assert_eq!(p, record);
    }

    // TODO(ahuszagh)
    //  Implement...
}
