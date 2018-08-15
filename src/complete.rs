/**
 *  Valid
 *  -----
 *
 *  Trait for a type to implement an `is_valid` function.
 *
 *  :copyright: (c) 2018 Alex Huszagh.
 *  :license: MIT, see LICENSE.md for more details.
 */

// TRAITS
// ------

/**
 *  \brief Trait requiring `is_complete` method.
 */
pub trait Complete {
    /**
     *  \brief Check if record contains all identifiers.
     */
    fn is_complete(&self) -> bool;
}
