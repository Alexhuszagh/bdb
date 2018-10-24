/// Determine if model is valid.
pub trait Valid: Sized {
    /// Determine if model contains only valid information.
    fn is_valid(&self) -> bool;
}

