//use util::ResultType;

/// Serialize to and from XML.
#[doc(hidden)]
pub trait Xml: Sized {
    // TODO(ahuszagh) Implement
    fn noop(self);
}

/// Specialization of the `Xml` trait for collections.
#[doc(hidden)]
pub trait XmlCollection: Xml {
    // TODO(ahuszagh) Implement
    fn noop(self);
}
