//! Shared helper utilities for unit testing.

use std::env;
use std::path::PathBuf;

// PATH

/// Return the `target/debug` directory path.
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
pub fn target_dir() -> PathBuf {
    debug_dir()
        .parent()
        .expect("target directory")
        .to_path_buf()
}

/// Return the project directory path.
pub fn project_dir() -> PathBuf {
    target_dir()
        .parent()
        .expect("project directory")
        .to_path_buf()
}

/// Return the `test` directory path.
pub fn test_dir() -> PathBuf {
    let mut dir = project_dir();
    dir.push("test");
    dir
}

/// Return the `test/data` directory path.
pub fn testdata_dir() -> PathBuf {
    let mut dir = test_dir();
    dir.push("data");
    dir
}

// REGEX

/// Check regex validates or does not validate text.
macro_rules! validate_regex {
    ($t:tt, $input:expr, $expected:expr) => ({
        assert_eq!($t::validate().is_match($input), $expected)
    })
}

/// Check regex matches or does not match text.
macro_rules! check_regex {
    ($t:tt, $input:expr, $expected:expr) => ({
        assert_eq!($t::validate().is_match($input), $expected);
        assert_eq!($t::extract().is_match($input), $expected)
    })
}


/// Check regex extracts the desired subgroup.
macro_rules! extract_regex {
    ($t:tt, $input:expr, $index:expr, $expected:expr, $meth:ident) => ({
        let re = $t::extract();
        let caps = re.captures($input).unwrap();
        assert_eq!(caps.get($index).unwrap().$meth(), $expected);
    })
}
