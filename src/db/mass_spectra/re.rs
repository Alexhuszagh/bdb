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

/// Regular expression to valid and parse MSConvert MGF files.
pub struct MsConvertMgfRegex;

impl MsConvertMgfRegex {
    /// Hard-coded index fields for data extraction.
    pub const FILE_INDEX: usize = 1;
    pub const SCAN_INDEX: usize = 2;
    pub const RT_INDEX: usize = 3;
    pub const PARENT_MZ_INDEX: usize = 4;
    pub const PARENT_INTENSITY_INDEX: usize = 5;
    pub const CHARGE_INDEX: usize = 6;
    pub const CHARGE_SIGN_INDEX: usize = 7;
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
            \sFile:"[^.="]+\.[^.="]+",\sNativeID:"
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
                [[:digit:]]+(?:\.[[:digit:]]+)
            )
            \r?\n

            # Line 4, Pep Mass.
            PEPMASS=
            (?:
                [[:digit:]]+(?:\.[[:digit:]]+)
            )
            (?:
                \s
                (?:
                    [[:digit:]]+(?:\.[[:digit:]]+)
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
            \sFile:"[^.="]+\.[^.="]+",\sNativeID:"
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
                [[:digit:]]+(?:\.[[:digit:]]+)
            )
            \r?\n

            # Line 4, Pep Mass.
            PEPMASS=
            # Group 4, Parent M/Z.
            (
                [[:digit:]]+(?:\.[[:digit:]]+)
            )
            (?:
                \s
                # Group 5, Parent Intensity.
                (
                    [[:digit:]]+(?:\.[[:digit:]]+)
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

// TODO(ahuszagh)
//  Add other MGF files...

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn msconvert_mgf_regex_test() {
        type T = MsConvertMgfRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        let text = "BEGIN IONS\nTITLE=QPvivo_2015_11_10_1targetmethod.33451.33451.5 File:\"QPvivo_2015_11_10_1targetmethod.raw\", NativeID:\"controllerType=0 controllerNumber=1 scan=33451\"\nRTINSECONDS=8692.81452\nPEPMASS=1197.992553710938\nCHARGE=5+\n";
        check_regex!(T, text, true);
    }
}
