//! Shared helper utilities for mass spectra unit testing.

use super::peak::Peak;
use super::record::Record;

// RECORDS

/// Create a record from a sample MSConvert scan.
pub fn msconvert_33450() -> Record {
    Record {
        num: 33450,
        ms_level: 0,
        rt: 8692.657303,
        parent_mz: 775.15625,
        parent_intensity: 170643.953125,
        parent_z: 4,
        file: String::from("QPvivo_2015_11_10_1targetmethod"),
        filter: String::from(""),
        peaks: vec![
            Peak { mz: 205.9304178, intensity: 0.0, z: 0 },
            Peak { mz: 205.9320046, intensity: 0.0, z: 0 },
            Peak { mz: 205.9335913, intensity: 0.0, z: 0 },
            Peak { mz: 205.9351781, intensity: 0.0, z: 0 },
            Peak { mz: 257.514984, intensity: 0.0, z: 0 },
            Peak { mz: 257.5172029, intensity: 0.0, z: 0 },
            Peak { mz: 257.5194218, intensity: 0.0, z: 0 },
            Peak { mz: 257.5216407, intensity: 0.0, z: 0 },
            Peak { mz: 257.5238596, intensity: 457.499206543, z: 0 },
            Peak { mz: 257.5260786, intensity: 742.1607666016, z: 0 },
            Peak { mz: 257.5282976, intensity: 832.3284301758, z: 0 },
            Peak { mz: 257.5305166, intensity: 666.099609375, z: 0 },
            Peak { mz: 257.5327357, intensity: 353.6197509766, z: 0 },
            Peak { mz: 257.5349181, intensity: 0.0, z: 0 },
            Peak { mz: 257.5371372, intensity: 0.0, z: 0 },
            Peak { mz: 257.5393564, intensity: 0.0, z: 0 },
            Peak { mz: 257.5415756, intensity: 0.0, z: 0 },
            Peak { mz: 266.3775252, intensity: 0.0, z: 0 },
            Peak { mz: 266.3798596, intensity: 0.0, z: 0 },
            Peak { mz: 266.382194, intensity: 0.0, z: 0 },
            Peak { mz: 266.3845284, intensity: 0.0, z: 0 },
            Peak { mz: 266.3868629, intensity: 395.335723877, z: 0 },
            Peak { mz: 266.3891974, intensity: 687.4059448242, z: 0 },
            Peak { mz: 266.3915319, intensity: 839.1334228516, z: 0 },
            Peak { mz: 266.3938665, intensity: 753.7129516602, z: 0 },
            Peak { mz: 266.3962011, intensity: 483.698425293, z: 0 },
            Peak { mz: 266.3985627, intensity: 0.0, z: 0 },
            Peak { mz: 266.4008973, intensity: 0.0, z: 0 },
            Peak { mz: 266.403232, intensity: 0.0, z: 0 },
            Peak { mz: 266.4055668, intensity: 0.0, z: 0 },
            Peak { mz: 274.490484, intensity: 0.0, z: 0 },
            Peak { mz: 274.4929259, intensity: 0.0, z: 0 },
            Peak { mz: 274.4953677, intensity: 0.0, z: 0 },
            Peak { mz: 274.4978097, intensity: 0.0, z: 0 },
            Peak { mz: 274.5002516, intensity: 359.3305664063, z: 0 },
            Peak { mz: 274.5026936, intensity: 691.2191162109, z: 0 },
            Peak { mz: 274.5051356, intensity: 1342.998046875, z: 0 },
            Peak { mz: 274.5075776, intensity: 1104.1827392578, z: 0 },
            Peak { mz: 274.5100197, intensity: 459.472442627, z: 0 },
            Peak { mz: 274.5124333, intensity: 0.0, z: 0 },
            Peak { mz: 274.5148754, intensity: 0.0, z: 0 },
            Peak { mz: 274.5173176, intensity: 0.0, z: 0 },
            Peak { mz: 274.5197598, intensity: 0.0, z: 0 },
            Peak { mz: 288.185445, intensity: 0.0, z: 0 },
            Peak { mz: 288.1880718, intensity: 0.0, z: 0 },
            Peak { mz: 288.1906987, intensity: 0.0, z: 0 },
            Peak { mz: 288.1933256, intensity: 0.0, z: 0 },
            Peak { mz: 288.1959526, intensity: 513.036315918, z: 0 },
            Peak { mz: 288.1985796, intensity: 1173.0286865234, z: 0 },
            Peak { mz: 288.2012066, intensity: 1705.58203125, z: 0 },
            Peak { mz: 288.2038337, intensity: 1740.2529296875, z: 0 },
            Peak { mz: 288.2064608, intensity: 1205.7132568359, z: 0 },
            Peak { mz: 288.2090879, intensity: 441.4267272949, z: 0 },
            Peak { mz: 288.2116643, intensity: 0.0, z: 0 },
            Peak { mz: 288.2142915, intensity: 0.0, z: 0 },
            Peak { mz: 288.2169188, intensity: 0.0, z: 0 },
            Peak { mz: 288.219546, intensity: 0.0, z: 0 },
            Peak { mz: 296.4551094, intensity: 0.0, z: 0 },
            Peak { mz: 296.4578501, intensity: 0.0, z: 0 },
            Peak { mz: 296.4605908, intensity: 0.0, z: 0 },
            Peak { mz: 296.4633316, intensity: 0.0, z: 0 },
            Peak { mz: 296.4660725, intensity: 195.8185119629, z: 0 },
            Peak { mz: 296.4688134, intensity: 706.2313232422, z: 0 },
            Peak { mz: 296.4715543, intensity: 1314.5838623047, z: 0 },
            Peak { mz: 296.4742952, intensity: 1367.2843017578, z: 0 },
            Peak { mz: 296.4770362, intensity: 595.6688842773, z: 0 },
            Peak { mz: 296.4797232, intensity: 0.0, z: 0 },
            Peak { mz: 296.4824643, intensity: 0.0, z: 0 },
            Peak { mz: 296.4852054, intensity: 0.0, z: 0 }],
        parent: vec![],
        children: vec![],
    }
}

/// Create a sample empty MSConvert scan.
pub fn msconvert_empty() -> Record {
    Record {
        num: 33450,
        ms_level: 0,
        rt: 8692.657303,
        parent_mz: 775.15625,
        parent_intensity: 170643.953125,
        parent_z: 4,
        file: String::from("QPvivo_2015_11_10_1targetmethod"),
        filter: String::from(""),
        peaks: vec![],
        parent: vec![],
        children: vec![]
    }
}

// MGF

/// Constant string for the GAPDH FASTA export.
#[cfg(feature = "mgf")]
pub const MSCONVERT_33450_MGF: &'static str = "BEGIN IONS\nTITLE=QPvivo_2015_11_10_1targetmethod.33450.33450.0 File:\"QPvivo_2015_11_10_1targetmethod\", NativeID:\"controllerType=0 controllerNumber=1 scan=33450\"\nRTINSECONDS=8692.657303\nPEPMASS=775.15625 170643.953125\nCHARGE=4+\n205.9304178 0.0\n205.9320046 0.0\n205.9335913 0.0\n205.9351781 0.0\n257.514984 0.0\n257.5172029 0.0\n257.5194218 0.0\n257.5216407 0.0\n257.5238596 457.499206543\n257.5260786 742.1607666016\n257.5282976 832.3284301758\n257.5305166 666.099609375\n257.5327357 353.6197509766\n257.5349181 0.0\n257.5371372 0.0\n257.5393564 0.0\n257.5415756 0.0\n266.3775252 0.0\n266.3798596 0.0\n266.382194 0.0\n266.3845284 0.0\n266.3868629 395.335723877\n266.3891974 687.4059448242\n266.3915319 839.1334228516\n266.3938665 753.7129516602\n266.3962011 483.698425293\n266.3985627 0.0\n266.4008973 0.0\n266.403232 0.0\n266.4055668 0.0\n274.490484 0.0\n274.4929259 0.0\n274.4953677 0.0\n274.4978097 0.0\n274.5002516 359.3305664063\n274.5026936 691.2191162109\n274.5051356 1342.998046875\n274.5075776 1104.1827392578\n274.5100197 459.472442627\n274.5124333 0.0\n274.5148754 0.0\n274.5173176 0.0\n274.5197598 0.0\n288.185445 0.0\n288.1880718 0.0\n288.1906987 0.0\n288.1933256 0.0\n288.1959526 513.036315918\n288.1985796 1173.0286865234\n288.2012066 1705.58203125\n288.2038337 1740.2529296875\n288.2064608 1205.7132568359\n288.2090879 441.4267272949\n288.2116643 0.0\n288.2142915 0.0\n288.2169188 0.0\n288.219546 0.0\n296.4551094 0.0\n296.4578501 0.0\n296.4605908 0.0\n296.4633316 0.0\n296.4660725 195.8185119629\n296.4688134 706.2313232422\n296.4715543 1314.5838623047\n296.4742952 1367.2843017578\n296.4770362 595.6688842773\n296.4797232 0.0\n296.4824643 0.0\n296.4852054 0.0\nEND IONS\n";

#[cfg(feature = "mgf")]
pub const MSCONVERT_EMPTY_MGF: &'static str = "BEGIN IONS\nTITLE=QPvivo_2015_11_10_1targetmethod.33450.33450.0 File:\"QPvivo_2015_11_10_1targetmethod\", NativeID:\"controllerType=0 controllerNumber=1 scan=33450\"\nRTINSECONDS=8692.657303\nPEPMASS=775.15625 170643.953125\nCHARGE=4+\nEND IONS\n";
