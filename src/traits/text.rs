use util::ResultType;

// TODO(ahuszagh)
//      Restore this module off the following format:
//          https://www.uniprot.org/uniprot/A6VR00.txt

/// Serialize to and from custom text formats.
pub trait Text: Sized {
    /// Export model to text.
    fn to_text(&self) -> ResultType<String>;

    /// Import model from text.
    fn from_text(text: &str) -> ResultType<Self>;
}

/// Specialization of the `Text` trait for collections.
///
/// This specialization may not be applicable for all text formats,
/// since certain proprietary text formats allow for a maximum of
/// 1 item per document.
pub trait TextCollection: Text{
    /// Export collection to text.
    ///
    /// Returns an error if any of the items within the collection
    /// are invalid.
    fn to_text_strict(&self) -> ResultType<String>;

    /// Export collection to text.
    ///
    /// Returns an error if none of the items are valid, otherwise,
    /// exports as many items as possible.
    fn to_text_lenient(&self) -> ResultType<String>;

    /// Import collection from text.
    ///
    /// Returns an error if any of the items within the document
    /// are invalid.
    fn from_text_strict(text: &str) -> ResultType<Self>;

    /// Import collection from text.
    ///
    /// Returns an error if none of the items within the document
    /// are valid, otherwise, imports as many items as possible.
    fn from_text_lenient(text: &str) -> ResultType<Self>;
}
