//! Client to request resources from the SRA.

// TODO(ahuszagh)
//      Implement...

// Look at the above
// https://www.ncbi.nlm.nih.gov/books/NBK158899/
// Example:
//  ERR1953444

// This is the file format reference:
//      https://www.ncbi.nlm.nih.gov/sra/docs/submitformats/#bam-files
// Need to support .sra binary records.... Le fuck.
//      ERR1953444.sra


// Hey!
//  We need to support these file formats...
//    fastq-dump: Converts data to fastq and fasta format.
//    sam-dump: Converts data to sam (human-readable bam). Data submitted as aligned bam are output as aligned sam, while other formats are output as unaligned sam.
//    sff-dump: Converts data to sff format. Note that only data submitted as sff can be converted back to this format.
//    abi-dump: Converts data to csfasta/csqual format. Note that data submitted in base-space can be represented in color-space, but please be aware of the advantages / disadvantages of converting between different encodings.
//    illumina-dump: Converts data to Illumina native and qseq formats.
//    vdb-dump: Exports the vdb-formatted data of the .sra file.

