use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use clap::Command;
use clap::error::{Error, ErrorKind};
use hound::{WavReader};
use crate::steps::normalize;

pub trait IntoMutSamplesVec {
    fn collect_samples(&mut self) -> Vec<i16>;
}

impl IntoMutSamplesVec for WavReader<BufReader<File>> {
    fn collect_samples(&mut self) -> Vec<i16> { // is it worth figuring out if there are any err variants?
        self.samples::<i16>().filter_map(|s| s.ok()).collect()
    }
}

pub fn normalize_if_required(samples: Vec<i16>, normalize_samples: bool) -> Vec<i16> {
	if normalize_samples {
		normalize(&samples)
	} else {
		samples
	}
}

pub fn verify_file_extension(path: PathBuf, command: &mut Command) -> clap::error::Result<PathBuf, Error> {
	if path.extension().unwrap_or(OsStr::new("")) != "wav" {
		Err(command.error(ErrorKind::InvalidValue, "only wav files are supported"))
	} else {
		Ok(path)
	}
}

pub fn get_number<T: PartialOrd + Copy + Display>(input: T, min: T, max: T, input_name: &str, command: &mut Command) -> clap::error::Result<T, Error> {
	if !(min..=max).contains(&input) {
		Err(command.error(ErrorKind::InvalidValue, format!("{} can only be from {} to {}", input_name, min, max)))
	} else {
		Ok(input)
	}
}

pub fn get_wav_reader(path: PathBuf, command: &mut Command) -> clap::error::Result<WavReader<BufReader<File>>, Error> {
	hound::WavReader::open(path).map_err(|err| hound_err_map(err, command))
}

pub fn hound_err_map(error: hound::Error, command: &mut Command) -> Error {
	match error {
		hound::Error::IoError(_error) => {
			command.error(ErrorKind::Io, "io error")
		},
		hound::Error::FormatError(_) | hound::Error::TooWide | hound::Error::UnfinishedSample => command.error(ErrorKind::Io, "found malformed wave data"),
		hound::Error::Unsupported => command.error(ErrorKind::Io, "unsupported format"),
		hound::Error::InvalidSampleFormat => command.error(ErrorKind::Io, "bad sample format"),
	}
}

/// Divides the two operands, returning the answer rounded to the highest nearest integer
pub fn div_ceil(a: usize, b: usize) -> usize {
	(a as f64 / b as f64).ceil() as usize
}

/// Divides the two operands, returning the answer rounded to the lowest nearest integer
pub fn div_floor(a: usize, b: usize) -> usize {
	(a as f64 / b as f64).floor() as usize
}