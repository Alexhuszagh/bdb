use std::convert::AsRef;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Cursor, Write};
use std::path::Path;

use util::{Bytes, Result};

/// Serialize to and from XML.
pub trait Xml: Sized {
    /// Estimate the size of the resulting XML output to avoid reallocations.
    #[inline(always)]
    fn estimate_xml_size(&self) -> usize {
        0
    }

    /// Export model to XML.
    ///
    /// Note that many small writers are made to the writer, so the writer
    /// should be buffered.
    fn to_xml<T: Write>(&self, writer: &mut T) -> Result<()>;

    // Export model to XML bytes.
    fn to_xml_bytes(&self) -> Result<Bytes> {
        let capacity = self.estimate_xml_size();
        let mut writer = Cursor::new(Vec::with_capacity(capacity));

        self.to_xml(&mut writer)?;
        Ok(writer.into_inner())
    }

    /// Export model to XML string.
    #[inline]
    fn to_xml_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.to_xml_bytes()?)?)
    }

    /// Export model to XML output file.
    #[inline]
    fn to_xml_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.to_xml(&mut writer)
    }

    /// Import model from XML.
    fn from_xml<T: BufRead>(reader: &mut T) -> Result<Self>;

    /// Import model from XML bytes.
    #[inline]
    fn from_xml_bytes(bytes: &[u8]) -> Result<Self> {
        // Rust uses the contents of the immutable &str as the buffer
        // Cursor is then immutable.
        let mut reader = Cursor::new(bytes);
        Self::from_xml(&mut reader)
    }

    /// Import model from XML string.
    #[inline]
    fn from_xml_string(string: &str) -> Result<Self> {
        Self::from_xml_bytes(string.as_bytes())
    }

    /// Import model from XML file.
    #[inline]
    fn from_xml_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Self::from_xml(&mut reader)
    }
}

/// Specialization of the `Xml` trait for collections.
pub trait XmlCollection: Xml {
    /// Export collection to XML.
    ///
    /// Returns an error if any of the items within the collection
    /// are invalid.
    ///
    /// Note that many small writers are made to the writer, so the writer
    /// should be buffered.
    fn to_xml_strict<T: Write>(&self, writer: &mut T) -> Result<()>;

    /// Export collection to XML.
    ///
    /// Returns only errors due to serialization issues, otherwise,
    /// exports as many items as possible.
    ///
    /// Note that many small writers are made to the writer, so the writer
    /// should be buffered.
    fn to_xml_lenient<T: Write>(&self, writer: &mut T) -> Result<()>;

    /// Import collection from XML.
    ///
    /// Returns an error if any of the items within the XML document
    /// are invalid.
    fn from_xml_strict<T: BufRead>(reader: &mut T) -> Result<Self>;

    /// Import collection from XML.
    ///
    /// Returns only errors due to deserialization errors, otherwise,
    /// imports as many items as possible.
    fn from_xml_lenient<T: BufRead>(reader: &mut T) -> Result<Self>;
}
