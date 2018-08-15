/**
 *  Text
 *  ----
 *
 *  Trait for tab-delimited text serializers and deserializers.
 *
 *  :copyright: (c) 2018 Alex Huszagh.
 *  :license: MIT, see LICENSE.md for more details.
 */

// ALIAS
// -----

// TODO(ahuszagh)
//  Define the `Column` and `ColumnList` aliases.
//pub type Column = &str;
//pub type Vec<&str>

// TRAITS
// ------

/**
 *  \brief Trait that defines text serializers and deserializers.
 *
 *  The `to_text` method should return a `String` of the following format,
 *  while the `from_text` method should create a struct instance from a
 *  string of the following format.
 *
 *  \format
 *      // TODO: define
 */
pub trait Text: Sized {
// TODO: I need columns to be passed....
// It really shouldn't support a single order....

//    /**
//     *  \brief Export struct to tab-delimited text document.
//     */
//    fn to_text(&self) -> Option<String>;
//
//    /**
//     *  \brief Export struct to row(s) in document.
//     */
//    fn to_text_row(&self) -> Option<String>;

// TODO(ahuszagh)
//    /**
//     *  \brief Import record from XML document.
//     */
//    // TODO(ahuszagh): implement in terms of `from_xml_node`.
//    fn from_xml(fasta: &str) -> Option<Self>;
//
//    /**
//     *  \brief Import record from XML node.
//     */
//    fn from_xml_node(fasta: &str) -> Option<Self>;
}
