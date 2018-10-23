//! Shared helper utilities for SRA unit testing.

use super::record::Record;

// RECORDS

/// Create a record for an SRA sample.
pub fn srr390728_2() -> Record {
    Record {
        seq_id: String::from("SRR390728.2"),
        description: String::from("2"),
        length: 72,
        sequence: b"AAGTAGGTCTCGTCTGTGTTTTCTACGAGCTTGTGTTCCAGCTGACCCACTCCCTGGGTGGGGGGACTGGGT".to_vec(),
        quality: b";;;;;;;;;;;;;;;;;4;;;;3;393.1+4&&5&&;;;;;;;;;;;;;;;;;;;;;<9;<;;;;;464262".to_vec(),
    }
}

/// Create a record for an SRA sample.
pub fn srr390728_3() -> Record {
    Record {
        seq_id: String::from("SRR390728.3"),
        description: String::from("3"),
        length: 72,
        sequence: b"CCAGCCTGGCCAACAGAGTGTTACCCCGTTTTTACTTATTTATTATTATTATTTTGAGACAGAGCATTGGTC".to_vec(),
        quality: b"-;;;8;;;;;;;,*;;';-4,44;,:&,1,4'./&19;;;;;;669;;99;;;;;-;3;2;0;+;7442&2/".to_vec(),
    }
}
