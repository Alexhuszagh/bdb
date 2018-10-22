//! Re-exports for low-level, efficient APIs.
//!
//! In order for high-performance processing of large documents,
//! We must use parsers that lazily read and write items to and from
//! documents. The writers accept both by-value and by-reference
//! iterators, allowing you to easily chain lazy readers and writers
//! to convert between export formats.
//!
//! The memory footprint of these lazy low-level functions is minimal,
//! typically < 16 KB required for internal buffers, and < 1 KB for each
//! individual item.

#[cfg(feature = "fastq")]
pub use super::fastq::*;
