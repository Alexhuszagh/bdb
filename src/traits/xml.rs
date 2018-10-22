use std::convert::AsRef;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Cursor, Write};
use std::path::Path;

use util::ResultType;

/// Serialize to and from XML.
pub trait Xml: Sized {
    /// Estimate the size of the resulting XML output to avoid reallocations.
    #[inline(always)]
    fn estimate_xml_size(&self) -> usize {
        0
    }

    /// Export model to XML.
    fn to_xml<T: Write>(&self, writer: &mut T) -> ResultType<()>;

    // Export model to XML string.
    fn to_xml_string(&self) -> ResultType<String> {
        let capacity = self.estimate_xml_size();
        let mut writer = Cursor::new(Vec::with_capacity(capacity));

        self.to_xml(&mut writer)?;
        match String::from_utf8(writer.into_inner()) {
            Err(e)  => Err(Box::new(e)),
            Ok(v)   => Ok(v),
        }
    }

    /// Export model to XML output file.
    #[inline]
    fn to_xml_file<P: AsRef<Path>>(&self, path: P) -> ResultType<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.to_xml(&mut writer)
    }

    /// Import model from XML.
    fn from_xml<T: BufRead>(reader: &mut T) -> ResultType<Self>;

    /// Import model from XML string.
    #[inline]
    fn from_xml_string(text: &str) -> ResultType<Self> {
        // Rust uses the contents of the immutable &str as the buffer
        // Cursor is then immutable.
        let mut reader = Cursor::new(text);
        Self::from_xml(&mut reader)
    }

    /// Import model from XML file.
    #[inline]
    fn from_xml_file<P: AsRef<Path>>(path: P) -> ResultType<Self> {
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
    fn to_xml_strict<T: Write>(&self, writer: &mut T) -> ResultType<()>;

    /// Export collection to XML.
    ///
    /// Returns only errors due to serialization issues, otherwise,
    /// exports as many items as possible.
    fn to_xml_lenient<T: Write>(&self, writer: &mut T) -> ResultType<()>;

    /// Import collection from XML.
    ///
    /// Returns an error if any of the items within the XML document
    /// are invalid.
    fn from_xml_strict<T: BufRead>(reader: &mut T) -> ResultType<Self>;

    /// Import collection from XML.
    ///
    /// Returns only errors due to deserialization errors, otherwise,
    /// imports as many items as possible.
    fn from_xml_lenient<T: BufRead>(reader: &mut T) -> ResultType<Self>;
}
