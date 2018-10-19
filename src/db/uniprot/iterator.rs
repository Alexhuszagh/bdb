//! Trait for iterators over UniProt records.

use std::fmt::Debug;

/// Marker trait for the base record iterator.
pub trait RecordIterator: Debug + Iterator {
}

/// Marker trait for the value iterator, which produces records by-value.
pub trait ValueRecordIterator: RecordIterator {
}

/// Marker trait for the output iterator, which saves records by reference.
pub trait ReferenceRecordIterator: RecordIterator {
}
