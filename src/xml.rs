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
    /**
     *  \brief Export struct to XML document.
     */
    // TODO(ahuszagh): implement in terms of `to_xml_node`.
    fn to_xml(&self) -> Option<String>;

    /**
     *  \brief Export struct to XML node.
     */
    fn to_xml_node(&self) -> Option<String>;


    /**
     *  \brief Import struct from XML document.
     */
    // TODO(ahuszagh): implement in terms of `from_xml_node`.
    fn from_xml(fasta: &str) -> Option<Self>;

    /**
     *  \brief Import struct from XML node.
     */
    fn from_xml_node(fasta: &str) -> Option<Self>;
}
