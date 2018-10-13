/**
 *  Text
 *  ----
 *
 *  Trait for serializers and deserializers for custom `.txt` formats.
 *
 *  :copyright: (c) 2018 Alex Huszagh.
 *  :license: MIT, see LICENSE.md for more details.
 */

// TRAITS
// ------

// TODO(ahuszagh)
//      Restore this module off the following format:
//          https://www.uniprot.org/uniprot/A6VR00.txt

/**
 *  \brief Trait that defines custom text serializers and deserializers.
 *
 *  The `to_text` method should return a `String` in the custom text format,
 *  while the `from_text` method should create a struct instance from that
 *  custom text format.
 */
pub trait Text: Sized {
    /**
     *  \brief Export record to custom text format.
     */
    fn to_text(&self) -> Result<String, &str>;

    /**
     *  \brief Import record from custom text format
     */
    fn from_text(text: &str) -> Result<Self, &str>;
}


/**
 *  \brief Specialized version of the Text trait for collections.
 *
 *  \warning This trait may not be relevant for all collections, since
 *  some `.txt` formats do not support multiple records in a single document.
 */
pub trait TextCollection: Sized {
    /**
     *  \brief Export collection of UniProt records to text.
     *
     *  `to_text_strict` requires all records inside the collection
     *  to be valid, or returns an `Err`, while `to_text_lenient` will
     *  return as many formatted records as possible, returning an error
     *  only if no records are valid.
     */
     fn to_text_strict(&self) -> Result<String, &str>;
     fn to_text_lenient(&self) -> Result<String, &str>;

     /**
     *  \brief Import record collection from text.
     *
     *  `from_text_strict` requires all records inside the text
     *  to be valid, or returns an `Err`, while `to_text_lenient` will
     *  return as many record structs as possible, returning an error
     *  only if no records are valid.
     */
    fn from_text_strict(text: &str) -> Result<Self, &str>;
    fn from_text_lenient(text: &str) -> Result<Self, &str>;
}
