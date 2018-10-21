//! Input and output utilities.
//!
//! High-level input and output utilities for models. These utilities
//! may suffer from low-performance and excess memory usage, since
//! they require all elements to be in memory at a given time.
//!
//! If you would like to use more efficient utilities, at the expense
//! of code complexity, look at the low-level APIs re-exported in each
//! model under `db`.

pub mod uniprot;
