/**
 *  Text
 *  ----
 *
 *  Trait for CSV and tab-delimited text (TBT) serializers and deserializers.
 *
 *  :copyright: (c) 2018 Alex Huszagh.
 *  :license: MIT, see LICENSE.md for more details.
 */

// TRAITS
// ------

/**
 *  \brief Trait that defines CSV serializers and deserializers.
 *
 *  The `to_tbt` method should return a `String` in CSV or TBT, with
 *  headers, while the `from_tbt` method should create a struct
 *  instance from a CSV or TBT string with headers.
 */
pub trait Tbt: Sized {
    /**
     *  \brief Export record to CSV or TBT.
     */
    fn to_tbt(&self) -> Result<String, &str>;

    /**
     *  \brief Import record from CSV or TBT text.
     *
     *  Works identically to a collection importer, only fetches at max
     *  1 record, since the headers are shared over all records.
     */
    fn from_tbt<'a>(text: &str) -> Result<Self, &'a str>;
}


/**
 *  \brief Specialized version of the Tbt trait for collections.
 */
pub trait TextCollection: Sized {
    /**
     *  \brief Export collection of UniProt records to CSV or TBT.
     *
     *  `to_tbt_strict` requires all records inside the collection
     *  to be valid, or returns an `Err`, while `to_tbt_lenient` will
     *  return as many formatted records as possible, returning an error
     *  only if no records are valid.
     */
     fn to_tbt_strict(&self) -> Result<String, &str>;
     fn to_tbt_lenient(&self) -> Result<String, &str>;

     /**
     *  \brief Import record collection from CSV or TBT.
     *
     *  `from_tbt_strict` requires all records inside the text
     *  to be valid, or returns an `Err`, while `to_tbt_lenient` will
     *  return as many record structs as possible, returning an error
     *  only if no records are valid.
     */
    fn from_tbt_strict<'a>(text: &str) -> Result<Self, &'a str>;
    fn from_tbt_lenient<'a>(text: &str) -> Result<Self, &'a str>;
}
