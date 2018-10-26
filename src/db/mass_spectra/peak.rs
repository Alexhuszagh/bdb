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
    // TODO(ahuszagh)   Implement...
}
