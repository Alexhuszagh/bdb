//! Regular expression utilities for mass spectral services.
//!
//! Disable Unicode for all but the generic header formats, which may
//! accept arbitrary Unicode input. The rest should only be valid ASCII,
//! and therefore we should disable matching to Unicode characters
//! explicitly.

use regex::Regex;

// Re-export regular-expression traits.
pub use util::{ExtractionRegex, ValidationRegex};

// MSCONVERT

/// Regular expression to validate and parse MSConvert MGF files.
pub struct MsConvertMgfRegex;

impl MsConvertMgfRegex {
    /// Hard-coded index fields for data extraction.
    pub const FILE_INDEX: usize = 1;
    pub const NUM_INDEX: usize = 2;
    pub const RT_INDEX: usize = 3;
    pub const PARENT_MZ_INDEX: usize = 4;
    pub const PARENT_INTENSITY_INDEX: usize = 5;
    pub const PARENT_Z_INDEX: usize = 6;
    pub const PARENT_Z_SIGN_INDEX: usize = 7;
}

impl ValidationRegex<Regex> for MsConvertMgfRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r##"(?x)(?m)
            \A
            # Line 1, Scan Start Delimiter.
            BEGIN\sIONS\r?\n

            # Line 2, Title Line
            TITLE=
            (?:
                [^.="]+
            )
            \.[[:digit:]]+\.[[:digit:]]+\.[[:digit:]]*
            \sFile:"[^.="]+(?:\.[^.="]+)?",\sNativeID:"
            controllerType=[[:digit:]]+
            \scontrollerNumber=[[:digit:]]+
            \sscan=
            (?:
                [[:digit:]]+
            )
            "\r?\n

            # Line 3, Retention Time.
            RTINSECONDS=
            (?:
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            \r?\n

            # Line 4, Pep Mass.
            PEPMASS=
            (?:
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            (?:
                \s
                (?:
                    [[:digit:]]+(?:\.[[:digit:]]+)?
                )
            )?
            \r?\n

            # Line 5, Charge.
            (?:
                CHARGE=
                (?:
                    [[:digit:]]+
                )
                (?:
                    [+\-]
                )
                \r?\n
            )?
        "##);
        &REGEX
    }
}

impl ExtractionRegex<Regex> for MsConvertMgfRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r##"(?x)(?m)
            \A
            # Line 1, Scan Start Delimiter.
            BEGIN\sIONS\r?\n

            # Line 2, Title Line
            TITLE=
            # Group 1, File Name.
            (
                [^.="]+
            )
            \.[[:digit:]]+\.[[:digit:]]+\.[[:digit:]]*
            \sFile:"[^.="]+(?:\.[^.="]+)?",\sNativeID:"
            controllerType=[[:digit:]]+
            \scontrollerNumber=[[:digit:]]+
            \sscan=
            # Group 2, Scan Number.
            (
                [[:digit:]]+
            )
            "\r?\n

            # Line 3, Retention Time.
            RTINSECONDS=
            # Group 3, Retention Time.
            (
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            \r?\n

            # Line 4, Pep Mass.
            PEPMASS=
            # Group 4, Parent M/Z.
            (
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            (?:
                \s
                # Group 5, Parent Intensity.
                (
                    [[:digit:]]+(?:\.[[:digit:]]+)?
                )
            )?
            \r?\n

            # Line 5, Charge.
            (?:
                CHARGE=
                # Group 6, Parent Charge.
                (
                    [[:digit:]]+
                )
                # Group 7, Charge Sign.
                (
                    [+\-]
                )
                \r?\n
            )?
        "##);
        &REGEX
    }
}

/// Regular expression to validate and parse MSConvert MGF title lines.
pub struct MsConvertMgfTitleRegex;

impl MsConvertMgfTitleRegex {
    /// Hard-coded index fields for data extraction.
    pub const FILE_INDEX: usize = 1;
    pub const NUM_INDEX: usize = 2;
}

impl ValidationRegex<Regex> for MsConvertMgfTitleRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r##"(?x)
            \A
            TITLE=
            (?:
                [^.="]+
            )
            \.[[:digit:]]+\.[[:digit:]]+\.[[:digit:]]*
            \sFile:"[^="]+",\sNativeID:"
            controllerType=[[:digit:]]+
            \scontrollerNumber=[[:digit:]]+
            \sscan=
            (?:
                [[:digit:]]+
            )
            "
            \z
        "##);
        &REGEX
    }
}

impl ExtractionRegex<Regex> for MsConvertMgfTitleRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r##"(?x)
            \A
            TITLE=
            # Group 1, File Name.
            (
                [^.="]+
            )
            \.[[:digit:]]+\.[[:digit:]]+\.[[:digit:]]*
            \sFile:"[^="]+",\sNativeID:"
            controllerType=[[:digit:]]+
            \scontrollerNumber=[[:digit:]]+
            \sscan=
            # Group 2, Scan Number.
            (
                [[:digit:]]+
            )
            "
            \z
        "##);
        &REGEX
    }
}

/// Regular expression to validate and parse MSConvert MGF RT lines.
pub struct MsConvertMgfRtRegex;

impl MsConvertMgfRtRegex {
    /// Hard-coded index fields for data extraction.
    pub const RT_INDEX: usize = 1;
}

impl ValidationRegex<Regex> for MsConvertMgfRtRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            RTINSECONDS=
            (?:
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<Regex> for MsConvertMgfRtRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            RTINSECONDS=
            # Group 1, Retention Time.
            (
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            \z
        ");
        &REGEX
    }
}

/// Regular expression to validate and parse MSConvert MGF pepmass lines.
pub struct MsConvertMgfPepMassRegex;

impl MsConvertMgfPepMassRegex {
    /// Hard-coded index fields for data extraction.
    pub const PARENT_MZ_INDEX: usize = 1;
    pub const PARENT_INTENSITY_INDEX: usize = 2;
}

impl ValidationRegex<Regex> for MsConvertMgfPepMassRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            PEPMASS=
            (?:
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            (?:
                \s
                (?:
                    [[:digit:]]+(?:\.[[:digit:]]+)?
                )
            )?
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<Regex> for MsConvertMgfPepMassRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            PEPMASS=
            # Group 1, Parent M/Z.
            (
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            (?:
                \s
                # Group 2, Parent Intensity.
                (
                    [[:digit:]]+(?:\.[[:digit:]]+)?
                )
            )?
            \z
        ");
        &REGEX
    }
}

/// Regular expression to validate and parse MSConvert MGF charge lines.
pub struct MsConvertMgfChargeRegex;

impl MsConvertMgfChargeRegex {
    /// Hard-coded index fields for data extraction.
    pub const PARENT_Z_INDEX: usize = 1;
    pub const PARENT_Z_SIGN_INDEX: usize = 2;
}

impl ValidationRegex<Regex> for MsConvertMgfChargeRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            CHARGE=
            (?:
                [[:digit:]]+
            )
            (?:
                [+\-]
            )
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<Regex> for MsConvertMgfChargeRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            CHARGE=
            # Group 1, Parent Charge.
            (
                [[:digit:]]+
            )
            # Group 2, Charge Sign.
            (
                [+\-]
            )
            \z
        ");
        &REGEX
    }
}

// PAVA

/// Regular expression to validate and parse Pava MGF files.
pub struct PavaMgfRegex;

// TODO(ahuszagh)   Implement...
//impl PavaMgfRegex {
//    /// Hard-coded index fields for data extraction.
//}
//
//impl ValidationRegex<Regex> for MsConvertMgfRegex {
//    fn validate() -> &'static Regex {}
//}
//
//impl ExtractionRegex<Regex> for MsConvertMgfRegex {
//    fn extract() -> &'static Regex {}
//}

/// Regular expression to validate and parse Pava MGF title lines.
pub struct PavaMgfTitleRegex;

impl PavaMgfTitleRegex {
    /// Hard-coded index fields for data extraction.
    pub const NUM_INDEX: usize = 1;
    pub const RT_INDEX: usize = 2;
    pub const FILE_INDEX: usize = 3;
}

impl ValidationRegex<Regex> for PavaMgfTitleRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r##"(?x)
            \A
            TITLE=Scan\s
            (?:
                [[:digit:]]+
            )
            \s\(rt=
            (?:
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            \)
            \s\[
            (?:
                [^.="]+
            )
            (?:\.[^.="]+)?
            \]
            \z
        "##);
        &REGEX
    }
}

impl ExtractionRegex<Regex> for PavaMgfTitleRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r##"(?x)
            \A
            TITLE=Scan\s
            # Group 1, Scan Number.
            (
                [[:digit:]]+
            )
            \s\(rt=
            # Group 2, Retention Time.
            (
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            \)
            \s\[
            # Group 3, File Name.
            (
                [^.="]+
            )
            (?:\.[^.="]+)?
            \]
            \z
        "##);
        &REGEX
    }
}

// TODO(ahuszagh)
//  Add PepMass, Charge

// TODO(ahuszagh)
//  Add other MGF files...

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    // MSCONVERT

    #[test]
    fn msconvert_mgf_regex_test() {
        type T = MsConvertMgfRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        let text = "BEGIN IONS\nTITLE=Sample.33451.33451.5 File:\"Sample.raw\", NativeID:\"controllerType=0 controllerNumber=1 scan=33451\"\nRTINSECONDS=8692.81452\nPEPMASS=1197.992553710938\nCHARGE=5+\n";
        check_regex!(T, text, true);

        // extract
        extract_regex!(T, text, 1, "Sample", as_str);
        extract_regex!(T, text, 2, "33451", as_str);
        extract_regex!(T, text, 3, "8692.81452", as_str);
        extract_regex!(T, text, 4, "1197.992553710938", as_str);
        extract_regex!(T, text, 6, "5", as_str);
        extract_regex!(T, text, 7, "+", as_str);

        let text = "BEGIN IONS\nTITLE=Sample.33450.33450.4 File:\"Sample.raw\", NativeID:\"controllerType=0 controllerNumber=1 scan=33450\"\nRTINSECONDS=8692.657303\nPEPMASS=775.15625 170643.953125\nCHARGE=4+\n";
        extract_regex!(T, text, 1, "Sample", as_str);
        extract_regex!(T, text, 2, "33450", as_str);
        extract_regex!(T, text, 3, "8692.657303", as_str);
        extract_regex!(T, text, 4, "775.15625", as_str);
        extract_regex!(T, text, 5, "170643.953125", as_str);
        extract_regex!(T, text, 6, "4", as_str);
        extract_regex!(T, text, 7, "+", as_str);
    }

    #[test]
    fn msconvert_mgf_title_regex_test() {
        type T = MsConvertMgfTitleRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "TITLE=Sample.350.350.4 File:\"Sample.raw\", NativeID:\"controllerType=0 controllerNumber=1 scan=350\"", true);

        // invalid
        check_regex!(T, "TITLE=Sample=.350.350.4 File:\"Sample.raw\", NativeID:\"controllerType=0 controllerNumber=1 scan=350\"", false);
        check_regex!(T, "TITLE=Sam.ple.350.350.4 File:\"Sample.raw\", NativeID:\"controllerType=0 controllerNumber=1 scan=350\"", false);
        check_regex!(T, "TITLE=Sam\"ple.350.350.4 File:\"Sample.raw\", NativeID:\"controllerType=0 controllerNumber=1 scan=350\"", false);
        check_regex!(T, "TITLE=Sample.350X.350.4 File:\"Sample.raw\", NativeID:\"controllerType=0 controllerNumber=1 scan=350\"", false);
        check_regex!(T, "TITLE=Sample.350.350X.4 File:\"Sample.raw\", NativeID:\"controllerType=0 controllerNumber=1 scan=350\"", false);
        check_regex!(T, "TITLE=Sample.350.350.4X File:\"Sample.raw\", NativeID:\"controllerType=0 controllerNumber=1 scan=350\"", false);
        check_regex!(T, "TITLE=Sample.350.350.4 File:\"Sample.raw\", NativeID:\"controllerType=0X controllerNumber=1 scan=350\"", false);
        check_regex!(T, "TITLE=Sample.350.350.4 File:\"Sample.raw\", NativeID:\"controllerType=0 controllerNumber=1X scan=350\"", false);
        check_regex!(T, "TITLE=Sample.350.350.4 File:\"Sample.raw\", NativeID:\"controllerType=0 controllerNumber=1 scan=350X\"", false);

        // extract
        extract_regex!(T, "TITLE=Sample.350.350.4 File:\"Sample.raw\", NativeID:\"controllerType=0 controllerNumber=1 scan=350\"", 1, "Sample", as_str);
        extract_regex!(T, "TITLE=Sample.350.350.4 File:\"Sample.raw\", NativeID:\"controllerType=0 controllerNumber=1 scan=350\"", 2, "350", as_str);
    }

    #[test]
    fn msconvert_mgf_rt_regex_test() {
        type T = MsConvertMgfRtRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "RTINSECONDS=8692", true);
        check_regex!(T, "RTINSECONDS=8692.657303", true);

        // invalid
        check_regex!(T, "RTINSECONDS=8692.", false);
        check_regex!(T, "RTINSECONDS=8692X", false);
        check_regex!(T, "RTINSECONDS=8692.123X", false);

        // extract
        extract_regex!(T, "RTINSECONDS=8692", 1, "8692", as_str);
        extract_regex!(T, "RTINSECONDS=8692.657303", 1, "8692.657303", as_str);
    }

    #[test]
    fn msconvert_mgf_pepmass_regex_test() {
        type T = MsConvertMgfPepMassRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "PEPMASS=775", true);
        check_regex!(T, "PEPMASS=775.15625", true);
        check_regex!(T, "PEPMASS=775 170643.953125", true);
        check_regex!(T, "PEPMASS=775.15625 170643", true);
        check_regex!(T, "PEPMASS=775.15625 170643.953125", true);

        // invalid
        check_regex!(T, "PEPMASS=775.", false);
        check_regex!(T, "PEPMASS=775.15X", false);
        check_regex!(T, "PEPMASS=775. 170643.953125", false);
        check_regex!(T, "PEPMASS=775.15625 170643.", false);
        check_regex!(T, "PEPMASS=775.15625X 170643.953125", false);
        check_regex!(T, "PEPMASS=775.15625 170643.953125X", false);

        // extract
        extract_regex!(T, "PEPMASS=775", 1, "775", as_str);
        extract_regex!(T, "PEPMASS=775.15625", 1, "775.15625", as_str);
        extract_regex!(T, "PEPMASS=775 170643.953125", 1, "775", as_str);
        extract_regex!(T, "PEPMASS=775 170643.953125", 2, "170643.953125", as_str);
    }

    #[test]
    fn msconvert_mgf_charge_regex_test() {
        type T = MsConvertMgfChargeRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "CHARGE=4+", true);

        // invalid
        check_regex!(T, "CHARGE=4+X", false);
        check_regex!(T, "CHARGE=4X+", false);
        check_regex!(T, "CHARGE=4", false);

        // extract
        extract_regex!(T, "CHARGE=4+", 1, "4", as_str);
        extract_regex!(T, "CHARGE=4+", 2, "+", as_str);
    }

    // PAVA

    #[test]
    fn pava_mgf_regex_test() {
        // TODO(ahuszagh)   Implement
    }

    #[test]
    fn pava_mgf_title_regex_test() {
        type T = PavaMgfTitleRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "TITLE=Scan 749 (rt=14.112) [beta_orbi111015_06.raw]", true);

        // invalid
        check_regex!(T, "TITLE=Scan 749X (rt=14.112) [beta_orbi111015_06.raw]", false);
        check_regex!(T, "TITLE=Scan 749 (rt=14.) [beta_orbi111015_06.raw]", false);
        check_regex!(T, "TITLE=Scan 749 (rt=14.112) [beta.orbi111015_06.raw]", false);
        check_regex!(T, "TITLE=Scan 749 (rt=14.112) [beta=orbi111015_06.raw]", false);

        // extract
        extract_regex!(T, "TITLE=Scan 749 (rt=14.112) [beta_orbi111015_06.raw]", 1, "749", as_str);
        extract_regex!(T, "TITLE=Scan 749 (rt=14.112) [beta_orbi111015_06.raw]", 2, "14.112", as_str);
        extract_regex!(T, "TITLE=Scan 749 (rt=14.112) [beta_orbi111015_06.raw]", 3, "beta_orbi111015_06", as_str);
    }

    // TODO(ahuszagh)   Add more tests here.
}
