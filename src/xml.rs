/**
 *  XML
 *  ---
 *
 *  Trait for XML serializers and deserializers.
 *
 *  :copyright: (c) 2018 Alex Huszagh.
 *  :license: MIT, see LICENSE.md for more details.
 */

// TRAITS
// ------

/**
 *  \brief Trait that defines XML serializers and deserializers.
 *
 *  The `to_xml` method should return a `String` of the following format,
 *  while the `from_xml` method should create a struct instance from a
 *  string of the following format.
 *
 *  \format
 *      // TODO: define
 */
pub trait Xml: Sized {
    // TODO(ahuszagh) Implement
    fn noop(self);
//    /**
//     *  \brief Export struct to XML document.
//     */
//    // TODO(ahuszagh): implement in terms of `to_xml_node`.
//    fn to_xml(&self) -> Result<String, &str>;
//
//    /**
//     *  \brief Export struct to XML node.
//     */
//    fn to_xml_node(&self) -> Result<String, &str>;
//
//
//    /**
//     *  \brief Import struct from XML document.
//     */
//    // TODO(ahuszagh): implement in terms of `from_xml_node`.
//    fn from_xml(fasta: &str) -> Result<Self, &str>;
//
//    /**
//     *  \brief Import struct from XML node.
//     */
//    fn from_xml_node(fasta: &str) -> Result<Self, &str>;
}
