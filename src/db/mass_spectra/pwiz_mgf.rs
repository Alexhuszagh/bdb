//! Utilities to load and save ProteoWizard MGF files.

use std::io::prelude::*;
//use std::io::Lines;

use traits::*;
use util::*;
use super::mgf::MgfRecordIter;
//use super::peak::Peak;
//use super::re::*;
use super::record::Record;

// SIZE

/// Estimate the size of an MSConvert MGF record.
#[inline]
#[allow(unused)]        // TODO(ahuszagh):  Remove
pub(crate) fn estimate_pwiz_mgf_record_size(record: &Record) -> usize {
    // TODO(ahuszagh) Implement
    0
}

// WRITER

#[inline(always)]
fn to_mgf<'a, T: Write>(writer: &mut T, record: &'a Record)
    -> ResultType<()>
{
    record_to_pwiz_mgf(writer, record)
}

// TODO(ahuszagh): Add

/// Export record to MSConvert MGF.
#[allow(unused)]        // TODO(ahuszagh):  Remove
pub(crate) fn record_to_pwiz_mgf<T: Write>(writer: &mut T, record: &Record)
    -> ResultType<()>
{
    writer.write_all(b"BEGIN IONS\n")?;
//    export_title(writer, record)?;
//    export_rt(writer, record)?;
//    export_pepmass(writer, record)?;
//    export_charge(writer, record)?;
//    export_spectra(writer, record)?;
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

/// Default exporter from a non-owning iterator to Pwiz MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_pwiz_mgf<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Default exporter from an owning iterator to MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_pwiz_mgf<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- STRICT

/// Strict exporter from a non-owning iterator to Pwiz MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_pwiz_mgf_strict<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Strict exporter from an owning iterator to Pwiz MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_pwiz_mgf_strict<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export_strict(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// WRITER -- LENIENT

/// Lenient exporter from a non-owning iterator to Pwiz MGF.
#[inline(always)]
pub(crate) fn reference_iterator_to_pwiz_mgf_lenient<'a, Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = &'a Record>
{
    reference_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

/// Lenient exporter from an owning iterator to Pwiz MGF.
#[inline(always)]
pub(crate) fn value_iterator_to_pwiz_mgf_lenient<Iter, T>(writer: &mut T, iter: Iter)
    -> ResultType<()>
    where T: Write,
          Iter: Iterator<Item = ResultType<Record>>
{
    value_iterator_export_lenient(writer, iter, b'\n', &init_cb, &export_cb, &dest_cb)
}

// READER

/// Import record from MGF.
#[allow(unused)]
pub(crate) fn record_from_pwiz_mgf<T: BufRead>(reader: &mut T)
    -> ResultType<Record>
{
    let mut lines = reader.lines();
    let mut record = Record::with_peak_capacity(50);

//    parse_start_line(&mut lines, &mut record)?;
//    parse_title_line(&mut lines, &mut record)?;
//    parse_rt_line(&mut lines, &mut record)?;
//    parse_pepmass_line(&mut lines, &mut record)?;
//    parse_charge_line(&mut lines, &mut record)?;
//    parse_spectra(&mut lines, &mut record)?;

    record.peaks.shrink_to_fit();
    Ok(record)
}

// READER -- DEFAULT

/// Create default record iterator from reader.
#[inline(always)]
pub(crate) fn iterator_from_pwiz_mgf<T: BufRead>(reader: T)
    -> MgfRecordIter<T>
{
    MgfRecordIter::new(reader, "BEGIN IONS", MgfKind::Pwiz)
}
