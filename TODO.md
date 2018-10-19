# TODO List

# Cleanup the Uniprot CSV and FASTA APIs.

# Text and CSV Readers
    - Need to ensure to use streams for the API.
    - Need the lenient and strict methods for them
        -Allow both one-shot and lazy saving and loading for everything.

    - Need to export numbers using thousand separators
        - Need to strip that in the import...

# Strict and Lenient
    - Need to rework the strict and lenient serializers...
        - We currently don't check `is_valid()`, we should for strict.

# Formats
    - Add MGF, mzXML, mzML, etc....
    - Going to need XML support.
