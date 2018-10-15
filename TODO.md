# TODO List

# Errors
    - Convert all errors to use Box<Error> as the error type.
        - Results should be Result<T, Box<Error>>
        - Easy to percolate errors down, can convert from stack values.
            - I think can convert from stack values...
            - Need to derive from std::Error, in any case....

# Text Reader
    - Need to_text and from_text (lenient and strict) implementations for collections
        - Can short-circuit the single-element items to allow the same overloads to be called but only retrieve a single item.

# Traits
    - Need to rename the Text trait to Csv
    - Need the Text trait to correspond to the following:
        https://www.uniprot.org/uniprot/A6VR00.txt
