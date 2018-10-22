//! XML reader and writer utilities.

// RE-EXPORTS

pub use self::reader::{XmlReader};
pub use self::writer::{XmlWriter};

// READER

mod reader {

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;
use super::super::alias::{BufferType, ResultType};
use super::super::error::ErrorKind;

/// Macro to seek another element within the tree.
///
/// For internal implementation only. Do not use.
macro_rules! xml_seek {
    ($event:ident, $self:ident, $buffer:ident, $name:ident, $depth:ident)
    =>
    ({
        loop {
            match $self.read_event($buffer) {
                Err(e) => return Some(Err(e)),
                Ok(v)  => match v {
                    Event::$event(e) => {
                        if $self.found_depth($depth) && $self.found_name($name, e.name()) {
                            return Some(Ok(()));
                        }
                    },
                    Event::Eof => return None,
                    _ => (),
                }
            }
            $buffer.clear();
        }
    })
}

/// Internal struct to store the current XML reader state.
struct XmlState<T: BufRead> {
    /// Internal XML reader.
    reader: Reader<T>,
    /// Raw depth of the XML tree (uncorrected!).
    raw_depth: usize,
    /// Node is a start element.
    is_start: bool,
}

impl<T: BufRead> XmlState<T> {
    /// Construct new state from reader.
    #[inline]
    pub fn new(reader: T) -> Self {
        let mut reader = Reader::from_reader(reader);
        reader.expand_empty_elements(true);
        XmlState {
            reader: reader,
            raw_depth: 0,
            is_start: false,
        }
    }

    /// Get the current depth (0-indexed) of the reader.
    ///
    /// Use a property to ensure the depths are actually symmetrical.
    #[inline(always)]
    pub fn depth(&self) -> usize {
        // Optimization to avoid conditional logic, since `false` is `0`,
        // and `true` is `1`.
        self.raw_depth - self.is_start as usize
    }

    /// Read an XML event.
    ///
    /// Always track XML depth to wrap the calls.
    /// Unfortunately, due to how the depth is tracked,
    /// the depth will always be asymmetric for start and end nodes.
    /// Start nodes will always be the same as the end node + 1.
    #[inline]
    pub fn read_event<'a>(&mut self, buffer: &'a mut BufferType)
        -> ResultType<Event<'a>>
    {
        match self.reader.read_event(buffer) {
            Ok(Event::Start(e)) => {
                self.raw_depth += 1;
                self.is_start = true;
                Ok(Event::Start(e))
            },
            Ok(Event::End(e)) => {
                self.raw_depth -= 1;
                self.is_start = false;
                Ok(Event::End(e))
            }
            Ok(event) => {
                self.is_start = false;
                Ok(event)
            },
            Err(e) => {
                self.is_start = false;
                Err(From::from(ErrorKind::Xml(e)))
            },
        }
    }

    /// Read until the corresponding end element.
    #[inline]
    pub fn read_to_end(&mut self, buffer: &mut BufferType, name: &[u8])
        -> ResultType<BufferType>
    {
        match self.reader.read_to_end(name, buffer) {
            Err(e) => return Err(From::from(ErrorKind::Xml(e))),
            Ok(_)  => self.is_start = false,
        }
        let result = buffer.clone();
        self.raw_depth -= 1;
        buffer.clear();
        Ok(result)
    }

    /// Read text between the start and end element.
    #[inline]
    pub fn read_text(&mut self, buffer: &mut BufferType, name: &[u8])
        -> ResultType<String>
    {
        let result = match self.reader.read_text(name, buffer) {
            Err(e) => Err(From::from(ErrorKind::Xml(e))),
            Ok(v)  => {
                self.is_start = false;
                Ok(v)
            },
        };
        self.raw_depth -= 1;
        buffer.clear();
        result
    }

    /// Check if we found the correct depth.
    #[inline(always)]
    fn found_depth(&self, depth: usize) -> bool {
        return depth == usize::max_value() || self.depth() == depth;
    }

    /// Check if we found the correct name.
    #[inline(always)]
    fn found_name(&self, expected: &[u8], actual: &[u8]) -> bool {
        return expected == b"" || actual == expected;
    }

    /// Implied function to process a callback on a start element.
    fn seek_start_callback_impl<State, Callback>(
        &mut self,
        buffer: &mut BufferType,
        name: &[u8],
        depth: usize,
        state: &mut State,
        callback: Callback
    )
        -> Option<ResultType<bool>>
        where Callback: Fn(BytesStart, &mut State) -> Option<ResultType<bool>>
    {
        loop {
            match self.read_event(buffer) {
                Err(e) => return Some(Err(e)),
                Ok(v)  => match v {
                    Event::Start(e) => {
                        if self.found_depth(depth) && self.found_name(name, e.name()) {
                            return callback(e, state);
                        }
                    },
                    Event::Eof => return None,
                    _ => (),
                }
            }
            buffer.clear();
        }
    }

    /// Seek start element event and process event with callback.
    pub fn seek_start_callback<State, Callback>(
        &mut self,
        buffer: &mut BufferType,
        name: &[u8],
        depth: usize,
        state: &mut State,
        callback: Callback
    )
        -> Option<ResultType<bool>>
        where Callback: Fn(BytesStart, &mut State) -> Option<ResultType<bool>>
    {
        let result = self.seek_start_callback_impl(buffer,name, depth,state, callback);
        buffer.clear();
        result
    }

    /// Seek start element based off name and depth.
    ///
    /// Does not sufficiently clear necessary buffers, and therefore
    /// must be wrapped in another caller.
    #[inline]
    fn seek_start_impl(&mut self, buffer: &mut BufferType, name: &[u8], depth: usize)
        -> Option<ResultType<()>>
    {
        xml_seek!(Start, self, buffer, name, depth)
    }

    /// Seek start element based off name and depth.
    #[inline]
    pub fn seek_start(&mut self, buffer: &mut BufferType, name: &[u8], depth: usize)
        -> Option<ResultType<()>>
    {
        let result = self.seek_start_impl(buffer,name, depth);
        buffer.clear();
        result
    }

    /// Private implied method to seek end.
    ///
    /// Does not sufficiently clear necessary buffers, and therefore
    /// must be wrapped in another caller.
    #[inline]
    fn seek_end_impl(&mut self, buffer: &mut BufferType, name: &[u8], depth: usize)
        -> Option<ResultType<()>>
    {
        xml_seek!(End, self, buffer, name, depth)
    }

    /// Seek end element based off name and depth.
    #[inline]
    pub fn seek_end(&mut self, buffer: &mut BufferType, name: &[u8], depth: usize)
        -> Option<ResultType<()>>
    {
        let result = self.seek_end_impl(buffer,name, depth);
        buffer.clear();
        result
    }
}

/// Public API for the XML reader.
pub struct XmlReader<T: BufRead> {
    /// Stored state for the reader.
    state: XmlState<T>,
    /// Buffer tied to XML events.
    buffer: BufferType,
}

impl<T: BufRead> XmlReader<T> {
    /// Create new XmlReader.
    #[inline]
    pub fn new(reader: T) -> Self {
        XmlReader {
            state: XmlState::new(reader),
            buffer: BufferType::with_capacity(8000),
        }
    }

    /// Read an XML event.
    #[inline(always)]
    pub fn read_event(&mut self) -> ResultType<Event> {
        self.state.read_event(&mut self.buffer)
    }

    /// Read until the matching XML end element.
    #[inline(always)]
    pub fn read_to_end(&mut self, name: &[u8]) -> ResultType<BufferType> {
        self.state.read_to_end(&mut self.buffer, name)
    }

    /// Read text between the start and end element.
    #[inline(always)]
    pub fn read_text(&mut self, name: &[u8]) -> ResultType<String> {
        self.state.read_text(&mut self.buffer, name)
    }

    /// Get the current depth (0-indexed) of the reader.
    #[inline(always)]
    pub fn depth(&self) -> usize {
        self.state.depth()
    }

    /// Get the current reader position in the buffer.
    #[inline(always)]
    pub fn buffer_position(&self) -> usize {
        self.state.reader.buffer_position()
    }

    /// Seek start element event by name and depth and process event with callback.
    #[inline(always)]
    pub fn seek_start_callback<State, Callback>(
        &mut self,
        name: &[u8],
        depth: usize,
        state: &mut State,
        callback: Callback
    )
        -> Option<ResultType<bool>>
        where Callback: Fn(BytesStart, &mut State) -> Option<ResultType<bool>>
    {
        self.state.seek_start_callback(&mut self.buffer, name, depth, state, callback)
    }

    /// Seek start element event by name and process event with callback.
    #[inline(always)]
    pub fn seek_start_name_callback<State, Callback>(
        &mut self,
        name: &[u8],
        state: &mut State,
        callback: Callback
    )
        -> Option<ResultType<bool>>
        where Callback: Fn(BytesStart, &mut State) -> Option<ResultType<bool>>
    {
        self.seek_start_callback(name, usize::max_value(), state, callback)
    }

    /// Seek start element event by name and process event with callback.
    #[inline(always)]
    pub fn seek_start_depth_callback<State, Callback>(
        &mut self,
        depth: usize,
        state: &mut State,
        callback: Callback
    )
        -> Option<ResultType<bool>>
        where Callback: Fn(BytesStart, &mut State) -> Option<ResultType<bool>>
    {
        self.seek_start_callback(b"", depth, state, callback)
    }

    /// Seek start element based off name and depth.
    #[inline(always)]
    pub fn seek_start(&mut self, name: &[u8], depth: usize) -> Option<ResultType<()>> {
        self.state.seek_start(&mut self.buffer, name, depth)
    }

    /// Seek start element based off name.
    #[inline(always)]
    pub fn seek_start_name(&mut self, name: &[u8]) -> Option<ResultType<()>> {
        self.seek_start(name, usize::max_value())
    }

    /// Seek start element based off depth.
    #[inline(always)]
    pub fn seek_start_depth(&mut self, depth: usize) -> Option<ResultType<()>> {
        self.seek_start(b"", depth)
    }

    /// Seek end element based off name and depth.
    #[inline(always)]
    pub fn seek_end(&mut self, name: &[u8], depth: usize) -> Option<ResultType<()>> {
        self.state.seek_end(&mut self.buffer, name, depth)
    }

    /// Seek end element based off name.
    #[inline]
    pub fn seek_end_name(&mut self, name: &[u8]) -> Option<ResultType<()>> {
        self.seek_end(name, usize::max_value())
    }

    /// Seek end element based off depth.
    #[inline]
    pub fn seek_end_depth(&mut self, depth: usize) -> Option<ResultType<()>> {
        self.seek_end(b"", depth)
    }
}

}   // reader

// WRITER

mod writer {

use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use std::io::Write;

use super::super::alias::ResultType;
use super::super::error::ErrorKind;

/// Public API for the XML writer.
pub struct XmlWriter<T: Write> {
    /// Internal XML writer.
    writer: Writer<T>,
}

impl<T: Write> XmlWriter<T> {
    /// Create new XmlWriter.
    #[inline]
    pub fn new(writer: T) -> Self {
        XmlWriter {
            writer: Writer::new(writer)
        }
    }

    /// Create start element
    #[inline(always)]
    fn new_start_element(bytes: &[u8]) -> BytesStart {
        BytesStart::borrowed(bytes, bytes.len())
    }

    /// Create text element.
    #[inline(always)]
    fn new_text_element<'a>(text: &[u8]) -> BytesText {
        BytesText::from_plain(text)
    }

    /// Create end element.
    #[inline(always)]
    fn new_end_element(bytes: &[u8]) -> BytesEnd {
        BytesEnd::borrowed(bytes)
    }

    /// Process a write event.
    #[inline(always)]
    fn write_event(&mut self, event: Event) -> ResultType<()> {
        match self.writer.write_event(event) {
            Err(e)  => Err(From::from(ErrorKind::Xml(e))),
            _       => Ok(()),
        }
    }

    /// Write the XML declaration.
    #[inline(always)]
    pub fn write_declaration(&mut self) -> ResultType<()> {
        let decl = BytesDecl::new(b"1.0", Some(b"UTF-8"), None);
        self.write_event(Event::Decl(decl))
    }

    /// Write start element.
    #[inline(always)]
    pub fn write_start_element(&mut self, name: &[u8], attributes: &[(&[u8], &[u8])])
        -> ResultType<()>
    {
        let mut elem = Self::new_start_element(name);
        for attribute in attributes {
            elem.push_attribute(*attribute);
        }
        self.write_event(Event::Start(elem))
    }

    /// Write text element (with start and end elements).
    pub fn write_text_element(&mut self, name: &[u8], text: &[u8], attributes: &[(&[u8], &[u8])])
        -> ResultType<()>
    {
        self.write_start_element(name, attributes)?;
        self.write_event(Event::Text(Self::new_text_element(text)))?;
        self.write_end_element(name)
    }

    /// Write start element.
    #[inline(always)]
    pub fn write_empty_element(&mut self, name: &[u8], attributes: &[(&[u8], &[u8])])
        -> ResultType<()>
    {
        self.write_start_element(name, attributes)?;
        self.write_end_element(name)
    }

    /// Write start element.
    #[inline(always)]
    pub fn write_end_element(&mut self, name: &[u8])
        -> ResultType<()>
    {
        self.write_event(Event::End(Self::new_end_element(name)))
    }
}

}   // writer

#[cfg(test)]
mod tests {
    // TODO(ahuszagh)
    // Implement...
}
