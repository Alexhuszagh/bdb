//! Mass spectrum integrations.

// Expose the low-level API in a public submodule.
pub mod low_level;

mod complete;
mod peak;
mod peak_list;
mod re;
mod record;
mod record_list;
mod valid;

cfg_if! {
    if #[cfg(feature = "mgf")] {
        mod mgf;
        mod fullms_mgf;
        mod msconvert_mgf;
        mod pava_mgf;
        mod pwiz_mgf;
    }
}

#[cfg(test)]
mod test;

// Re-export the models into the parent module.
pub use self::peak::Peak;
pub use self::peak_list::PeakList;
pub use self::record::Record;
pub use self::record_list::RecordList;
