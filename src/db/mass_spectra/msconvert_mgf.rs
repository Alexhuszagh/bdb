//! Utilities to load and save MSConvert MGF files.

use std::io::prelude::*;
use std::io::Lines;
use std::iter::Peekable;

use traits::*;
use util::*;
use super::mgf::MgfRecordIter;
use super::peak::Peak;
use super::re::*;
use super::record::Record;

// SIZE

/// Estimate the size of an MSConvert MGF record.
#[inline]
pub(crate) fn estimate_msconvert_mgf_record_size(record: &Record) -> usize {
    // Actual size is ~125 with a lot of extra size for the 3x scans,
    // and the peptide RT, m/z, and intensity.
    const MGF_VOCABULARY_SIZE: usize = 200;
    // Estimated average is ~20 characters per line, assume slightly above.
    const MGF_PEAK_SIZE: usize = 25;
    MGF_VOCABULARY_SIZE +
        record.file.len() +
        record.file.len() +
        MGF_PEAK_SIZE * record.peaks.len()
}

// WRITER

#[inline(always)]
fn to_mgf<'a, T: Write>(writer: &mut T, record: &'a Record)
    -> ResultType<()>
{
    record_to_msconvert_mgf(writer, record)
}

#[inline(always)]
fn export_title<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    let num = record.num.to_string();
    write_alls!(
        writer,
        b"TITLE=", record.file.as_bytes(), b".",
        num.as_bytes(), b".", num.as_bytes(),
        b".0 File:\"", record.file.as_bytes(),
        b"\", NativeID:\"controllerType=0 controllerNumber=1 scan=",
        num.as_bytes(), b"\"\n"
    )?;

    Ok(())
}

#[inline(always)]
fn export_rt<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    let rt = record.rt.to_string();
    write_alls!(writer, b"RTINSECONDS=", rt.as_bytes(), b"\n")?;

    Ok(())
}

#[inline(always)]
fn export_pepmass<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    let parent_mz = record.parent_mz.to_string();
    write_alls!(writer, b"PEPMASS=", parent_mz.as_bytes())?;
    if record.parent_intensity != 0.0 {
        let parent_intensity = record.parent_intensity.to_string();
        write_alls!(writer, b" ", parent_intensity.as_bytes())?;
    }
    writer.write_all(b"\n")?;

    Ok(())
}

#[inline(always)]
fn export_charge<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    if record.parent_z != 1 {
        writer.write_all(b"CHARGE=")?;
        if record.parent_z > 0 {
            let parent_z = record.parent_z.to_string();
            write_alls!(writer, parent_z.as_bytes(), b"+")?;
        } else {
            let z = -record.parent_z;
            let parent_z = z.to_string();
            write_alls!(writer, parent_z.as_bytes(), b"-")?;
        }
        writer.write_all(b"\n")?;
    }

    Ok(())
}

#[inline(always)]
fn export_spectra<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    for peak in record.peaks.iter() {
        let text = format!("{:?} {:?}\n", peak.mz, peak.intensity);
        writer.write_all(text.as_bytes())?;
    }

    Ok(())
}

/// Export record to MSConvert MGF.
pub(crate) fn record_to_msconvert_mgf<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    writer.write_all(b"BEGIN IONS\n")?;
    export_title(writer, record)?;
    export_rt(writer, record)?;
    export_pepmass(writer, record)?;
    export_charge(writer, record)?;
    export_spectra(writer, record)?;
    writer.write_all(b"END IONS\n")?;

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

/// Default exporter from a non-owning iterator to MSConvert MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_msconvert_mgf<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Default exporter from an owning iterator to MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_msconvert_mgf<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to MSConvert MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_msconvert_mgf_strict<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Strict exporter from an owning iterator to MSConvert MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_msconvert_mgf_strict<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to MSConvert MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_msconvert_mgf_lenient<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Lenient exporter from an owning iterator to MSConvert MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_msconvert_mgf_lenient<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// READER

type PeakableLines<T> = Peekable<Lines<T>>;

/// Parse the start header line.
#[inline(always)]
fn parse_start_line<T: BufRead>(lines: &mut PeakableLines<T>, _: &mut Record)
    -> ResultType<()>
{
    // Verify the start header line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    bool_to_error!(line == "BEGIN IONS", InvalidInput);

    Ok(())
}

/// Parse the title header line.
#[inline(always)]
fn parse_title_line<T: BufRead>(lines: &mut PeakableLines<T>, record: &mut Record)
    -> ResultType<()>
{
    type Title = MsConvertMgfTitleRegex;

    // Verify and parse the title line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    let captures = none_to_error!(Title::extract().captures(&line), InvalidInput);
    record.file = capture_as_string(&captures, Title::FILE_INDEX);

    let num = capture_as_str(&captures, Title::NUM_INDEX);
    record.num = num.parse::<u32>()?;

    Ok(())
}

/// Parse the RT header line.
#[inline(always)]
fn parse_rt_line<T: BufRead>(lines: &mut PeakableLines<T>, record: &mut Record)
    -> ResultType<()>
{
    type Rt = MsConvertMgfRtRegex;

    // Verify and parse the RT line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    let captures = none_to_error!(Rt::extract().captures(&line), InvalidInput);

    let rt = capture_as_str(&captures, Rt::RT_INDEX);
    record.rt = rt.parse::<f64>()?;

    Ok(())
}

/// Parse the pepmass header line.
#[inline(always)]
fn parse_pepmass_line<T: BufRead>(lines: &mut PeakableLines<T>, record: &mut Record)
    -> ResultType<()>
{
    type PepMass = MsConvertMgfPepMassRegex;

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
fn parse_charge_line<T: BufRead>(lines: &mut PeakableLines<T>, record: &mut Record)
    -> ResultType<()>
{
    type Charge = MsConvertMgfChargeRegex;

    // Check if the following line is an error or a charge line.
    let is_err: bool;
    let is_charge: bool;
    {
        let peeked_line = none_to_error!(lines.peek(), InvalidInput);
        match peeked_line {
            Err(_) => {
                is_err = true;
                is_charge = false;
            },
            Ok(ref v) => {
                is_err = false;
                is_charge = v.starts_with("CHARGE=");
            }
        }
    }

    // Process the optional charge line.
    if is_err {
        // Return an error if the line
        return Err(From::from(lines.next().unwrap().unwrap_err()));
    } else if is_charge {
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
    } else {
        // Do not consume the peeked value!
        record.parent_z = 1;
    }

    Ok(())
}

/// Parse the charge header line.
#[inline(always)]
fn parse_spectra<T: BufRead>(lines: &mut PeakableLines<T>, record: &mut Record)
    -> ResultType<()>
{
    for result in lines {
        let line = result?;
        if line == "END IONS" {
            break;
        }

        // Parse the line data
        let mut items = line.split(' ');
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
pub(crate) fn record_from_msconvert_mgf<T: BufRead>(reader: &mut T)
    -> ResultType<Record>
{
    let mut lines = reader.lines().peekable();
    let mut record = Record::with_peak_capacity(50);

    parse_start_line(&mut lines, &mut record)?;
    parse_title_line(&mut lines, &mut record)?;
    parse_rt_line(&mut lines, &mut record)?;
    parse_pepmass_line(&mut lines, &mut record)?;
    parse_charge_line(&mut lines, &mut record)?;
    parse_spectra(&mut lines, &mut record)?;

    record.peaks.shrink_to_fit();
    Ok(record)
}

// READER -- DEFAULT

/// Create default record iterator from reader.
#[inline(always)]
pub(crate) fn iterator_from_msconvert_mgf<T: BufRead>(reader: T)
    -> MgfRecordIter<T>
{
    MgfRecordIter::new(reader, "BEGIN IONS", MgfKind::MsConvert)
}
