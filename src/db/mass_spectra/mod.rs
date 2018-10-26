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

#[cfg(feature = "mgf")]
mod mgf;

#[cfg(feature = "mgf")]
mod msconvert_mgf;

#[cfg(test)]
mod test;

// Re-export the models into the parent module.
pub use self::peak::Peak;
pub use self::peak_list::PeakList;
pub use self::record::Record;
pub use self::record_list::RecordList;
