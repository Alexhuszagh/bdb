//! Model for mass spectra peak definitions.

/// Model for a spectral peak.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Peak {
    /// Mass to charge ratio.
    pub mz: f64,
    /// Maximum intensity.
    pub intensity: f64,
    /// Charge state of the ion.
    pub z: i8,
}

impl Peak {
    /// Create new, empty spectral peak.
    #[inline]
    pub fn new() -> Self {
        Peak {
            mz: 0.0,
            intensity: 0.0,
            z: 0,
        }
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_peak() {
        let peak = Peak { mz: 257.1, intensity: 457.5, z: 1 };
        let text = format!("{:?}", peak);
        assert_eq!(text, "Peak { mz: 257.1, intensity: 457.5, z: 1 }");
    }

    #[test]
    fn equality_peak() {
        let x = Peak { mz: 257.1, intensity: 457.5, z: 1 };
        let y = Peak { mz: 257.1, intensity: 457.5, z: 1 };
        let z = Peak { mz: 257.1, intensity: 457.5, z: 2 };
        assert_eq!(x, y);
        assert_ne!(x, z);
        assert_ne!(y, z);
    }
}
