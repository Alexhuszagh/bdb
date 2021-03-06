//! Utilities to load and save Pava MGF files.

use std::io::prelude::*;
use std::io::Lines;

use traits::*;
use util::*;
use super::mgf::MgfRecordIter;
use super::peak::Peak;
use super::re::*;
use super::record::Record;

// SIZE

/// Estimate the size of a Pava MGF record.
#[inline]
pub(crate) fn estimate_pava_mgf_record_size(record: &Record) -> usize {
    // Actual size is ~50 with a lot of extra size for the scan,
    // and the peptide RT, m/z, and intensity.
    const MGF_VOCABULARY_SIZE: usize = 100;
    // Estimated average is ~20 characters per line, assume slightly above.
    const MGF_PEAK_SIZE: usize = 25;
    MGF_VOCABULARY_SIZE +
        record.file.len() +
        MGF_PEAK_SIZE * record.peaks.len()
}

// WRITER

#[inline(always)]
fn to_mgf<'a, T: Write>(writer: &mut T, record: &'a Record)
    -> Result<()>
{
    record_to_pava_mgf(writer, record)
}

#[inline(always)]
fn export_title<T: Write>(writer: &mut T, record: &Record)
    -> Result<()>
{
    let num = to_bytes(&record.num)?;
    let rt = to_bytes(&record.rt)?;
    write_alls!(
        writer,
        b"TITLE=Scan ", num.as_slice(), b" (rt=", rt.as_slice(),
        b") [", record.file.as_bytes(), b"]\n"
    )?;

    Ok(())
}

#[inline(always)]
fn export_pepmass<T: Write>(writer: &mut T, record: &Record)
    -> Result<()>
{
    let parent_mz = to_bytes(&record.parent_mz)?;
    write_alls!(writer, b"PEPMASS=", parent_mz.as_slice())?;
    if record.parent_intensity != 0.0 {
        let parent_intensity = to_bytes(&record.parent_intensity)?;
        write_alls!(writer, b"\t", parent_intensity.as_slice())?;
    }
    writer.write_all(b"\n")?;

    Ok(())
}

#[inline(always)]
fn export_charge<T: Write>(writer: &mut T, record: &Record)
    -> Result<()>
{
    writer.write_all(b"CHARGE=")?;
    if record.parent_z > 0 {
        let parent_z = to_bytes(&record.parent_z)?;
        write_alls!(writer, parent_z.as_slice(), b"+")?;
    } else {
        let z = -record.parent_z;
        let parent_z = to_bytes(&z)?;
        write_alls!(writer, parent_z.as_slice(), b"-")?;
    }
    writer.write_all(b"\n")?;

    Ok(())
}

#[inline(always)]
fn export_spectra<T: Write>(writer: &mut T, record: &Record)
    -> Result<()>
{
    for peak in record.peaks.iter() {
        let mz = to_bytes(&peak.mz)?;
        let intensity = to_bytes(&peak.intensity)?;
        write_alls!(writer, mz.as_slice(), b"\t", intensity.as_slice(), b"\n")?;
    }

    Ok(())
}

/// Export record to PAVA MGF.
pub(crate) fn record_to_pava_mgf<T: Write>(writer: &mut T, record: &Record)
    -> Result<()>
{
    writer.write_all(b"BEGIN IONS\n")?;
    export_title(writer, record)?;
    export_pepmass(writer, record)?;
    export_charge(writer, record)?;
    export_spectra(writer, record)?;
    writer.write_all(b"END IONS\n\n")?;

    Ok(())
}

// WRITER -- DEFAULT

#[inline(always)]
fn init_cb<T: Write>(writer: &mut T, delimiter: u8)
    -> Result<TextWriterState<T>>
{
    Ok(TextWriterState::new(writer, delimiter))
}

#[inline(always)]
fn export_cb<'a, T: Write>(writer: &mut TextWriterState<T>, record: &'a Record)
    -> Result<()>
{
    writer.export(record, &to_mgf)
}

#[inline(always)]
fn dest_cb<T: Write>(_: &mut TextWriterState<T>)
    -> Result<()>
{
    Ok(())
}

/// Default exporter from a non-owning iterator to Pava MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_pava_mgf<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Default exporter from an owning iterator to Pava MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_pava_mgf<Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    value_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to Pava MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_pava_mgf_strict<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Strict exporter from an owning iterator to Pava MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_pava_mgf_strict<Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    value_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to Pava MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_pava_mgf_lenient<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Lenient exporter from an owning iterator to Pava MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_pava_mgf_lenient<Iter, T>(writer: &mut T, iter: Iter)
    -> Result<()>
    where T: Write,
          Iter: Iterator<Item = Result<Record>>
{
    value_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// READER

/// Parse the start header line.
#[inline(always)]
fn parse_start_line<T: BufRead>(lines: &mut Lines<T>, _: &mut Record)
    -> Result<()>
{
    // Verify the start header line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    bool_to_error!(line == "BEGIN IONS", InvalidInput);

    Ok(())
}

/// Parse the title header line.
#[inline(always)]
fn parse_title_line<T: BufRead>(lines: &mut Lines<T>, record: &mut Record)
    -> Result<()>
{
    type Title = PavaMgfTitleRegex;

    // Verify and parse the title line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    let captures = none_to_error!(Title::extract().captures(&line), InvalidInput);
    record.file = capture_as_string(&captures, Title::FILE_INDEX);

    let num = capture_as_str(&captures, Title::NUM_INDEX);
    record.num = from_string(num)?;

    let rt = capture_as_str(&captures, Title::RT_INDEX);
    record.rt = from_string(rt)?;

    Ok(())
}

/// Parse the pepmass header line.
#[inline(always)]
fn parse_pepmass_line<T: BufRead>(lines: &mut Lines<T>, record: &mut Record)
    -> Result<()>
{
    type PepMass = PavaMgfPepMassRegex;

    // Verify and parse the pepmass line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    let captures = none_to_error!(PepMass::extract().captures(&line), InvalidInput);

    let mz = capture_as_str(&captures, PepMass::PARENT_MZ_INDEX);
    record.parent_mz = from_string(mz)?;

    let intensity = optional_capture_as_str(&captures, PepMass::PARENT_INTENSITY_INDEX);
    record.parent_intensity = nonzero_from_string(intensity)?;

    Ok(())
}

/// Parse the charge header line.
#[inline(always)]
fn parse_charge_line<T: BufRead>(lines: &mut Lines<T>, record: &mut Record)
    -> Result<()>
{
    type Charge = PavaMgfChargeRegex;

    // Verify and parse the charge line
    let line = none_to_error!(lines.next(), InvalidInput)?;
    let captures = none_to_error!(Charge::extract().captures(&line), InvalidInput);
    let z: i8 = from_string(capture_as_str(&captures, Charge::PARENT_Z_INDEX))?;
    let sign = capture_as_str(&captures, Charge::PARENT_Z_SIGN_INDEX);
    match sign {
        "-" => record.parent_z = -z,
        "+" => record.parent_z = z,
        // The capture group recognizes exactly "-" or "+".
        _   => unreachable!(),
    }

    Ok(())
}

/// Parse the charge header line.
#[inline(always)]
fn parse_spectra<T: BufRead>(lines: &mut Lines<T>, record: &mut Record)
    -> Result<()>
{
    for result in lines {
        let line = result?;
        if line == "END IONS" {
            break;
        }

        // Parse the line data
        let mut items: Vec<&str> = Vec::with_capacity(5);
        items.extend(line.split('\t'));
        unsafe {
            if items.len() == 2 {
                // mz, intensity
                record.peaks.push(Peak {
                    mz: from_string(items.get_unchecked(0))?,
                    intensity: from_string(items.get_unchecked(1))?,
                    z: 0
                });
            } else if items.len() == 3 {
                // mz, z, intensity
                record.peaks.push(Peak {
                    mz: from_string(items.get_unchecked(0))?,
                    intensity: from_string(items.get_unchecked(2))?,
                    z: from_string(items.get_unchecked(1))?
                });
            } else {
                return Err(From::from(ErrorKind::InvalidInput));
            }
        }
    }

    Ok(())
}

/// Import record from MGF.
pub(crate) fn record_from_pava_mgf<T: BufRead>(reader: &mut T)
    -> Result<Record>
{
    let mut lines = reader.lines();
    let mut record = Record::with_peak_capacity(50);

    parse_start_line(&mut lines, &mut record)?;
    parse_title_line(&mut lines, &mut record)?;
    parse_pepmass_line(&mut lines, &mut record)?;
    parse_charge_line(&mut lines, &mut record)?;
    parse_spectra(&mut lines, &mut record)?;

    record.peaks.shrink_to_fit();
    Ok(record)
}

// READER -- DEFAULT

/// Create default record iterator from reader.
#[inline(always)]
pub(crate) fn iterator_from_pava_mgf<T: BufRead>(reader: T)
    -> MgfRecordIter<T>
{
    MgfRecordIter::new(reader, b"BEGIN IONS", MgfKind::Pava)
}
