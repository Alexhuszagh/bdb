//! Valid trait implementation for SRA models.

use traits::Valid;
use super::re::*;
use super::record::Record;
use super::record_list::RecordList;

impl Valid for Record {
    fn is_valid(&self) -> bool {
        (
            !self.seq_id.is_empty() &&
            self.length as usize == self.sequence.len() &&
            self.length as usize == self.quality.len() &&
            NucleotideRegex::validate().is_match(&self.sequence) &&
            SequenceQualityRegex::validate().is_match(&self.quality)
        )
    }
}

impl Valid for RecordList {
    #[inline]
    fn is_valid(&self) -> bool {
        self.iter().all(|ref x| x.is_valid())
    }
}
