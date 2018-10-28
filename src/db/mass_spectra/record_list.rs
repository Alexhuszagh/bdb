//! Model for spectral collections.

use super::record::Record;

/// Spectral record collection type.
pub type RecordList = Vec<Record>;

// TESTS
// -----

#[cfg(test)]
mod tests {
//    use super::*;
    use super::super::test::*;

    #[test]
    fn debug_record_list() {
        let text = format!("{:?}", vec![mgf_empty(), mgf_empty()]);
        assert_eq!(text, "[Record { num: 33450, ms_level: 0, rt: 8692.0, parent_mz: 775.15625, parent_intensity: 170643.953125, parent_z: 4, file: \"QPvivo_2015_11_10_1targetmethod\", filter: \"\", peaks: [], parent: [], children: [] }, Record { num: 33450, ms_level: 0, rt: 8692.0, parent_mz: 775.15625, parent_intensity: 170643.953125, parent_z: 4, file: \"QPvivo_2015_11_10_1targetmethod\", filter: \"\", peaks: [], parent: [], children: [] }]");
    }

    #[test]
    fn equality_record_list() {
        let x = vec![mgf_33450(), mgf_empty()];
        let y = vec![mgf_33450(), mgf_empty()];
        let z = vec![mgf_empty(), mgf_33450()];
        assert_eq!(x, y);
        assert_ne!(x, z);
        assert_ne!(y, z);
    }

    // TODO(ahuszagh)   Add more unittests...
}
