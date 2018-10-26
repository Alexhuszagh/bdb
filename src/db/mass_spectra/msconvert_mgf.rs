//! Utilities to load and save MSConvert MGF files.

use std::io::prelude::*;

use util::*;
//use super::mgf::MgfIter;
use super::record::Record;


// SIZE

/// Estimate the size of an MSConvert MGF record.
#[inline]
pub fn estimate_msconvert_mgf_record_size(_: &Record) -> usize {
    // TODO(ahuszagh)   Implement
    0
}

// WRITER

/// Export record to MSConvert MGF.
#[allow(unused_variables)]     // TODO(ahuszagh), Remove
pub fn record_to_msconvert_mgf<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    // TODO(ahuszagh)
    //  Implement
    Ok(())
}

// READER

/// Import record from MGF.
#[allow(unused_variables)]     // TODO(ahuszagh), Remove
pub fn record_from_msconvert_mgf<T: BufRead>(reader: &mut T)
    -> ResultType<Record>
{
    Ok(Record::new())
}

// TODO(ahuszagh)
//  Implement...

//static void parse(Spectrum &spectrum,
//    string::Wrapper &str,
//    PeakProcessor &process)
//{
//    // scan title line, "TITLE=QPvivo_2015_11_10_1targetmethod..."
//    if (mgf::match(title, str)) {
//        spectrum.file = title.captured(1);
//        spectrum.num = lexi::lexi<Num>(title.captured(2));
//    }
//
//    // rt line, "RTINSECONDS=8692.657303"
//    if (mgf::match(rt, str)) {
//        spectrum.rt = lexi::lexi<RetentionTime>(rt.captured(1)) / 60;
//    }
//
//    // pepMass line, "PEPMASS=775.15625 170643.953125"
//    if (mgf::match(mass, str)) {
//        spectrum.parent_mz = lexi::lexi<Mz>(mass.captured(1));
//        if (!mass.piece(2).empty()) {
//            spectrum.parent_intensity = lexi::lexi<Intensity>(mass.captured(2));
//        }
//    }
//
//    // charge line, "CHARGE=4+"
//    if (mgf::match(charge, str)) {
//        spectrum.parent_z = lexi::lexi<Z>(charge.captured(1));
//    }
//
//    if (process.store()) {
//        mgf::parse_spectra(spectrum, str, delimiter);
//        process(spectrum.peaks);
//    }
//}
