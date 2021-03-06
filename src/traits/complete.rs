/// Determine if model is complete.
pub trait Complete: Sized {
    /// Determine if model contains all possible information.
    fn is_complete(&self) -> bool;
}
