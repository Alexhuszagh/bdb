use ref_slice::ref_slice;
use std::io::Write;

use super::alias::{ResultType};

// WRITER STATE

/// Stores the current text writer state.
pub struct TextWriterState<'r, T: 'r + Write> {
    writer: &'r mut T,
    /// Whether the previous record exported successfully.
    previous: bool,
    /// Delimiter between records.
    delimiter: u8,
}

impl<'r, T: 'r + Write> TextWriterState<'r, T> {
    /// Construct new state from writer.
    #[inline]
    pub fn new(writer: &'r mut T, delimiter: u8) -> TextWriterState<'r, T> {
        TextWriterState {
            writer: writer,
            previous: false,
            delimiter: delimiter,
        }
    }

    /// Export record to FASTA.
    pub fn export<Value, Callback>(&mut self, value: &Value, callback: Callback)
        -> ResultType<()>
        where Callback: Fn(&mut T, &Value) -> ResultType<()>
    {
        if self.previous {
            self.writer.write_all(ref_slice(&self.delimiter))?;
        }
        match callback(self.writer, value) {
            Err(e)  => {
                self.previous = false;
                Err(e)
            },
            Ok(()) => {
                self.previous = true;
                Ok(())
            }
        }
    }
}
