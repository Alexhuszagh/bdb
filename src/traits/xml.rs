use std::io::{Write};

use util::ResultType;

/// Serialize to and from XML.
/// TODO(ahuszagh)
///     Expand on this and implement...
#[doc(hidden)]
pub trait Xml: Sized {
    /// Estimate the size of the resulting XML output to avoid reallocations.
    #[inline(always)]
    fn estimate_xml_size(&self) -> usize {
        0
    }

    /// Export model to XML.
    fn to_xml<T: Write>(&self, writer: &mut T) -> ResultType<()>;
}

/// Specialization of the `Xml` trait for collections.
#[doc(hidden)]
pub trait XmlCollection: Xml {
    // TODO(ahuszagh) Implement
    fn noop(self);
}
