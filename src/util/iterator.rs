//! Shared iterator templates and utilities.

use std::io::prelude::*;

use traits::Valid;
use super::alias::ResultType;
use super::error::ErrorKind;

// READER

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

// WRITER

// These are extremely low-level helpers to facilitate writing
// iterators to an export format. They take a few specific pieces
// of information:
//
// 1. A generic iterator, returning either `&Record` or `Result<Record>`.
// 2. A generic writer, implementing `Write`.
// 3. A delimiter to separate records or items.
// 4. A callback which converts the writer to an inner writer.
// 5. A callback which exports a record using the inner writer.
// 6. A callback which ends the inner writer.

/// Default exporter from a non-owning iterator.
pub fn reference_iterator_export<
    'a, 'b,
    Iter,
    Writer,
    InnerWriter,
    Record,
    InitCb,
    ExportCb,
    DestCb
>
(
    writer: &'b mut Writer,
    iter: Iter,
    delimiter: u8,
    init_cb: &InitCb,
    export_cb: &ExportCb,
    dest_cb: &DestCb
)
    -> ResultType<()>
    where Writer: Write,
          Iter: Iterator<Item = &'a Record>,
          Record: 'a + Valid,
          InitCb: Fn(&'b mut Writer, u8) -> ResultType<InnerWriter>,
          ExportCb: Fn(&mut InnerWriter, &'a Record) -> ResultType<()>,
          DestCb: Fn(&mut InnerWriter) -> ResultType<()>
{
    let mut inner = init_cb(writer, delimiter)?;

    // Write all records
    // Error only raised for write error, which should percolate.
    for record in iter {
        export_cb(&mut inner, record)?;
    }

    dest_cb(&mut inner)
}

/// Default exporter from an owning iterator.
pub fn value_iterator_export<
    'a,
    Iter,
    Writer,
    InnerWriter,
    Record,
    InitCb,
    ExportCb,
    DestCb
>
(
    writer: &'a mut Writer,
    iter: Iter,
    delimiter: u8,
    init_cb: &InitCb,
    export_cb: &ExportCb,
    dest_cb: &DestCb
)
    -> ResultType<()>
    where Writer: Write,
          Iter: Iterator<Item = ResultType<Record>>,
          Record: Valid,
          InitCb: Fn(&'a mut Writer, u8) -> ResultType<InnerWriter>,
          ExportCb: Fn(&mut InnerWriter, &Record) -> ResultType<()>,
          DestCb: Fn(&mut InnerWriter) -> ResultType<()>
{
    let mut inner = init_cb(writer, delimiter)?;

    // Write all records
    // Error only raised for read or write errors, which should percolate.
    for record in iter {
        export_cb(&mut inner, &record?)?;
    }

    dest_cb(&mut inner)
}

/// Strict exporter from a non-owning iterator.
pub fn reference_iterator_export_strict<
    'a, 'b,
    Iter,
    Writer,
    InnerWriter,
    Record,
    InitCb,
    ExportCb,
    DestCb
>
(
    writer: &'b mut Writer,
    iter: Iter,
    delimiter: u8,
    init_cb: &InitCb,
    export_cb: &ExportCb,
    dest_cb: &DestCb
)
    -> ResultType<()>
    where Writer: Write,
          Iter: Iterator<Item = &'a Record>,
          Record: 'a + Valid,
          InitCb: Fn(&'b mut Writer, u8) -> ResultType<InnerWriter>,
          ExportCb: Fn(&mut InnerWriter, &'a Record) -> ResultType<()>,
          DestCb: Fn(&mut InnerWriter) -> ResultType<()>
{
    let mut inner = init_cb(writer, delimiter)?;

    for record in iter {
        if record.is_valid() {
            export_cb(&mut inner, record)?;
        } else {
            return Err(From::from(ErrorKind::InvalidRecord));
        }
    }

    dest_cb(&mut inner)
}

/// Strict exporter from an owning iterator.
pub fn value_iterator_export_strict<
    'a,
    Iter,
    Writer,
    InnerWriter,
    Record,
    InitCb,
    ExportCb,
    DestCb
>
(
    writer: &'a mut Writer,
    iter: Iter,
    delimiter: u8,
    init_cb: &InitCb,
    export_cb: &ExportCb,
    dest_cb: &DestCb
)
    -> ResultType<()>
    where Writer: Write,
          Iter: Iterator<Item = ResultType<Record>>,
          Record: Valid,
          InitCb: Fn(&'a mut Writer, u8) -> ResultType<InnerWriter>,
          ExportCb: Fn(&mut InnerWriter, &Record) -> ResultType<()>,
          DestCb: Fn(&mut InnerWriter) -> ResultType<()>
{
    let mut inner = init_cb(writer, delimiter)?;

    for result in iter {
        let record = result?;
        if record.is_valid() {
            export_cb(&mut inner, &record)?;
        } else {
            return Err(From::from(ErrorKind::InvalidRecord));
        }
    }

    dest_cb(&mut inner)
}

/// Lenient exporter from a non-owning iterator.
pub fn reference_iterator_export_lenient<
    'a, 'b,
    Iter,
    Writer,
    InnerWriter,
    Record,
    InitCb,
    ExportCb,
    DestCb
>
(
    writer: &'b mut Writer,
    iter: Iter,
    delimiter: u8,
    init_cb: &InitCb,
    export_cb: &ExportCb,
    dest_cb: &DestCb
)
    -> ResultType<()>
    where Writer: Write,
          Iter: Iterator<Item = &'a Record>,
          Record: 'a + Valid,
          InitCb: Fn(&'b mut Writer, u8) -> ResultType<InnerWriter>,
          ExportCb: Fn(&mut InnerWriter, &'a Record) -> ResultType<()>,
          DestCb: Fn(&mut InnerWriter) -> ResultType<()>
{
    let mut inner = init_cb(writer, delimiter)?;

    // Write all records
    // Error only raised for write error, which should percolate.
    for record in iter {
        if record.is_valid() {
            export_cb(&mut inner, record)?;
        }
    }

    dest_cb(&mut inner)
}

/// Lenient exporter from an owning iterator.
pub fn value_iterator_export_lenient<
    'a,
    Iter,
    Writer,
    InnerWriter,
    Record,
    InitCb,
    ExportCb,
    DestCb
>
(
    writer: &'a mut Writer,
    iter: Iter,
    delimiter: u8,
    init_cb: &InitCb,
    export_cb: &ExportCb,
    dest_cb: &DestCb
)
    -> ResultType<()>
    where Writer: Write,
          Iter: Iterator<Item = ResultType<Record>>,
          Record: Valid,
          InitCb: Fn(&'a mut Writer, u8) -> ResultType<InnerWriter>,
          ExportCb: Fn(&mut InnerWriter, &Record) -> ResultType<()>,
          DestCb: Fn(&mut InnerWriter) -> ResultType<()>
{
    let mut inner = init_cb(writer, delimiter)?;

    // Write all records
    // Error only raised for write error, which should percolate.
    for result in iter {
        let record = result?;
        if record.is_valid() {
            export_cb(&mut inner, &record)?;
        }
    }

    dest_cb(&mut inner)
}
