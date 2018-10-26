//! Valid trait implementation for mass spectral models.

use traits::Valid;
use super::record::Record;
use super::record_list::RecordList;

impl Valid for Record {
    fn is_valid(&self) -> bool {
        (
            self.num != 0 &&
            self.rt >= 0.0 &&
            self.parent_mz >= 0.0 &&
            self.parent_intensity >= 0.0 &&
            self.parent_z != 0 &&
            !self.peaks.is_empty()
        )
    }
}

impl Valid for RecordList {
    #[inline]
    fn is_valid(&self) -> bool {
        self.iter().all(|ref x| x.is_valid())
    }
}