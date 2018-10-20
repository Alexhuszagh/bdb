//! Valid trait implementation for UniProt models.

use traits::Valid;
use super::re::*;
use super::evidence::ProteinEvidence;
use super::record::Record;
use super::record_list::RecordList;

impl Valid for Record {
    fn is_valid(&self) -> bool {
        (
            // Do not try to validate the Organism
            // With virus names being non-standard, it is impossible
            // with an NFA, and extremely time complex otherwise.
            self.sequence_version > 0 &&
            self.protein_evidence < ProteinEvidence::Unknown &&
            self.mass > 0 &&
            self.length as usize == self.sequence.len() &&
            !self.sequence.is_empty() &&
            !self.name.is_empty() &&
            !self.organism.is_empty() &&
            GeneRegex::validate().is_match(&self.gene) &&
            AccessionRegex::validate().is_match(&self.id) &&
            MnemonicRegex::validate().is_match(&self.mnemonic) &&
            AminoacidRegex::validate().is_match(&self.sequence) &&
            (
                self.proteome.is_empty() ||
                ProteomeRegex::validate().is_match(&self.proteome)
            ) &&
            (
                self.taxonomy.is_empty() ||
                TaxonomyRegex::validate().is_match(&self.taxonomy)
            )
        )
    }
}

impl Valid for RecordList {
    #[inline]
    fn is_valid(&self) -> bool {
        self.iter().all(|ref x| x.is_valid())
    }
}
