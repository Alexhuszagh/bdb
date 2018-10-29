//! Mass spectrum integrations.

// Expose the low-level API in a public submodule.
pub mod low_level;

pub(crate) mod complete;
pub(crate) mod peak;
pub(crate) mod peak_list;
pub(crate) mod re;
pub(crate) mod record;
pub(crate) mod record_list;
pub(crate) mod valid;

cfg_if! {
    if #[cfg(feature = "mgf")] {
        pub(crate) mod mgf;
        pub(crate) mod fullms_mgf;
        pub(crate) mod msconvert_mgf;
        pub(crate) mod pava_mgf;
        pub(crate) mod pwiz_mgf;
    }
}

#[cfg(test)]
pub(crate) mod test;

// Re-export the models into the parent module.
pub use self::peak::Peak;
pub use self::peak_list::PeakList;
pub use self::record::Record;
pub use self::record_list::RecordList;
