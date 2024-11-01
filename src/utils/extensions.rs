use crate::steps::tidy;
use crate::Cli;
use clap::error::{Error, ErrorKind};
use clap::Command;
use hound::{Sample, SampleFormat, WavReader, WavSpec};
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub trait IntoMutSamplesVec {
	fn collect_samples<S: Sample>(&mut self) -> Vec<S>;
}

impl IntoMutSamplesVec for WavReader<BufReader<File>> {
	fn collect_samples<S: Sample>(&mut self) -> Vec<S> {
		// is it worth figuring out if there are any err variants?
		self.samples::<S>().filter_map(|s| s.ok()).collect()
	}
}

pub fn tidy_samples(samples: Vec<i16>, channels: u16, normalize_samples: bool) -> Vec<i16> {
	let samples = tidy::flatten(samples, channels);

	if normalize_samples {
		tidy::normalize(&samples)
	} else {
		samples
	}
}

pub fn verify_file_extension<'a>(
	path: &'a PathBuf,
	command: &mut Command,
) -> clap::error::Result<&'a PathBuf, Error> {
	if path.extension().unwrap_or(OsStr::new("")) != "wav" {
		Err(command.error(ErrorKind::InvalidValue, "only wav files are supported"))
	} else {
		Ok(path)
	}
}

pub fn get_number<T: PartialOrd + Copy + Display>(
	input: T,
	min: T,
	max: T,
	input_name: &str,
	command: &mut Command,
) -> clap::error::Result<T, Error> {
	if !(min..=max).contains(&input) {
		Err(command.error(
			ErrorKind::InvalidValue,
			format!("{} can only be from {} to {}", input_name, min, max),
		))
	} else {
		Ok(input)
	}
}

pub fn get_wav_reader(
	path: &PathBuf,
	command: &mut Command,
) -> clap::error::Result<WavReader<BufReader<File>>, Error> {
	hound::WavReader::open(path).map_err(|err| hound_err_map(err, command))
}

pub fn hound_err_map(error: hound::Error, command: &mut Command) -> Error {
	match error {
		hound::Error::IoError(_error) => command.error(ErrorKind::Io, "io error"),
		hound::Error::FormatError(_) | hound::Error::TooWide | hound::Error::UnfinishedSample => {
			command.error(ErrorKind::Io, "found malformed wave data")
		}
		hound::Error::Unsupported => command.error(ErrorKind::Io, "unsupported format"),
		hound::Error::InvalidSampleFormat => command.error(ErrorKind::Io, "bad sample format"),
	}
}

pub fn get_samples(
	reader: WavReader<BufReader<File>>,
	cli: &Cli,
	spec: &WavSpec,
) -> clap::error::Result<Vec<i16>, Error> {
	match spec.sample_format {
		SampleFormat::Float => {
			// convert_samples_to_i16::<f32>(reader, cli, spec.channels)
			panic!("unsupported audio format")
		}
		SampleFormat::Int => match spec.bits_per_sample {
			// 8 => convert_samples_to_i16::<i8>(reader, cli, spec.channels),
			16 => convert_samples_to_i16::<i16>(reader, cli, spec.channels),
			// 32 => convert_samples_to_i16::<i32>(reader, cli, spec.channels),
			_ => panic!("unsupported audio format"),
		},
	}
}

fn convert_samples_to_i16<S: Sample + Copy>(
	mut reader: WavReader<BufReader<File>>,
	cli: &Cli,
	channels: u16,
) -> clap::error::Result<Vec<i16>, Error> {
	// TODO: does normalization actually work?
	let samples: Vec<i16> = reader
		.collect_samples::<S>()
		.iter()
		.map(|s| s.as_i16())
		.collect(); // TODO: as_i16 does not actually convert the samples properly

	let samples = tidy_samples(samples, channels, cli.normalize);
	Ok(samples)
}

pub fn warn_if_flattening_required(specs: (&WavSpec, &WavSpec)) {
	if specs.0.channels > 1 || specs.1.channels > 1 {
		println!("clips containing more than one channel will be converted to mono");
	}
}

pub fn throw_if_sample_rate_mismatch(
	specs: (&WavSpec, &WavSpec),
	command: &mut Command,
) -> clap::error::Result<(), Error> {
	if specs.0.sample_rate != specs.1.sample_rate {
		Err(command.error(
			ErrorKind::ValueValidation,
			"target and paint audio clips must have the same sample rate",
		))
	} else {
		Ok(())
	}
}

/// Divides the two operands, returning the answer rounded to the highest nearest integer
pub fn div_ceil(a: usize, b: usize) -> usize {
	(a as f32 / b as f32).ceil() as usize
}

/// Divides the two operands, returning the answer rounded to the lowest nearest integer
pub fn _div_floor(a: usize, b: usize) -> usize {
	(a as f32 / b as f32).floor() as usize
}
