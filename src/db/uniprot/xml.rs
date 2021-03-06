//! Helper utilities for XML loading and saving.
//!
//! This module, especially the implementation of the reader, is quite
//! difficult to understand, due to the low-level optimizations and the
//! SAX-like API present for the pull XML parser. The module is copiously
//! commented to try to facilitate maintainability.

use quick_xml::events::BytesStart;
use std::io::prelude::*;

use traits::*;
use util::*;
use super::evidence::ProteinEvidence;
use super::record::Record;
use super::record_list::RecordList;

// SIZE

/// Estimated size of the XML shared properties/attributes.
const XML_SHARED_SIZE: usize = 244;

/// Estimate the size of an XML record.
#[inline]
fn estimate_record_size(record: &Record) -> usize {
    // The actual size is ~590, but give ourselves some wiggle room
    // for the numbers.
    const XML_RECORD_SIZE: usize = 610;
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
    -> Result<Record>
{
    none_to_error!(iterator_from_xml(reader).next(), UnexpectedEof)
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

/// Macro to process a buffer to UTF8.
macro_rules! from_utf8 {
    ($buf:expr) => (match String::from_utf8($buf) {
        Err(e) => return Some(Err(From::from(ErrorKind::FromUtf8(e)))),
        Ok(v)  => v,
    })
}

/// Macro to parse an integer from a `&[u8]`.
macro_rules! parse_integer {
    ($s:expr) => ({
        match from_bytes($s) {
            Err(e)  => return Some(Err(From::from(e))),
            Ok(v)   => v,
        }
    });
    ($s:expr, $t:ty) => ({
        match from_bytes::<$t>($s) {
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
    fn enter_entry(&mut self) -> Option<Result<bool>> {

        //  Entry XML format.
        //      <entry dataset="TrEMBL" ... />
        //      <entry dataset="Swiss-Prot" ... />

        // Callback to determine if we're using a reviewed database.
        fn is_reviewed<'a>(event: BytesStart<'a>, _: &mut bool)
            -> Option<Result<bool>>
        {
            for result in event.attributes() {
                let attribute = parse_attribute!(result);
                if attribute.key == b"dataset" {
                    if &*attribute.value == b"TrEMBL" {
                        return Some(Ok(false));
                    } else if &*attribute.value == b"Swiss-Prot" {
                        return Some(Ok(true));
                    }
                }
            }
            Some(Err(From::from(ErrorKind::InvalidInput)))
        }

        // Use dummy state.
        let mut state: bool = true;
        self.reader.seek_start_callback(b"entry", 1, &mut state, is_reviewed)
    }

    /// Leave the entry element.
    #[inline]
    fn leave_entry(&mut self) -> Option<Result<()>> {
        self.reader.seek_end(b"entry", 1)
    }

    /// Read the accession number.
    #[inline]
    fn read_accession(&mut self, record: &mut Record) -> Option<Result<()>> {
        try_opterr!(self.reader.seek_start(b"accession", 2));

        match self.reader.read_text(b"accession") {
            Err(e)  => return Some(Err(e)),
            Ok(v)   => record.id = from_utf8!(v),
        }

        Some(Ok(()))
    }

    /// Read the mnemonic identifier.
    #[inline]
    fn read_mnemonic(&mut self, record: &mut Record) -> Option<Result<()>> {
        try_opterr!(self.reader.seek_start(b"name", 2));

        match self.reader.read_text(b"name") {
            Err(e)  => return Some(Err(e)),
            Ok(v)   => record.mnemonic = from_utf8!(v),
        }

        Some(Ok(()))
    }

    /// Read the SwissProt protein name.
    #[inline]
    fn read_swissport_protein(&mut self, record: &mut Record) -> Option<Result<()>> {
        // Ensure we get to the recommendedName, since "alternativeName"
        // also has the same attributes.
        try_opterr!(self.reader.seek_start(b"recommendedName", 3));

        // Read the protein name
        try_opterr!(self.reader.seek_start(b"fullName", 4));
        match self.reader.read_text(b"fullName") {
            Err(e)  => return Some(Err(e)),
            Ok(v)   => record.name = from_utf8!(v),
        }

        self.reader.seek_end(b"recommendedName", 3)
    }

    /// Read the TrEMBL protein name.
    #[inline]
    fn read_trembl_protein(&mut self, record: &mut Record) -> Option<Result<()>> {
        try_opterr!(self.reader.seek_start(b"submittedName", 3));

        // Read the protein name
        try_opterr!(self.reader.seek_start(b"fullName", 4));
        match self.reader.read_text(b"fullName") {
            Err(e)  => return Some(Err(e)),
            Ok(v)   => record.name = from_utf8!(v),
        }

        self.reader.seek_end(b"submittedName", 3)
    }

    /// Read the protein name.
    #[inline]
    fn read_protein(&mut self, record: &mut Record) -> Option<Result<()>> {
        match record.reviewed {
            true    => self.read_swissport_protein(record),
            false   => self.read_trembl_protein(record),
        }
    }

    /// Read the text from the name element.
    #[inline]
    fn read_gene_name(&mut self, record: &mut Record) -> Option<Result<()>> {
        match self.reader.read_text(b"name") {
            Err(e)  => return Some(Err(e)),
            Ok(v)   => record.gene = from_utf8!(v),
        }

        Some(Ok(()))
    }

    /// Read the gene name.
    /// Use as the callback if the seek to the "gene" start element succeededs.
    #[inline]
    fn read_gene_inside(&mut self, record: &mut Record) -> Option<Result<()>> {
        //  Gene XML format.
        //      <gene>
        //      <name type="primary">GAPDH</name>
        //      <name type="synonym">GAPD</name>
        //      </gene>

        // Callback to determine if we're reading the primary gene name.
        fn is_gene<'a>(event: BytesStart<'a>, _: &mut Record)
            -> Option<Result<bool>>
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
                        try_opterr!(self.read_gene_name(record));
                        return self.reader.seek_end(b"gene", 2);
                    }
                }
            }
        }
    }

    /// Read the taxonomy.
    #[inline]
    fn read_taxonomy(&mut self, record: &mut Record) -> Option<Result<()>> {
        // Callback to parse the taxonomy information.
        fn parse_taxonomy<'a>(event: BytesStart<'a>, record: &mut Record)
            -> Option<Result<bool>>
        {
            for result in event.attributes() {
                let attribute = parse_attribute!(result);
                if attribute.key == b"type" && &*attribute.value != b"NCBI Taxonomy" {
                    // If the dbReference type is not NCBI Taxonomy, quit early
                    return Some(Ok(false));
                } else if attribute.key == b"id" {
                    // Parse the taxonomic identifier.
                    record.taxonomy = from_utf8!(attribute.value.to_vec());
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
    fn read_organism_value(&mut self, record: &mut Record) -> Option<Result<()>> {
        match self.reader.read_text(b"name") {
            Err(e)  => return Some(Err(e)),
            Ok(v)   => record.organism = from_utf8!(v),
        }

        Some(Ok(()))
    }

    /// Read the organism name implied.
    /// Use as the callback if the seek to the "gene" start element fails.
    #[inline]
    fn read_organism_inside(&mut self, record: &mut Record) -> Option<Result<()>> {
        //  Organism XML format.
        //        <organism>
        //        <name type="scientific">Oryctolagus cuniculus</name>
        //        <name type="common">Rabbit</name>
        //        <dbReference type="NCBI Taxonomy" id="9986"/>
        //        ...
        //        </organism>

        // Callback to determine if we're reading the scientific name.
        fn is_organism<'a>(event: BytesStart<'a>, _: &mut Record)
            -> Option<Result<bool>>
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
                        try_opterr!(self.read_organism_value(record));
                        try_opterr!(self.read_taxonomy(record));
                        // Leave organism for next element to shine.
                        return self.reader.seek_end(b"organism", 2)
                    }
                }
            }
        }
    }

    /// Read the gene and organism name.
    /// The gene information may be lacking, so we must call
    /// the organism as a fallback if so.
    #[inline]
    fn read_gene_or_organism(&mut self, record: &mut Record) -> Option<Result<()>> {

        match self.reader.seek_start_or_fallback(b"gene", 2, b"organism", 2)? {
            Err(e)  => Some(Err(e)),
            Ok(v)   => {
                if v {
                    // able to find gene, process gene then organism
                    try_opterr!(self.read_gene_inside(record));
                    try_opterr!(self.reader.seek_start(b"organism", 2));
                    self.read_organism_inside(record)
                } else {
                    // unable to find gene, process organism
                    self.read_organism_inside(record)
                }
            },
        }
    }

    /// Read the proteome ID.
    #[inline]
    fn read_proteome(&mut self, record: &mut Record) -> Option<Result<()>> {
        //  Proteomes XML format.
        //        <dbReference type="Proteomes" id="UP000001811">

        // Callback to determine if we're reading the proteome reference.
        fn parse_proteome<'a>(event: BytesStart<'a>, record: &mut Record)
            -> Option<Result<bool>>
        {
            for result in event.attributes() {
                let attribute = parse_attribute!(result);
                if attribute.key == b"type" && &*attribute.value != b"Proteomes" {
                    return Some(Ok(false));
                } else if attribute.key == b"id" {
                     // Parse the taxonomic identifier.
                    record.proteome = from_utf8!(attribute.value.to_vec());
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
    fn read_evidence(&mut self, record: &mut Record) -> Option<Result<()>> {
        // Callback to parse the protein evidence information.
        fn parse_evidence<'a>(event: BytesStart<'a>, record: &mut Record)
            -> Option<Result<bool>>
        {
            for result in event.attributes() {
                let attribute = parse_attribute!(result);
                if attribute.key == b"type" {
                    // Parse the taxonomic identifier.
                    let pe: &[u8] = &*attribute.value;
                    record.protein_evidence = match ProteinEvidence::from_xml_verbose_bytes(pe) {
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
    fn read_sequence(&mut self, record: &mut Record) -> Option<Result<()>> {
        // Callback to parse the protein evidence information.
        fn parse_sequence<'a>(event: BytesStart<'a>, record: &mut Record)
            -> Option<Result<bool>>
        {
            for result in event.attributes() {
                let attribute = parse_attribute!(result);

                if attribute.key == b"length" {
                    record.length = parse_integer!(&*attribute.value);
                } else if attribute.key == b"mass" {
                    record.mass = parse_integer!(&*attribute.value);
                } else if attribute.key == b"version" {
                    record.sequence_version = parse_integer!(&*attribute.value);
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
                        let mut sequence = Vec::with_capacity(v.len());
                        v.split(|c| *c == b'\n').for_each(|s| sequence.extend(s));
                        record.sequence = sequence;
                        Ok(())
                    },
                }
            },
        })
    }

    /// Parse the UniProt record.
    fn parse_record(&mut self, record: &mut Record) -> Option<Result<()>> {
        try_opterr!(self.read_accession(record));
        try_opterr!(self.read_mnemonic(record));
        try_opterr!(self.read_protein(record));
        try_opterr!(self.read_gene_or_organism(record));
        if record.reviewed {
            try_opterr!(self.read_proteome(record));
        }
        try_opterr!(self.read_evidence(record));
        try_opterr!(self.read_sequence(record));

        Some(Ok(()))
    }
}

impl<T: BufRead> Iterator for XmlRecordIter<T> {
    type Item = Result<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        // Enter the entry, which stores our position for the entry element.
        // Get whether the protein entry is reviewed or not from the query.
        let mut record = Record::new();
        record.reviewed = match self.enter_entry()? {
            Err(e)  => return Some(Err(e)),
            Ok(v)   => v,
        };
        try_opterr!(self.parse_record(&mut record));

        // Exit the entry, so we're ready for the next iteration.
        match self.leave_entry() {
            None    => return Some(Err(From::from(ErrorKind::UnexpectedEof))),
            Some(v) => match v {
                Err(e)  => return Some(Err(e)),
                _  => (),
            },
        }

        Some(Ok(record))
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
pub type XmlRecordStrictIter<T> = StrictIter<Record, XmlRecordIter<T>>;

/// Create strict record iterator from reader.
#[inline(always)]
pub fn iterator_from_xml_strict<T: BufRead>(reader: T) -> XmlRecordStrictIter<T> {
    XmlRecordStrictIter::new(iterator_from_xml(reader))
}

// READER -- LENIENT

/// Iterator to lazily load `Record`s from a document.
pub type XmlRecordLenientIter<T> = LenientIter<Record, XmlRecordIter<T>>;

/// Create lenient record iterator from reader.
#[inline(always)]
pub fn iterator_from_xml_lenient<T: BufRead>(reader: T) -> XmlRecordLenientIter<T> {
    XmlRecordLenientIter::new(iterator_from_xml(reader))
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
    pub fn write_declaration(&mut self) -> Result<()> {
        self.writer.write_declaration()
    }

    /// Write the UniProt start element.
    #[inline]
    fn write_uniprot_start(&mut self) -> Result<()> {
        self.writer.write_start_element(b"uniprot", &[
            (b"xlmns", b"http://uniprot.org/uniprot"),
            (b"xmlns:xsi", b"http://www.w3.org/2001/XMLSchema-instance"),
            (b"xmlns:schemaLocation", b"http://uniprot.org/uniprot http://www.uniprot.org/support/docs/uniprot.xsd")
        ])
    }

    /// Write the UniProt end element.
    #[inline]
    fn write_uniprot_end(&mut self) -> Result<()> {
        self.writer.write_end_element(b"uniprot")
    }

    /// Write the entry start element.
    #[inline]
    fn write_entry_start(&mut self, record: &Record) -> Result<()> {
        match record.reviewed {
            true    => self.writer.write_start_element(b"entry", &[
                    (b"dataset", b"Swiss-Prot"),
                ]),
            false   => self.writer.write_start_element(b"entry", &[
                    (b"dataset", b"TrEMBL"),
                ]),
        }
    }

    /// Write the entry end element.
    #[inline]
    fn write_entry_end(&mut self) -> Result<()> {
        self.writer.write_end_element(b"entry")
    }

    /// Write the accession element.
    #[inline]
    fn write_id(&mut self, record: &Record) -> Result<()> {
        self.writer.write_text_element(b"accession", record.id.as_bytes(), &[])
    }

    /// Write the mnemonic element.
    #[inline]
    fn write_mnemonic(&mut self, record: &Record) -> Result<()> {
        self.writer.write_text_element(b"name", record.mnemonic.as_bytes(), &[])
    }

    /// Write the protein element.
    #[inline]
    fn write_protein(&mut self, record: &Record) -> Result<()> {
        self.writer.write_start_element(b"protein", &[])?;
        match record.reviewed {
            true    => self.write_recommended_name(record)?,
            false   => self.write_submitted_name(record)?,
        };
        self.writer.write_end_element(b"protein")
    }

    /// Write the protein element.
    #[inline]
    fn write_recommended_name(&mut self, record: &Record) -> Result<()> {
        self.writer.write_start_element(b"recommendedName", &[])?;
        self.write_full_name(record)?;
        self.write_gene_name(record)?;
        self.writer.write_end_element(b"recommendedName")
    }

    /// Write the protein element.
    #[inline]
    fn write_submitted_name(&mut self, record: &Record) -> Result<()> {
        self.writer.write_start_element(b"submittedName", &[])?;
        self.write_full_name(record)?;
        self.writer.write_end_element(b"submittedName")
    }

    /// Write the name element.
    #[inline]
    fn write_full_name(&mut self, record: &Record) -> Result<()> {
        self.writer.write_text_element(b"fullName", record.name.as_bytes(), &[])
    }

    /// Write the gene element.
    #[inline]
    fn write_gene_name(&mut self, record: &Record) -> Result<()> {
        self.writer.write_text_element(b"shortName", record.gene.as_bytes(), &[])
    }

    /// Write the gene information element.
    #[inline]
    fn write_gene(&mut self, record: &Record) -> Result<()> {
        self.writer.write_start_element(b"gene", &[])?;
        self.write_primary_name(record)?;
        self.writer.write_end_element(b"gene")
    }

    /// Write the primary gene name element.
    #[inline]
    fn write_primary_name(&mut self, record: &Record) -> Result<()> {
        self.writer.write_text_element(b"name", record.gene.as_bytes(), &[
            (b"type", b"primary")
        ])
    }

    /// Write the organism information element.
    #[inline]
    fn write_organism(&mut self, record: &Record) -> Result<()> {
        self.writer.write_start_element(b"organism", &[])?;
        // Skip the common name since we can never guess....
        self.write_scientific_name(record)?;
        self.write_taxonomy_id(record)?;
        // Skip the lineage section since we don't store that info.
        self.writer.write_end_element(b"organism")
    }

    #[inline]
    fn write_scientific_name(&mut self, record: &Record) -> Result<()> {
        self.writer.write_text_element(b"name", record.organism.as_bytes(), &[
            (b"type", b"scientific")
        ])
    }

    #[inline]
    fn write_taxonomy_id(&mut self, record: &Record) -> Result<()> {
        self.writer.write_empty_element(b"dbReference", &[
            (b"type", b"NCBI Taxonomy"),
            (b"id", record.taxonomy.as_bytes())
        ])
    }

    #[inline]
    fn write_proteome(&mut self, record: &Record) -> Result<()> {
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
    fn write_protein_existence(&mut self, record: &Record) -> Result<()> {
        self.writer.write_empty_element(b"proteinExistence", &[
            (b"type", record.protein_evidence.xml_verbose().as_bytes())
        ])
    }

    #[inline]
    fn write_sequence(&mut self, record: &Record) -> Result<()>
    {
        let length = to_bytes(&record.length)?;
        let mass = to_bytes(&record.mass)?;
        let version = to_bytes(&record.sequence_version)?;

        self.writer.write_text_element(b"sequence", record.sequence.as_slice(), &[
            (b"length", length.as_slice()),
            (b"mass", mass.as_slice()),
            (b"version", version.as_slice())
        ])
    }

    /// Write the entry element.
    #[inline]
    fn write_entry(&mut self, record: &Record) -> Result<()> {
        self.write_entry_start(record)?;
        self.write_id(record)?;
        self.write_mnemonic(record)?;
        self.write_protein(record)?;
        self.write_gene(record)?;
        self.write_organism(record)?;
        if record.reviewed {
            self.write_proteome(record)?;
        }
        self.write_protein_existence(record)?;
        self.write_sequence(record)?;

        self.write_entry_end()
    }
}

// WRITER

/// Export record data to XML.
fn item_to_xml<T: Write>(writer: &mut XmlUniProtWriter<T>, record: &Record)
    -> Result<()>
{
    writer.write_uniprot_start()?;
    writer.write_entry(record)?;
    writer.write_uniprot_end()
}

/// Export record to XML.
pub fn record_to_xml<T: Write>(writer: &mut T, record: &Record)
    -> Result<()>
{
    let mut writer = XmlUniProtWriter::new(writer);
    writer.write_declaration()?;
    item_to_xml(&mut writer, record)
}

// WRITER -- DEFAULT

#[inline(always)]
fn to_xml<'a, T: Write>(writer: &mut XmlUniProtWriter<T>, record: &'a Record)
    -> Result<()>
{
    writer.write_entry(record)
}

#[inline(always)]
fn init_cb<T: Write>(writer: T, _: u8)
    -> Result<XmlUniProtWriter<T>>
{
    let mut writer = XmlUniProtWriter::new(writer);
    writer.write_declaration()?;
    writer.write_uniprot_start()?;
    Ok(writer)
}

#[inline(always)]
fn export_cb<'a, T: Write>(writer: &mut XmlUniProtWriter<T>, record: &'a Record)
    -> Result<()>
{
    to_xml(writer, record)
}

#[inline(always)]
fn dest_cb<T: Write>(writer: &mut XmlUniProtWriter<T>)
    -> Result<()>
{
    writer.write_uniprot_end()
}

/// Default exporter from a non-owning iterator to XML.
#[inline(always)]
pub fn reference_iterator_to_xml<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export(writer, iter, b'\0', &init_cb, &export_cb, &dest_cb)
}

/// Default exporter from an owning iterator to XML.
#[inline(always)]
pub fn value_iterator_to_xml<Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    value_iterator_export(writer, iter, b'\0', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to XML.
#[inline(always)]
pub fn reference_iterator_to_xml_strict<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_strict(writer, iter, b'\0', &init_cb, &export_cb, &dest_cb)
}

/// Strict exporter from an owning iterator to XML.
#[inline(always)]
pub fn value_iterator_to_xml_strict<Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    value_iterator_export_strict(writer, iter, b'\0', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to XML.
#[inline(always)]
pub fn reference_iterator_to_xml_lenient<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_lenient(writer, iter, b'\0', &init_cb, &export_cb, &dest_cb)
}

/// Lenient exporter from an owning iterator to XML.
#[inline(always)]
pub fn value_iterator_to_xml_lenient<Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    value_iterator_export_lenient(writer, iter, b'\0', &init_cb, &export_cb, &dest_cb)
}

// TRAITS

impl Xml for Record {
    #[inline(always)]
    fn estimate_xml_size(&self) -> usize {
        XML_SHARED_SIZE + estimate_record_size(self)
    }

    #[inline(always)]
    fn to_xml<T: Write>(&self, writer: &mut T) -> Result<()> {
        record_to_xml(writer, self)
    }

    #[inline(always)]
    fn from_xml<T: BufRead>(reader: &mut T) -> Result<Self> {
        record_from_xml(reader)
    }
}

impl Xml for RecordList {
    #[inline(always)]
    fn estimate_xml_size(&self) -> usize {
        XML_SHARED_SIZE + estimate_list_size(self)
    }

    #[inline(always)]
    fn to_xml<T: Write>(&self, writer: &mut T) -> Result<()> {
        reference_iterator_to_xml(writer, self.iter())
    }

    #[inline(always)]
    fn from_xml<T: BufRead>(reader: &mut T) -> Result<Self> {
        iterator_from_xml(reader).collect()
    }
}

impl XmlCollection for RecordList {
    #[inline(always)]
    fn to_xml_strict<T: Write>(&self, writer: &mut T) -> Result<()> {
        reference_iterator_to_xml_strict(writer, self.iter())
    }

    #[inline(always)]
    fn to_xml_lenient<T: Write>(&self, writer: &mut T) -> Result<()> {
        reference_iterator_to_xml_lenient(writer, self.iter())
    }

    #[inline(always)]
    fn from_xml_strict<T: BufRead>(reader: &mut T) -> Result<Self> {
        iterator_from_xml_strict(reader).collect()
    }

    #[inline(always)]
    fn from_xml_lenient<T: BufRead>(reader: &mut T) -> Result<Self> {
        Ok(iterator_from_xml_lenient(reader).filter_map(Result::ok).collect())
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use bencher;
    use std::fs::File;
    use std::io::{BufReader, Cursor};
    use std::path::PathBuf;
    use test::testdata_dir;
    use super::*;
    use super::super::test::*;

    #[test]
    fn estimate_size_test() {
        let g = gapdh();
        let b = bsa();
        let v = vec![gapdh(), bsa()];
        assert_eq!(estimate_record_size(&g), 1024);
        assert_eq!(estimate_record_size(&b), 1259);
        assert_eq!(estimate_list_size(&v), 2283);
    }

    #[test]
    fn iterator_to_xml_test() {
        let v = vec![gapdh(), bsa()];
        let u = vec![gapdh(), bsa(), Record::new()];

        // reference -- default
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_xml(&mut w, v.iter()).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_XML);

        // value -- default
        let mut w = Cursor::new(vec![]);
        value_iterator_to_xml(&mut w, iterator_by_value!(v.iter())).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_XML);

        // reference -- strict
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_xml_strict(&mut w, v.iter()).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_XML);

        let mut w = Cursor::new(vec![]);
        let r = reference_iterator_to_xml_strict(&mut w, u.iter());
        assert!(r.is_err());

        // value -- strict
        let mut w = Cursor::new(vec![]);
        value_iterator_to_xml_strict(&mut w, iterator_by_value!(v.iter())).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_XML);

        let mut w = Cursor::new(vec![]);
        let r = value_iterator_to_xml_strict(&mut w, iterator_by_value!(u.iter()));
        assert!(r.is_err());

        // reference -- lenient
        let mut w = Cursor::new(vec![]);
        reference_iterator_to_xml_lenient(&mut w, v.iter()).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_XML);

        let mut w = Cursor::new(vec![]);
        reference_iterator_to_xml_lenient(&mut w, u.iter()).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_XML);

        // value -- lenient
        let mut w = Cursor::new(vec![]);
        value_iterator_to_xml_lenient(&mut w, iterator_by_value!(v.iter())).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_XML);

        let mut w = Cursor::new(vec![]);
        value_iterator_to_xml_lenient(&mut w, iterator_by_value!(u.iter())).unwrap();
        assert_eq!(w.into_inner(), GAPDH_BSA_XML);
    }

    #[test]
    fn iterator_from_xml_test() {
        // VALID
        let text = GAPDH_BSA_XML;
        let expected = vec![gapdh(), bsa()];

        // record iterator -- default
        let iter = XmlRecordIter::new(Cursor::new(text));
        let v: Result<RecordList> = iter.collect();
        assert_eq!(&expected, &v.unwrap());

        // Compile check only
        iterator_from_xml(&mut Cursor::new(text));

        // record iterator -- strict
        let iter = iterator_from_xml_strict(Cursor::new(text));
        let v: Result<RecordList> = iter.collect();
        assert_eq!(&expected, &v.unwrap());

        // Compile check only
        iterator_from_xml_strict(&mut Cursor::new(text));

        // record iterator -- lenient
        let iter = iterator_from_xml_lenient(Cursor::new(text));
        let v: Result<RecordList> = iter.collect();
        assert_eq!(&expected, &v.unwrap());

        // Compile check only
        iterator_from_xml_lenient(&mut Cursor::new(text));

        // INVALID
        let text = GAPDH_EMPTY_XML;
        let expected1 = vec![gapdh(), Record::new()];
        let expected2 = vec![gapdh()];

        // record iterator -- default
        let iter = iterator_from_xml(Cursor::new(text));
        let v: Result<RecordList> = iter.collect();
        let v = v.unwrap();
        assert_eq!(expected1.len(), v.len());
        assert_eq!(&expected1[0], &v[0]);
        assert_eq!(expected1[1], v[1]);

        // record iterator -- strict
        let iter = iterator_from_xml_strict(Cursor::new(text));
        let v: Result<RecordList> = iter.collect();
        assert!(v.is_err());

        // record iterator -- lenient
        let iter = iterator_from_xml_lenient(Cursor::new(text));
        let v: Result<RecordList> = iter.collect();
        assert_eq!(&expected2, &v.unwrap());
    }

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

    #[test]
    #[ignore]
    fn bsa_test() {
        let mut path = xml_dir();
        path.push("P02769.xml");
        let mut reader = BufReader::new(File::open(path).unwrap());

        let p = bsa();
        let record = record_from_xml(&mut reader).unwrap();
        assert_eq!(p, record);
    }

    #[test]
    #[ignore]
    fn dpb1_test() {
        let mut path = xml_dir();
        path.push("A0A2U8RNL1.xml");
        let mut reader = BufReader::new(File::open(path).unwrap());

        let record = record_from_xml(&mut reader).unwrap();
        assert_eq!(record.sequence_version, 1);
        assert_eq!(record.protein_evidence, ProteinEvidence::Predicted);
        assert_eq!(record.mass, 10636);
        assert_eq!(record.length, 87);
        assert_eq!(record.gene, "DPB1");
        assert_eq!(record.id, "A0A2U8RNL1");
        assert_eq!(record.mnemonic, "A0A2U8RNL1_HUMAN");
        assert_eq!(record.name, "MHC class II antigen");
        assert_eq!(record.organism, "Homo sapiens");
        assert_eq!(record.proteome, "");
        assert_eq!(record.sequence, b"NYLFQGRQECYAFNGTQRFLERYIYNREEFVRFDSDVGEFRAVTELGRPDEEYWNSQKDILEEKRAVPDRMCRHNYELGGPMTLQRR".to_vec());
        assert_eq!(record.taxonomy, "9606");
        assert_eq!(record.reviewed, false);
    }

    #[test]
    #[ignore]
    fn atcc_xml_test() {
        let mut path = xml_dir();
        path.push("ATCC.xml");
        let reader = BufReader::new(File::open(path).unwrap());
        let iter = XmlRecordIter::new(reader);

        // do nothing, just check it parses.
        for item in iter {
            bencher::black_box(item).unwrap();
        }
    }
}
