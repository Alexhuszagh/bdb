//! Regular expression utilities for mass spectral services.
//!
//! Disable Unicode for all but the generic header formats, which may
//! accept arbitrary Unicode input. The rest should only be valid ASCII,
//! and therefore we should disable matching to Unicode characters
//! explicitly.

use regex::Regex;

// Re-export regular-expression traits.
pub(crate) use util::{ExtractionRegex, ValidationRegex};

// FULL MS

/// Regular expression to validate and parse Pava FullMs MGF scan lines.
pub struct FullMsMgfScanRegex;

impl FullMsMgfScanRegex {
    /// Hard-coded index fields for data extraction.
    pub const NUM_INDEX: usize = 1;
}

impl ValidationRegex<Regex> for FullMsMgfScanRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            Scan\#:\s
            (?:
                [[:digit:]]+
            )
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<Regex> for FullMsMgfScanRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            Scan\#:\s
            # Group 1, Scan Number.
            (
                [[:digit:]]+
            )
            \z
        ");
        &REGEX
    }
}

/// Regular expression to validate and parse Pava FullMs MGF RT lines.
pub struct FullMsMgfRtRegex;

impl FullMsMgfRtRegex {
    /// Hard-coded index fields for data extraction.
    pub const RT_INDEX: usize = 1;
}

impl ValidationRegex<Regex> for FullMsMgfRtRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            Ret\.Time:\s
            (?:
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<Regex> for FullMsMgfRtRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            Ret\.Time:\s
            # Group 1, Retention Time.
            (
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            \z
        ");
        &REGEX
    }
}

// MSCONVERT

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

/// Regular expression to validate and parse Pava MGF pepmass lines.
pub struct PavaMgfPepMassRegex;

impl PavaMgfPepMassRegex {
    /// Hard-coded index fields for data extraction.
    pub const PARENT_MZ_INDEX: usize = 1;
    pub const PARENT_INTENSITY_INDEX: usize = 2;
}

impl ValidationRegex<Regex> for PavaMgfPepMassRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            PEPMASS=
            (?:
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            (?:
                \t
                (?:
                    [[:digit:]]+(?:\.[[:digit:]]+)?
                )
            )?
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<Regex> for PavaMgfPepMassRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            PEPMASS=
            # Group 1, Parent M/Z.
            (
                [[:digit:]]+(?:\.[[:digit:]]+)?
            )
            (?:
                \t
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

/// Regular expression to validate and parse Pava MGF charge lines.
pub struct PavaMgfChargeRegex;

impl PavaMgfChargeRegex {
    /// Hard-coded index fields for data extraction.
    pub const PARENT_Z_INDEX: usize = 1;
    pub const PARENT_Z_SIGN_INDEX: usize = 2;
}

impl ValidationRegex<Regex> for PavaMgfChargeRegex {
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

impl ExtractionRegex<Regex> for PavaMgfChargeRegex {
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

// PWIZ

/// Regular expression to validate and parse Pwiz MGF title lines.
pub struct PwizMgfTitleRegex;

impl PwizMgfTitleRegex {
    /// Hard-coded index fields for data extraction.
    pub const FILE_INDEX: usize = 1;
    pub const NUM_INDEX: usize = 2;
}

impl ValidationRegex<Regex> for PwizMgfTitleRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            TITLE=
            (?:
                [^\x20]+
            )
            \sSpectrum[0-9]+\sscans:\s
            (?:
                [[:digit:]]+
            )
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<Regex> for PwizMgfTitleRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            TITLE=
            # Group 1, File Name.
            (
                [^\x20]+
            )
            \sSpectrum[0-9]+\sscans:\s
            # Group 1, Scan Number.
            (
                [[:digit:]]+
            )
            \z
        ");
        &REGEX
    }
}

/// Regular expression to validate and parse Pwiz MGF pepmass lines.
pub struct PwizMgfPepMassRegex;

impl PwizMgfPepMassRegex {
    /// Hard-coded index fields for data extraction.
    pub const PARENT_MZ_INDEX: usize = 1;
    pub const PARENT_INTENSITY_INDEX: usize = 2;
}

impl ValidationRegex<Regex> for PwizMgfPepMassRegex {
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

impl ExtractionRegex<Regex> for PwizMgfPepMassRegex {
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

/// Regular expression to validate and parse Pwiz MGF charge lines.
pub struct PwizMgfChargeRegex;

impl PwizMgfChargeRegex {
    /// Hard-coded index fields for data extraction.
    pub const PARENT_Z_INDEX: usize = 1;
    pub const PARENT_Z_SIGN_INDEX: usize = 2;
}

impl ValidationRegex<Regex> for PwizMgfChargeRegex {
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

impl ExtractionRegex<Regex> for PwizMgfChargeRegex {
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

/// Regular expression to validate and parse Pwiz MGF RT lines.
pub struct PwizMgfRtRegex;

impl PwizMgfRtRegex {
    /// Hard-coded index fields for data extraction.
    pub const RT_INDEX: usize = 1;
}

impl ValidationRegex<Regex> for PwizMgfRtRegex {
    fn validate() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            RTINSECONDS=
            (?:
                [[:digit:]]+
            )
            \z
        ");
        &REGEX
    }
}

impl ExtractionRegex<Regex> for PwizMgfRtRegex {
    fn extract() -> &'static Regex {
        lazy_regex!(Regex, r"(?x)
            \A
            RTINSECONDS=
            # Group 1, Retention Time.
            (
                [[:digit:]]+
            )
            \z
        ");
        &REGEX
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    // FULLMS

    #[test]
    fn fullms_mgf_scan_regex_test() {
        type T = FullMsMgfScanRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "Scan#: 2182", true);

        // invalid
        check_regex!(T, "Scan: 2182", false);
        check_regex!(T, "Scan# 2182", false);
        check_regex!(T, "Scan#: X2182", false);

        // extract
        extract_regex!(T, "Scan#: 2182", 1, "2182", as_str);
    }

    #[test]
    fn fullms_mgf_rt_regex_test() {
        type T = FullMsMgfRtRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "Ret.Time: 8692", true);
        check_regex!(T, "Ret.Time: 8692.657303", true);

        // invalid
        check_regex!(T, "Ret.Time: 8692.", false);
        check_regex!(T, "RetTime: 8692.657303", false);
        check_regex!(T, "Ret.Time: 8692X", false);
        check_regex!(T, "Ret.Time: 8692.123X", false);

        // extract
        extract_regex!(T, "Ret.Time: 8692", 1, "8692", as_str);
        extract_regex!(T, "Ret.Time: 8692.657303", 1, "8692.657303", as_str);
    }

    // MSCONVERT

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

    #[test]
    fn pava_mgf_pepmass_regex_test() {
        type T = PavaMgfPepMassRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "PEPMASS=775", true);
        check_regex!(T, "PEPMASS=775.15625", true);
        check_regex!(T, "PEPMASS=775\t170643.953125", true);
        check_regex!(T, "PEPMASS=775.15625\t170643", true);
        check_regex!(T, "PEPMASS=775.15625\t170643.953125", true);

        // invalid
        check_regex!(T, "PEPMASS=775.", false);
        check_regex!(T, "PEPMASS=775.15X", false);
        check_regex!(T, "PEPMASS=775.\t170643.953125", false);
        check_regex!(T, "PEPMASS=775.15625\t170643.", false);
        check_regex!(T, "PEPMASS=775.15625X\t170643.953125", false);
        check_regex!(T, "PEPMASS=775.15625\t170643.953125X", false);

        // extract
        extract_regex!(T, "PEPMASS=775", 1, "775", as_str);
        extract_regex!(T, "PEPMASS=775.15625", 1, "775.15625", as_str);
        extract_regex!(T, "PEPMASS=775\t170643.953125", 1, "775", as_str);
        extract_regex!(T, "PEPMASS=775\t170643.953125", 2, "170643.953125", as_str);
    }

    #[test]
    fn pava_mgf_charge_regex_test() {
        type T = PavaMgfChargeRegex;

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

    // PWIZ

    #[test]
    fn pwiz_mgf_title_regex_test() {
        type T = PwizMgfTitleRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "TITLE=File73 Spectrum1 scans: 750", true);

        // invalid
        check_regex!(T, "TITLE=File73 Spectrum1X scans: 750", false);
        check_regex!(T, "TITLE=File73 Spectrum1 scans: 750X", false);
        check_regex!(T, "TITLE=File73 Spectrum1]tscans: 750", false);

        // extract
        extract_regex!(T, "TITLE=File73 Spectrum1 scans: 750", 1, "File73", as_str);
        extract_regex!(T, "TITLE=File73 Spectrum1 scans: 750", 2, "750", as_str);
    }

    #[test]
    fn pwiz_mgf_pepmass_regex_test() {
        type T = PwizMgfPepMassRegex;

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
    fn pwiz_mgf_charge_regex_test() {
        type T = PwizMgfChargeRegex;

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

    #[test]
    fn pwiz_mgf_rt_regex_test() {
        type T = PwizMgfRtRegex;

        // empty
        check_regex!(T, "", false);

        // valid
        check_regex!(T, "RTINSECONDS=8692", true);

        // invalid
        check_regex!(T, "RTINSECONDS=8692.", false);
        check_regex!(T, "RTINSECONDS=8692.657303", false);
        check_regex!(T, "RTINSECONDS=8692X", false);
        check_regex!(T, "RTINSECONDS=8692.123X", false);

        // extract
        extract_regex!(T, "RTINSECONDS=8692", 1, "8692", as_str);
    }
}
