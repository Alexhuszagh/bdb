//! Shared iterator templates and utilities.

use std::io::prelude::*;

use traits::Valid;
use super::alias::{Bytes, Result};
use super::error::ErrorKind;

// READER

/// Iterator which raises an error for invalid items.
pub struct StrictIter<T: Valid, U: Iterator<Item = Result<T>>> {
    /// Wrapped internal iterator.
    iter: U,
}

impl<T: Valid, U: Iterator<Item = Result<T>>> StrictIter<T, U> {
    /// Create new StrictIter from a buffered reader.
    #[inline]
    pub fn new(iter: U) -> Self {
        StrictIter {
            iter: iter
        }
    }
}

impl<T: Valid, U: Iterator<Item = Result<T>>> Iterator for StrictIter<T, U> {
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
pub struct LenientIter<T: Valid, U: Iterator<Item = Result<T>>> {
    /// Wrapped internal iterator.
    iter: U,
}

impl<T: Valid, U: Iterator<Item = Result<T>>> LenientIter<T, U> {
    /// Create new LenientIter from a buffered reader.
    #[inline]
    pub fn new(iter: U) -> Self {
        LenientIter {
            iter: iter
        }
    }
}

impl<T: Valid, U: Iterator<Item = Result<T>>> Iterator for LenientIter<T, U> {
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
    -> Result<()>
    where Writer: Write,
          Iter: Iterator<Item = &'a Record>,
          Record: 'a + Valid,
          InitCb: Fn(&'b mut Writer, u8) -> Result<InnerWriter>,
          ExportCb: Fn(&mut InnerWriter, &'a Record) -> Result<()>,
          DestCb: Fn(&mut InnerWriter) -> Result<()>
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
    -> Result<()>
    where Writer: Write,
          Iter: Iterator<Item = Result<Record>>,
          Record: Valid,
          InitCb: Fn(&'a mut Writer, u8) -> Result<InnerWriter>,
          ExportCb: Fn(&mut InnerWriter, &Record) -> Result<()>,
          DestCb: Fn(&mut InnerWriter) -> Result<()>
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
    -> Result<()>
    where Writer: Write,
          Iter: Iterator<Item = &'a Record>,
          Record: 'a + Valid,
          InitCb: Fn(&'b mut Writer, u8) -> Result<InnerWriter>,
          ExportCb: Fn(&mut InnerWriter, &'a Record) -> Result<()>,
          DestCb: Fn(&mut InnerWriter) -> Result<()>
{
    let mut inner = init_cb(writer, delimiter)?;

    for record in iter {
        bool_to_error!(record.is_valid(), InvalidRecord);
        export_cb(&mut inner, record)?;
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
    -> Result<()>
    where Writer: Write,
          Iter: Iterator<Item = Result<Record>>,
          Record: Valid,
          InitCb: Fn(&'a mut Writer, u8) -> Result<InnerWriter>,
          ExportCb: Fn(&mut InnerWriter, &Record) -> Result<()>,
          DestCb: Fn(&mut InnerWriter) -> Result<()>
{
    let mut inner = init_cb(writer, delimiter)?;

    for result in iter {
        let record = result?;
        bool_to_error!(record.is_valid(), InvalidRecord);
        export_cb(&mut inner, &record)?;
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
    -> Result<()>
    where Writer: Write,
          Iter: Iterator<Item = &'a Record>,
          Record: 'a + Valid,
          InitCb: Fn(&'b mut Writer, u8) -> Result<InnerWriter>,
          ExportCb: Fn(&mut InnerWriter, &'a Record) -> Result<()>,
          DestCb: Fn(&mut InnerWriter) -> Result<()>
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
    -> Result<()>
    where Writer: Write,
          Iter: Iterator<Item = Result<Record>>,
          Record: Valid,
          InitCb: Fn(&'a mut Writer, u8) -> Result<InnerWriter>,
          ExportCb: Fn(&mut InnerWriter, &Record) -> Result<()>,
          DestCb: Fn(&mut InnerWriter) -> Result<()>
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

// NEXT

/// Clone the resulting buffer (or none if the buffer is empty.)
/// Must be called inside an `unsafe` block.
#[doc(hidden)]
#[macro_export]
macro_rules! clone_bytes {
    ($buf:expr) => ({
        let result = match $buf.len() {
            0   => None,
            _   => Some(Ok($buf.clone())),
        };
        $buf.set_len(0);
        result
    })
}

/// Macro to fetch the next item from a reader.
#[doc(hidden)]
#[macro_export]
macro_rules! bytes_next {
    ($reader:expr, $buf:expr, $line:expr, $block:expr) => ({
        loop {
            match $reader.read_until(b'\n', $line) {
                Err(e)      => return Some(Err(From::from(e))),
                Ok(size)    => match size {
                    // Reached EOF
                    0   => return unsafe { clone_bytes!($buf) },
                    // Read bytes, process them.
                    _   => $block,
                }
            }
        }
    })
}

/// Produce the next element from a bytes-based iterator (skipping whitespace).
pub fn bytes_next_skip_whitespace<T: BufRead>(
    start: &[u8],
    reader: &mut T,
    buf: &mut Bytes,
    line: &mut Bytes
)
    -> Option<Result<Bytes>>
{
    bytes_next!(reader, buf, line, unsafe {
        if line == b"\n" || line == b"\r\n" {
            // Ignore whitespace.
            line.set_len(0);
            continue;
        } else if buf.len() > 0 && line.starts_with(start) {
            // Create result from existing buffer,
            // clear the existing buffer, and add
            // the current line to a new buffer.
            let result = clone_bytes!(buf);
            buf.append(line);
            return result;
        } else {
            // Move the line to the buffer.
            buf.append(line);
        }
    })
}
