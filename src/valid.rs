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
 *  \brief Trait requiring `is_valid` method.
 */
pub trait Valid {
    /**
     *  \brief Check if record contains valid information.
     */
    fn is_valid(&self) -> bool;
}

