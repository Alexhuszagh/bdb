//! Complete trait implementation for UniProt models.

use traits::{Complete, Valid};
use super::record::Record;
use super::record_list::RecordList;


impl Complete for Record {
    #[inline]
    fn is_complete(&self) -> bool {
        (
            self.is_valid() &&
            self.ms_level != 0 &&
            !self.filter.is_empty()
        )
    }
}

impl Complete for RecordList {
    #[inline]
    fn is_complete(&self) -> bool {
        self.iter().all(|ref x| x.is_complete())
    }
}
