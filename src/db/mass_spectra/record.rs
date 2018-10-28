//! Model for mass spectra definitions.

use super::peak_list::PeakList;

/// Model for a single record from a spectral scan.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Record {
    /// Scan number for the spectrum.
    pub num: u32,
    /// MS acquisition level of the spectrum.
    pub ms_level: u8,
    /// Time of spectrum acquisition.
    pub rt: f64,
    /// Mass to charge value of parent.
    pub parent_mz: f64,
    /// Intensity of parent ion.
    pub parent_intensity: f64,
    /// Charge of parent ion
    pub parent_z: i8,
    /// File of acquisition.
    pub file: String,
    /// Scan filter for MS acquisition.
    pub filter: String,
    /// MS spectral data (m/z, intensity, z)
    pub peaks: PeakList,
    /// Number of parent scans
    pub parent: Vec<u32>,
    /// Number of children scans.
    pub children: Vec<u32>,
}

impl Record {
    /// Create new, empty spectral record.
    #[inline]
    pub fn new() -> Self {
        Record {
            num: 0,
            ms_level: 0,
            rt: 0.0,
            parent_mz: 0.0,
            parent_intensity: 0.0,
            parent_z: 0,
            file: String::new(),
            filter: String::new(),
            peaks: vec![],
            parent: vec![],
            children: vec![],
        }
    }

    /// Create new, empty spectral record.
    #[inline]
    pub fn with_peak_capacity(capacity: usize) -> Self {
        Record {
            num: 0,
            ms_level: 0,
            rt: 0.0,
            parent_mz: 0.0,
            parent_intensity: 0.0,
            parent_z: 0,
            file: String::new(),
            filter: String::new(),
            peaks: PeakList::with_capacity(capacity),
            parent: vec![],
            children: vec![],
        }
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
//    use super::*;
    use super::super::test::*;

    #[test]
    fn debug_record() {
        let text = format!("{:?}", mgf_empty());
        assert_eq!(text, "Record { num: 33450, ms_level: 0, rt: 8692.0, parent_mz: 775.15625, parent_intensity: 170643.953125, parent_z: 4, file: \"QPvivo_2015_11_10_1targetmethod\", filter: \"\", peaks: [], parent: [], children: [] }");
    }

    #[test]
    fn equality_record() {
        let x = mgf_33450();
        let y = mgf_33450();
        let z = mgf_empty();
        assert_eq!(x, y);
        assert_ne!(x, z);
        assert_ne!(y, z);
    }

    // TODO(ahuszagh)   Add more unittests...
}
