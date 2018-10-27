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
#[allow(unused)] // TODO(ahuszagh) Remove
pub(crate) fn estimate_pava_mgf_record_size(record: &Record) -> usize {
    // TODO(ahuszagh)   Implement
    0
}

// WRITER

#[inline(always)]
fn to_mgf<'a, T: Write>(writer: &mut T, record: &'a Record)
    -> ResultType<()>
{
    record_to_pava_mgf(writer, record)
}

#[inline(always)]
fn export_title<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    let num = record.num.ntoa()?;
    let rt = format!("{:?}", record.rt);
    write_alls!(
        writer,
        b"TITLE=Scan ", num.as_bytes(), b" (rt=", rt.as_bytes(),
        b") [", record.file.as_bytes(), b"]\n"
    )?;

    Ok(())
}

#[inline(always)]
fn export_pepmass<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    let parent_mz = record.parent_mz.ntoa()?;
    write_alls!(writer, b"PEPMASS=", parent_mz.as_bytes())?;
    if record.parent_intensity != 0.0 {
        let parent_intensity = record.parent_intensity.ntoa()?;
        write_alls!(writer, b"\t", parent_intensity.as_bytes())?;
    }
    writer.write_all(b"\n")?;

    Ok(())
}

#[inline(always)]
fn export_charge<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    writer.write_all(b"CHARGE=")?;
    if record.parent_z > 0 {
        let parent_z = record.parent_z.ntoa()?;
        write_alls!(writer, parent_z.as_bytes(), b"+")?;
    } else {
        let z = -record.parent_z;
        let parent_z = z.ntoa()?;
        write_alls!(writer, parent_z.as_bytes(), b"-")?;
    }
    writer.write_all(b"\n")?;

    Ok(())
}

#[inline(always)]
fn export_spectra<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    for peak in record.peaks.iter() {
        let text = format!("{:?}\t{:?}\n", peak.mz, peak.intensity);
        writer.write_all(text.as_bytes())?;
    }

    Ok(())
}

/// Export record to PAVA MGF.
pub(crate) fn record_to_pava_mgf<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
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
    -> ResultType<TextWriterState<T>>
{
    Ok(TextWriterState::new(writer, delimiter))
}

#[inline(always)]
fn export_cb<'a, T: Write>(writer: &mut TextWriterState<T>, record: &'a Record)
    -> ResultType<()>
{
    writer.export(record, &to_mgf)
}

#[inline(always)]
fn dest_cb<T: Write>(_: &mut TextWriterState<T>)
    -> ResultType<()>
{
    Ok(())
}

/// Default exporter from a non-owning iterator to Pava MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_pava_mgf<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Default exporter from an owning iterator to Pava MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_pava_mgf<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to Pava MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_pava_mgf_strict<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Strict exporter from an owning iterator to Pava MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_pava_mgf_strict<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to Pava MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_pava_mgf_lenient<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Lenient exporter from an owning iterator to Pava MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_pava_mgf_lenient<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// READER

/// Parse the start header line.
#[inline(always)]
fn parse_start_line<T: BufRead>(lines: &mut Lines<T>, _: &mut Record)
    -> ResultType<()>
{
    // Verify the start header line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    bool_to_error!(line == "BEGIN IONS", InvalidInput);

    Ok(())
}

/// Parse the title header line.
#[inline(always)]
fn parse_title_line<T: BufRead>(lines: &mut Lines<T>, record: &mut Record)
    -> ResultType<()>
{
    type Title = PavaMgfTitleRegex;

    // Verify and parse the title line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    let captures = none_to_error!(Title::extract().captures(&line), InvalidInput);
    record.file = capture_as_string(&captures, Title::FILE_INDEX);

    let num = capture_as_str(&captures, Title::NUM_INDEX);
    record.num = num.parse::<u32>()?;

    let rt = capture_as_str(&captures, Title::RT_INDEX);
    record.rt = rt.parse::<f64>()?;

    Ok(())
}

/// Parse the pepmass header line.
#[inline(always)]
fn parse_pepmass_line<T: BufRead>(lines: &mut Lines<T>, record: &mut Record)
    -> ResultType<()>
{
    type PepMass = PavaMgfPepMassRegex;

    // Verify and parse the pepmass line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    let captures = none_to_error!(PepMass::extract().captures(&line), InvalidInput);

    let mz = capture_as_str(&captures, PepMass::PARENT_MZ_INDEX);
    record.parent_mz = mz.parse::<f64>()?;

    let intensity = optional_capture_as_str(&captures, PepMass::PARENT_INTENSITY_INDEX);
    record.parent_intensity = nonzero_float_from_string!(intensity, f64)?;

    Ok(())
}

/// Parse the charge header line.
#[inline(always)]
fn parse_charge_line<T: BufRead>(lines: &mut Lines<T>, record: &mut Record)
    -> ResultType<()>
{
    type Charge = PavaMgfChargeRegex;

    // Verify and parse the charge line
    let line = lines.next().unwrap()?;
    let captures = none_to_error!(Charge::extract().captures(&line), InvalidInput);
    let z = capture_as_str(&captures, Charge::PARENT_Z_INDEX).parse::<i8>()?;
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
    -> ResultType<()>
{
    for result in lines {
        let line = result?;
        if line == "END IONS" {
            break;
        }

        // Parse the line data
        let mut items = line.split('\t');
        let mz = none_to_error!(items.next(), InvalidInput);
        let intensity = none_to_error!(items.next(), InvalidInput);
        bool_to_error!(items.next().is_none(), InvalidInput);

        record.peaks.push(Peak {
            mz: mz.parse::<f64>()?,
            intensity: intensity.parse::<f64>()?,
            z: 0,
        });
    }

    Ok(())
}

/// Import record from MGF.
pub(crate) fn record_from_pava_mgf<T: BufRead>(reader: &mut T)
    -> ResultType<Record>
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
    MgfRecordIter::new(reader, "BEGIN IONS", MgfKind::Pava)
}