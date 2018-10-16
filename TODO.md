# TODO List

# Readers and Writers
    - from_x should really be implemented with a BufReader
    - to_x should really be implemented with a BufWriter
        - Use Cursor<T> to wrap in-memory objects
        - https://stackoverflow.com/a/41069910/4131059

# Text Reader
    - Need to_text and from_text (lenient and strict) implementations for collections
        - Can short-circuit the single-element items to allow the same overloads to be called but only retrieve a single item.
