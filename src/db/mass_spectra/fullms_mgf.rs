//! Utilities to load and save Pava FullMS MGF files.

use std::io::prelude::*;
use std::io::Lines;

use traits::*;
use util::*;
use super::mgf::MgfRecordIter;
use super::peak::Peak;
use super::re::*;
use super::record::Record;

// SIZE

/// Estimate the size of a Pava FullMS MGF record.
#[inline]
pub(crate) fn estimate_fullms_mgf_record_size(record: &Record) -> usize {
    // Actual size is ~100 with a lot of extra size for the scan,
    // and the peptide RT, average m/z and intensity.
    const MGF_VOCABULARY_SIZE: usize = 175;
    // Estimated average is ~20 characters per line, assume slightly above.
    const MGF_PEAK_SIZE: usize = 25;
    MGF_VOCABULARY_SIZE + MGF_PEAK_SIZE * record.peaks.len()
}

// WRITER

#[inline(always)]
fn to_mgf<'a, T: Write>(writer: &mut T, record: &'a Record)
    -> ResultType<()>
{
    record_to_fullms_mgf(writer, record)
}

#[inline(always)]
fn export_scan<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    let num = record.num.ntoa()?;
    write_alls!(writer, b"Scan#: ", num.as_bytes(), b"\n")?;

    Ok(())
}

#[inline(always)]
fn export_rt<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    let rt = record.rt.ntoa()?;
    write_alls!(writer, b"Ret.Time: ", rt.as_bytes(), b"\n")?;

    Ok(())
}

#[inline(always)]
#[allow(unused)]    // TODO(ahuszagh)   Remove...
fn export_basepeak_mz<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    // TODO(ahuszagh)   Implement...
//    let rt = record.rt.ntoa()?;
//    write_alls!(writer, b"Ret.Time: ", rt.as_bytes(), b"\n")?;

    Ok(())
}

#[inline(always)]
#[allow(unused)]    // TODO(ahuszagh)   Remove...
fn export_basepeak_intensity<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    // TODO(ahuszagh)   Implement...
//    let rt = record.rt.ntoa()?;
//    write_alls!(writer, b"Ret.Time: ", rt.as_bytes(), b"\n")?;

    Ok(())
}

#[inline(always)]
fn export_spectra<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    for peak in record.peaks.iter() {
        let mz = peak.mz.ntoa()?;
        let intensity = peak.intensity.ntoa()?;
        write_alls!(writer, mz.as_bytes(), b"\t", intensity.as_bytes(), b"\n")?;
    }

    Ok(())
}

/// Export record to Pava FullMS MGF.
pub(crate) fn record_to_fullms_mgf<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    export_scan(writer, record)?;
    export_rt(writer, record)?;
    // Export null values,since we don't store this information.
    writer.write_all(b"IonInjectionTime(ms): 0.0\nTotalIonCurrent: 0\n")?;
    export_basepeak_mz(writer, record)?;
    export_basepeak_intensity(writer, record)?;
    export_spectra(writer, record)?;
    writer.write_all(b"\n\n")?;

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

/// Default exporter from a non-owning iterator to Pava FullMS MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_fullms_mgf<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Default exporter from an owning iterator to Pava FullMS MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_fullms_mgf<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to Pava FullMS MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_fullms_mgf_strict<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Strict exporter from an owning iterator to Pava FullMS MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_fullms_mgf_strict<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to Pava FullMS MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_fullms_mgf_lenient<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Lenient exporter from an owning iterator to Pava FullMS MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_fullms_mgf_lenient<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// READER

/// Parse the title header line.
#[inline(always)]
fn parse_scan_line<T: BufRead>(lines: &mut Lines<T>, record: &mut Record)
    -> ResultType<()>
{
    type Scan = FullMsMgfScanRegex;

    // Verify and parse the scan line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    let captures = none_to_error!(Scan::extract().captures(&line), InvalidInput);

    let num = capture_as_str(&captures, Scan::NUM_INDEX);
    record.num = num.parse::<u32>()?;

    Ok(())
}

/// Parse the RT header line.
#[inline(always)]
fn parse_rt_line<T: BufRead>(lines: &mut Lines<T>, record: &mut Record)
    -> ResultType<()>
{
    type Rt = FullMsMgfRtRegex;

    // Verify and parse the RT line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    let captures = none_to_error!(Rt::extract().captures(&line), InvalidInput);

    let rt = capture_as_str(&captures, Rt::RT_INDEX);
    record.rt = rt.parse::<f64>()?;

    Ok(())
}

/// Parse the ion injection time line.
#[inline(always)]
fn parse_ion_injection_time_line<T: BufRead>(lines: &mut Lines<T>, _: &mut Record)
    -> ResultType<()>
{
    // Verify the ion injection time line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    bool_to_error!(line.starts_with("IonInjectionTime(ms): "), InvalidInput);

    Ok(())
}

/// Parse the total ion current line.
#[inline(always)]
fn parse_total_ion_current_line<T: BufRead>(lines: &mut Lines<T>, _: &mut Record)
    -> ResultType<()>
{
    // Verify the total ion current line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    bool_to_error!(line.starts_with("TotalIonCurrent: "), InvalidInput);

    Ok(())
}

/// Parse the basepeak mass line.
#[inline(always)]
fn parse_basepeak_mass_line<T: BufRead>(lines: &mut Lines<T>, _: &mut Record)
    -> ResultType<()>
{
    // Verify the basepeak mass line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    bool_to_error!(line.starts_with("BasePeakMass: "), InvalidInput);

    Ok(())
}

/// Parse the basepeak intensity line.
#[inline(always)]
fn parse_basepeak_intensity_line<T: BufRead>(lines: &mut Lines<T>, _: &mut Record)
    -> ResultType<()>
{
    // Verify the basepeak intensity line.
    let line = none_to_error!(lines.next(), InvalidInput)?;
    bool_to_error!(line.starts_with("BasePeakIntensity: "), InvalidInput);

    Ok(())
}

/// Parse the charge header line.
#[inline(always)]
fn parse_spectra<T: BufRead>(lines: &mut Lines<T>, record: &mut Record)
    -> ResultType<()>
{
    for result in lines {
        let line = result?;
        if line.is_empty() {
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
pub(crate) fn record_from_fullms_mgf<T: BufRead>(reader: &mut T)
    -> ResultType<Record>
{
    let mut lines = reader.lines();
    let mut record = Record::with_peak_capacity(50);

    parse_scan_line(&mut lines, &mut record)?;
    parse_rt_line(&mut lines, &mut record)?;
    parse_ion_injection_time_line(&mut lines, &mut record)?;
    parse_total_ion_current_line(&mut lines, &mut record)?;
    parse_basepeak_mass_line(&mut lines, &mut record)?;
    parse_basepeak_intensity_line(&mut lines, &mut record)?;
    parse_spectra(&mut lines, &mut record)?;

    record.peaks.shrink_to_fit();
    Ok(record)
}

// READER -- DEFAULT

/// Create default record iterator from reader.
#[inline(always)]
pub(crate) fn iterator_from_fullms_mgf<T: BufRead>(reader: T)
    -> MgfRecordIter<T>
{
    MgfRecordIter::new(reader, "Scan#:", MgfKind::FullMs)
}
