//! Model for spectral collections.

use super::peak::Peak;

/// Spectral peak collection type.
pub type PeakList = Vec<Peak>;

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_peak_list() {
        let v = vec![
            Peak { mz: 257.5, intensity: 457.5, z: 1 },
            Peak { mz: 257.6, intensity: 742.2, z: 1 },
        ];
        let text = format!("{:?}", v);
        assert_eq!(text, "[Peak { mz: 257.5, intensity: 457.5, z: 1 }, Peak { mz: 257.6, intensity: 742.2, z: 1 }]");
    }

    #[test]
    fn equality_peak_list() {
        let p1 = Peak { mz: 257.5, intensity: 457.5, z: 1 };
        let p2 = Peak { mz: 257.6, intensity: 742.2, z: 1 };
        let x = vec![p1.clone(), p2.clone()];
        let y = vec![p1.clone(), p2.clone()];
        let z = vec![p2.clone(), p1.clone()];
        assert_eq!(x, y);
        assert_ne!(x, z);
        assert_ne!(y, z);
    }
}
