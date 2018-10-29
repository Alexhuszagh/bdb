//! Input and output helpers for mass spectral models.

// RE-EXPORTS

#[cfg(feature = "mgf")]
pub use self::private::FullMsMgf;

#[cfg(feature = "mgf")]
pub use self::private::MsConvertMgf;

#[cfg(feature = "mgf")]
pub use self::private::PavaMgf;

#[cfg(feature = "mgf")]
pub use self::private::PwizMgf;

// PRIVATE
// -------

mod private {

use std::convert::AsRef;
use std::io::{BufRead, Write};
use std::path::Path;

use db::mass_spectra::RecordList;
use traits::*;
use util::{Bytes, Result};

/// Reader/writer for mass spectral FullMs MGF records.
#[cfg(feature = "mgf")]
pub struct FullMsMgf;

#[cfg(feature = "mgf")]
impl FullMsMgf {
    /// Save mass spectral records to stream.
    #[inline(always)]
    pub fn to_stream<T: Write>(list: &RecordList, writer: &mut T) -> Result<()> {
        list.to_mgf(writer, MgfKind::FullMs)
    }

    /// Save mass spectral records to bytes.
    #[inline(always)]
    pub fn to_bytes(list: &RecordList) -> Result<Bytes> {
        list.to_mgf_bytes(MgfKind::FullMs)
    }

    /// Save mass spectral records to string.
    #[inline(always)]
    pub fn to_string(list: &RecordList) -> Result<String> {
        list.to_mgf_string(MgfKind::FullMs)
    }

    /// Save mass spectral records to file.
    #[inline(always)]
    pub fn to_file<P: AsRef<Path>>(list: &RecordList, path: P) -> Result<()> {
        list.to_mgf_file(path, MgfKind::FullMs)
    }

    /// Load mass spectral records from stream.
    #[inline(always)]
    pub fn from_stream<T: BufRead>(reader: &mut T) -> Result<RecordList> {
        RecordList::from_mgf(reader, MgfKind::FullMs)
    }

    /// Load mass spectral records from bytes.
    #[inline(always)]
    pub fn from_bytes(bytes: &[u8]) -> Result<RecordList> {
        RecordList::from_mgf_bytes(bytes, MgfKind::FullMs)
    }

    /// Load mass spectral records from string.
    #[inline(always)]
    pub fn from_string(string: &str) -> Result<RecordList> {
        RecordList::from_mgf_string(string, MgfKind::FullMs)
    }

    /// Load mass spectral records from file.
    #[inline(always)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<RecordList> {
        RecordList::from_mgf_file(path, MgfKind::FullMs)
    }
}

/// Reader/writer for mass spectral MsConvert MGF records.
#[cfg(feature = "mgf")]
pub struct MsConvertMgf;

#[cfg(feature = "mgf")]
impl MsConvertMgf {
    /// Save mass spectral records to stream.
    #[inline(always)]
    pub fn to_stream<T: Write>(list: &RecordList, writer: &mut T) -> Result<()> {
        list.to_mgf(writer, MgfKind::MsConvert)
    }

    /// Save mass spectral records to bytes.
    #[inline(always)]
    pub fn to_bytes(list: &RecordList) -> Result<Bytes> {
        list.to_mgf_bytes(MgfKind::MsConvert)
    }

    /// Save mass spectral records to string.
    #[inline(always)]
    pub fn to_string(list: &RecordList) -> Result<String> {
        list.to_mgf_string(MgfKind::MsConvert)
    }

    /// Save mass spectral records to file.
    #[inline(always)]
    pub fn to_file<P: AsRef<Path>>(list: &RecordList, path: P) -> Result<()> {
        list.to_mgf_file(path, MgfKind::MsConvert)
    }

    /// Load mass spectral records from stream.
    #[inline(always)]
    pub fn from_stream<T: BufRead>(reader: &mut T) -> Result<RecordList> {
        RecordList::from_mgf(reader, MgfKind::MsConvert)
    }

    /// Load mass spectral records from bytes.
    #[inline(always)]
    pub fn from_bytes(bytes: &[u8]) -> Result<RecordList> {
        RecordList::from_mgf_bytes(bytes, MgfKind::MsConvert)
    }

    /// Load mass spectral records from string.
    #[inline(always)]
    pub fn from_string(string: &str) -> Result<RecordList> {
        RecordList::from_mgf_string(string, MgfKind::MsConvert)
    }

    /// Load mass spectral records from file.
    #[inline(always)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<RecordList> {
        RecordList::from_mgf_file(path, MgfKind::MsConvert)
    }
}

/// Reader/writer for mass spectral Pava MGF records.
#[cfg(feature = "mgf")]
pub struct PavaMgf;

#[cfg(feature = "mgf")]
impl PavaMgf {
    /// Save mass spectral records to stream.
    #[inline(always)]
    pub fn to_stream<T: Write>(list: &RecordList, writer: &mut T) -> Result<()> {
        list.to_mgf(writer, MgfKind::Pava)
    }

    /// Save mass spectral records to bytes.
    #[inline(always)]
    pub fn to_bytes(list: &RecordList) -> Result<Bytes> {
        list.to_mgf_bytes(MgfKind::Pava)
    }

    /// Save mass spectral records to string.
    #[inline(always)]
    pub fn to_string(list: &RecordList) -> Result<String> {
        list.to_mgf_string(MgfKind::Pava)
    }

    /// Save mass spectral records to file.
    #[inline(always)]
    pub fn to_file<P: AsRef<Path>>(list: &RecordList, path: P) -> Result<()> {
        list.to_mgf_file(path, MgfKind::Pava)
    }

    /// Load mass spectral records from stream.
    #[inline(always)]
    pub fn from_stream<T: BufRead>(reader: &mut T) -> Result<RecordList> {
        RecordList::from_mgf(reader, MgfKind::Pava)
    }

    /// Load mass spectral records from bytes.
    #[inline(always)]
    pub fn from_bytes(bytes: &[u8]) -> Result<RecordList> {
        RecordList::from_mgf_bytes(bytes, MgfKind::Pava)
    }

    /// Load mass spectral records from string.
    #[inline(always)]
    pub fn from_string(string: &str) -> Result<RecordList> {
        RecordList::from_mgf_string(string, MgfKind::Pava)
    }

    /// Load mass spectral records from file.
    #[inline(always)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<RecordList> {
        RecordList::from_mgf_file(path, MgfKind::Pava)
    }
}

/// Reader/writer for mass spectral Pwiz MGF records.
#[cfg(feature = "mgf")]
pub struct PwizMgf;

#[cfg(feature = "mgf")]
impl PwizMgf {
    /// Save mass spectral records to stream.
    #[inline(always)]
    pub fn to_stream<T: Write>(list: &RecordList, writer: &mut T) -> Result<()> {
        list.to_mgf(writer, MgfKind::Pwiz)
    }

    /// Save mass spectral records to bytes.
    #[inline(always)]
    pub fn to_bytes(list: &RecordList) -> Result<Bytes> {
        list.to_mgf_bytes(MgfKind::Pwiz)
    }

    /// Save mass spectral records to string.
    #[inline(always)]
    pub fn to_string(list: &RecordList) -> Result<String> {
        list.to_mgf_string(MgfKind::Pwiz)
    }

    /// Save mass spectral records to file.
    #[inline(always)]
    pub fn to_file<P: AsRef<Path>>(list: &RecordList, path: P) -> Result<()> {
        list.to_mgf_file(path, MgfKind::Pwiz)
    }

    /// Load mass spectral records from stream.
    #[inline(always)]
    pub fn from_stream<T: BufRead>(reader: &mut T) -> Result<RecordList> {
        RecordList::from_mgf(reader, MgfKind::Pwiz)
    }

    /// Load mass spectral records from bytes.
    #[inline(always)]
    pub fn from_bytes(bytes: &[u8]) -> Result<RecordList> {
        RecordList::from_mgf_bytes(bytes, MgfKind::Pwiz)
    }

    /// Load mass spectral records from string.
    #[inline(always)]
    pub fn from_string(string: &str) -> Result<RecordList> {
        RecordList::from_mgf_string(string, MgfKind::Pwiz)
    }

    /// Load mass spectral records from file.
    #[inline(always)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<RecordList> {
        RecordList::from_mgf_file(path, MgfKind::Pwiz)
    }
}

}   //private

#[cfg(test)]
mod tests {
    use bencher::black_box;
    use std::fs::read_to_string;
    use std::path::PathBuf;
    use test::testdata_dir;
    use super::*;

    #[cfg(feature = "mgf")]
    fn mgf_dir() -> PathBuf {
        let mut dir = testdata_dir();
        dir.push("mass_spectra/mgf");
        dir
    }

    macro_rules! mgf_file_test {
        ($t:tt, $path:ident) => ({
            let expected = read_to_string(&$path).unwrap();
            let actual = $t::to_bytes(&$t::from_file(&$path).unwrap()).unwrap();
            black_box(expected);
            black_box(actual);
        });
    }

    #[cfg(feature = "mgf")]
    #[test]
    #[ignore]
    fn fullms_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_fullms.txt");
        mgf_file_test!(FullMsMgf, path);
    }

    #[cfg(feature = "mgf")]
    #[test]
    #[ignore]
    fn msconvert_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_msconvert_ms2.txt");
        mgf_file_test!(MsConvertMgf, path);
    }

    #[cfg(feature = "mgf")]
    #[test]
    #[ignore]
    fn pava_ms2_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_pava_ms2.txt");
        mgf_file_test!(PavaMgf, path);
    }

    #[cfg(feature = "mgf")]
    #[test]
    #[ignore]
    fn pava_ms3_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_pava_ms3.txt");
        mgf_file_test!(PavaMgf, path);
    }

    #[cfg(feature = "mgf")]
    #[test]
    #[ignore]
    fn pava_ms3_20170411_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_pava_ms3_20170411.txt");
        mgf_file_test!(PavaMgf, path);
    }

    #[cfg(feature = "mgf")]
    #[test]
    #[ignore]
    fn pwiz_ms2_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_pwiz_ms2.txt");
        mgf_file_test!(PwizMgf, path);
    }

    #[cfg(feature = "mgf")]
    #[test]
    #[ignore]
    fn pwiz_ms3_mgf_test() {
        let mut path = mgf_dir();
        path.push("mgf_pwiz_ms3.txt");
        mgf_file_test!(PwizMgf, path);
    }
}
