//! Shared helper utilities for unit testing.

use std::env;
use std::path::PathBuf;

/// Return the `target/debug` directory path.
#[cfg(test)]
pub fn debug_dir() -> PathBuf {
    env::current_exe()
        .expect("unittest executable path")
        .parent()
        .expect("unittest executable directory")
        .parent()
        .expect("debug directory")
        .to_path_buf()
}

/// Return the `target` directory path.
#[cfg(test)]
pub fn target_dir() -> PathBuf {
    debug_dir()
        .parent()
        .expect("target directory")
        .to_path_buf()
}

/// Return the project directory path.
#[cfg(test)]
pub fn project_dir() -> PathBuf {
    target_dir()
        .parent()
        .expect("project directory")
        .to_path_buf()
}

/// Return the `test` directory path.
#[cfg(test)]
pub fn test_dir() -> PathBuf {
    let mut dir = project_dir();
    dir.push("test");
    dir
}

/// Return the `test/data` directory path.
#[cfg(test)]
pub fn testdata_dir() -> PathBuf {
    let mut dir = test_dir();
    dir.push("data");
    dir
}
