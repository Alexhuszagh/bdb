//! Shared iterator templates and utilities.

use traits::Valid;
use super::alias::ResultType;
use super::error::ErrorKind;

/// Iterator which raises an error for invalid items.
pub struct StrictIter<T: Valid, U: Iterator<Item = ResultType<T>>> {
    /// Wrapped internal iterator.
    iter: U,
}

impl<T: Valid, U: Iterator<Item = ResultType<T>>> StrictIter<T, U> {
    /// Create new StrictIter from a buffered reader.
    #[inline]
    pub fn new(iter: U) -> Self {
        StrictIter {
            iter: iter
        }
    }
}

impl<T: Valid, U: Iterator<Item = ResultType<T>>> Iterator for StrictIter<T, U> {
    type Item = U::Item;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next()?.and_then(|r| {
            match r.is_valid() {
                true    => Ok(r),
                false   => Err(From::from(ErrorKind::InvalidRecord)),
            }
        }))
    }
}

/// Iterator which ignores invalid items.
pub struct LenientIter<T: Valid, U: Iterator<Item = ResultType<T>>> {
    /// Wrapped internal iterator.
    iter: U,
}

impl<T: Valid, U: Iterator<Item = ResultType<T>>> LenientIter<T, U> {
    /// Create new LenientIter from a buffered reader.
    #[inline]
    pub fn new(iter: U) -> Self {
        LenientIter {
            iter: iter
        }
    }
}

impl<T: Valid, U: Iterator<Item = ResultType<T>>> Iterator for LenientIter<T, U> {
    type Item = U::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next()? {
                Err(e)  => return Some(Err(e)),
                Ok(r)   => {
                    if r.is_valid() {
                        return Some(Ok(r));
                    }
                },
            }
        }
    }
}
