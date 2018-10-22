//! Model for SRA (Sequence Read Archive) read definitions.

//@SRR390728.1 1 length=72
//CATTCTTCACGTAGTTCTCGAGCCTTGGTTTTCAGCGATGGAGAATGACTTTGACAAGCTGAGAGAAGNTNC
//+SRR390728.1 1 length=72
//;;;;;;;;;;;;;;;;;;;;;;;;;;;9;;665142;;;;;;;;;;;;;;;;;;;;;;;;;;;;;96&&&&(
//@SRR390728.2 2 length=72
//AAGTAGGTCTCGTCTGTGTTTTCTACGAGCTTGTGTTCCAGCTGACCCACTCCCTGGGTGGGGGGACTGGGT
//+SRR390728.2 2 length=72
//;;;;;;;;;;;;;;;;;4;;;;3;393.1+4&&5&&;;;;;;;;;;;;;;;;;;;;;<9;<;;;;;464262
//@SRR390728.3 3 length=72
//CCAGCCTGGCCAACAGAGTGTTACCCCGTTTTTACTTATTTATTATTATTATTTTGAGACAGAGCATTGGTC
//+SRR390728.3 3 length=72
//-;;;8;;;;;;;,*;;';-4,44;,:&,1,4'./&19;;;;;;669;;99;;;;;-;3;2;0;+;7442&2/
//@SRR390728.4 4 length=72
//ATAAAATCAGGGGTGTTGGAGATGGGATGCCTATTTCTGCACACCTTGGCCTCCCAAATTGCTGGGATTACA
//+SRR390728.4 4 length=72
//1;;;;;;,;;4;3;38;8%&,,;)*;1;;,)/%4+,;1;;);;;;;;;4;(;1;;;;24;;;;41-444//0
//@SRR390728.5 5 length=72
//TTAAGAAATTTTTGCTCAAACCATGCCCTAAAGGGTTCTGTAATAAATAGGGCTGGGAAAACTGGCAAGCCA
//+SRR390728.5 5 length=72
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;9445552;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;446662

// Look at the above
// https://www.ncbi.nlm.nih.gov/books/NBK158899/

/// Enumerated values for Record fields.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[allow(dead_code)]     // TODO(ahuszagh)       Remove
pub enum RecordField {
    SeqId,
    Description,
    Length,
    Sequence,
    Quality,
}


/// Model for a single record from a sequence read.
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Record {
    /// Sequence identifier for the read.
    pub seq_id: String,
    /// Description for the sequence identifier.
    pub description: String,
    /// Read length.
    pub length: u32,
    /// Nucleotide sequence.
    pub sequence: Vec<u8>,
    /// Nucleotide sequence quality scores.
    pub quality: Vec<u8>,
}

// TODO(ahuszagh)
//  Implement re, valid, and complete models for this.
