use std::convert::AsRef;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Cursor, Write};
use std::path::Path;

use util::ResultType;

/// Identifier for the MGF file format type.
///
/// MGF files are superficially similar text files, with a start and end
/// delimiter for each scan (usually "BEGIN IONS" and "END IONS"). The
/// scan metadata is encoded in a header, and each peak in the scan
/// is encoded on a separate line.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MgfKind {
    /// MSConvert MGF file format.
    MsConvert = 1,
    /// Pava MGF file format.
    Pava = 2,
    ///// ProteoWizard MGF file format.
    //Pwiz = 3,
    ///// Pava MS1 (FullMs) MGF file format.
    //FullMs = 4,
    // TODO(ahuszagh)   Add others...
}


/// Serialize to and from MGF.
///
/// MGF, or Mascot General Format, is generic format with a start and
/// end scan delimiter, along with metadata along 1 or multiple header
/// lines, followed by individual scans (usually with tab-delimited data).
/// There are many MGF flavors, however, a sample (MSConvert) format
/// is shown below.
///
/// # Serialized Format
/// BEGIN IONS
/// TITLE=Sample.33450.33450.4 File:"Sample.raw", NativeID:"controllerType=0 controllerNumber=1 scan=33450"
/// RTINSECONDS=8692.657303
/// PEPMASS=775.15625 170643.953125
/// CHARGE=4+
/// 205.9304178 0.0
/// 205.9320046 0.0
/// 205.9335913 0.0
/// 205.9351781 0.0
/// END IONS
pub trait Mgf: Sized {
    /// Estimate the size of the resulting MGF output to avoid reallocations.
    #[inline(always)]
    fn estimate_mgf_size(&self, _: MgfKind) -> usize {
        0
    }

    /// Export model to MGF.
    fn to_mgf<T: Write>(&self, writer: &mut T, kind: MgfKind) -> ResultType<()>;

    /// Export model to MGF string.
    fn to_mgf_string(&self, kind: MgfKind) -> ResultType<String> {
        let capacity = self.estimate_mgf_size(kind);
        let mut writer = Cursor::new(Vec::with_capacity(capacity));

        self.to_mgf(&mut writer, kind)?;
        match String::from_utf8(writer.into_inner()) {
            Err(e)  => Err(Box::new(e)),
            Ok(v)   => Ok(v),
        }
    }

    /// Export model to MGF output file.
    #[inline]
    fn to_mgf_file<P: AsRef<Path>>(&self, path: P, kind: MgfKind) -> ResultType<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.to_mgf(&mut writer, kind)
    }

    /// Import model from MGF.
    fn from_mgf<T: BufRead>(reader: &mut T, kind: MgfKind) -> ResultType<Self>;

    /// Import model from MGF string.
    #[inline]
    fn from_mgf_string(text: &str, kind: MgfKind) -> ResultType<Self> {
        // Rust uses the contents of the immutable &str as the buffer
        // Cursor is then immutable.
        let mut reader = Cursor::new(text);
        Self::from_mgf(&mut reader, kind)
    }

    /// Import model from MGF file.
    #[inline]
    fn from_mgf_file<P: AsRef<Path>>(path: P, kind: MgfKind) -> ResultType<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Self::from_mgf(&mut reader, kind)
    }
}

/// Specialization of the `Mgf` trait for collections.
pub trait MgfCollection: Mgf {
    /// Export collection to MGF.
    ///
    /// Returns an error if any of the items within the collection
    /// are invalid.
    fn to_mgf_strict<T: Write>(&self, writer: &mut T, kind: MgfKind) -> ResultType<()>;

    /// Export collection to MGF.
    ///
    /// Returns only errors due to serialization issues, otherwise,
    /// exports as many items as possible.
    fn to_mgf_lenient<T: Write>(&self, writer: &mut T, kind: MgfKind) -> ResultType<()>;

    /// Import collection from MGF.
    ///
    /// Returns an error if any of the items within the MGF document
    /// are invalid.
    fn from_mgf_strict<T: BufRead>(reader: &mut T, kind: MgfKind) -> ResultType<Self>;

    /// Import collection from MGF.
    ///
    /// Returns only errors due to deserialization errors, otherwise,
    /// imports as many items as possible.
    fn from_mgf_lenient<T: BufRead>(reader: &mut T, kind: MgfKind) -> ResultType<Self>;
}
