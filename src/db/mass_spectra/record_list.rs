//! Model for spectral collections.

use super::record::Record;

/// Spectral record collection type.
pub type RecordList = Vec<Record>;

// TESTS
// -----

#[cfg(test)]
mod tests {
    use traits::*;
    use super::*;
    use super::super::test::*;

    #[test]
    fn debug_list_test() {
        let text = format!("{:?}", vec![mgf_empty(), mgf_empty()]);
        assert_eq!(text, "[Record { num: 33450, ms_level: 0, rt: 8692.0, parent_mz: 775.15625, parent_intensity: 170643.953125, parent_z: 4, file: \"QPvivo_2015_11_10_1targetmethod\", filter: \"\", peaks: [], parent: [], children: [] }, Record { num: 33450, ms_level: 0, rt: 8692.0, parent_mz: 775.15625, parent_intensity: 170643.953125, parent_z: 4, file: \"QPvivo_2015_11_10_1targetmethod\", filter: \"\", peaks: [], parent: [], children: [] }]");
    }

    #[test]
    fn equality_list_test() {
        let x = vec![mgf_33450(), mgf_empty()];
        let y = vec![mgf_33450(), mgf_empty()];
        let z = vec![mgf_empty(), mgf_33450()];
        assert_eq!(x, y);
        assert_ne!(x, z);
        assert_ne!(y, z);
    }

    #[test]
    fn properties_list_test() {
        // initial check
        let x = vec![mgf_33450(), mgf_empty()];
        let mut y = vec![mgf_33450(), mgf_33450()];
        assert!(!x.is_valid());
        assert!(!x.is_complete());
        assert!(y.is_valid());
        assert!(!y.is_complete());

        y[1].num = 0;
        assert!(!y.is_valid());
        assert!(!y.is_complete());
    }

    #[cfg(feature = "mgf")]
    fn mgf_list_test(l: RecordList, text: &str, kind: MgfKind) {
        let x = l.to_mgf_string(kind).unwrap();
        assert_eq!(x, text);
        let y = RecordList::from_mgf_string(&x, kind).unwrap();
        assert_eq!(l, y);
    }

    #[cfg(feature = "mgf")]
    #[test]
    fn fullms_mgf_list_test() {
        // sample scan
        mgf_list_test(vec![fullms_mgf_33450()], FULLMS_33450_MGF, MgfKind::FullMs);
        mgf_list_test(vec![mgf_33450()], MSCONVERT_33450_MGF, MgfKind::MsConvert);
        mgf_list_test(vec![mgf_33450()], PAVA_33450_MGF, MgfKind::Pava);
        mgf_list_test(vec![mgf_33450()], PWIZ_33450_MGF, MgfKind::Pwiz);

        // empty scan
        mgf_list_test(vec![fullms_mgf_empty()], FULLMS_EMPTY_MGF, MgfKind::FullMs);
        mgf_list_test(vec![mgf_empty()], MSCONVERT_EMPTY_MGF, MgfKind::MsConvert);
        mgf_list_test(vec![mgf_empty()], PAVA_EMPTY_MGF, MgfKind::Pava);
        mgf_list_test(vec![mgf_empty()], PWIZ_EMPTY_MGF, MgfKind::Pwiz);
    }
}
