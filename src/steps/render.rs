use std::path::PathBuf;
use clap::error::Error;
use crate::utils::{generate_fades, SampleSpan};

pub fn render(chunk_matches: &Vec<SampleSpan>, target_samples: &[i16], paint_samples: &[i16], chunk_size: usize, output: PathBuf, dry_wet_mix: f32) -> clap::error::Result<(), Error> {
	let spec = hound::WavSpec { // TODO:
		channels: 1,
		sample_rate: 44100,
		bits_per_sample: 16,
		sample_format: hound::SampleFormat::Int,
	};
	let mut writer = hound::WavWriter::create(output, spec).unwrap();

	let number_of_samples = count_number_of_output_samples(&chunk_matches);

	let crossfade_length = chunk_size / 4;

	let mut result: Vec<i16> = vec![0; number_of_samples];
	let mut progress: usize = 0;

	for (i, chunk) in chunk_matches.iter().enumerate() {
		let chunk_samples = &paint_samples[chunk.range()];

		let (head, tail) = generate_fades(paint_samples, &chunk, crossfade_length);
		
		if i > 0 {
			stamp(&mut result, &head, progress - crossfade_length);
		}

		stamp(&mut result, chunk_samples, progress);
		progress += chunk_samples.len();

		if i < chunk_matches.len() - 1 {
			stamp(&mut result, &tail, progress);
		}
	}

	for (i, sample) in result.into_iter().enumerate() {
		let dry_amplitude = dry_wet_mix;
		let wet_amplitude = 1. - dry_wet_mix;

		let mixed_sample = sample as f32 * wet_amplitude + target_samples[i] as f32 * dry_amplitude;

		writer.write_sample(mixed_sample as i16).unwrap() // TODO: proper error handle
	}

	writer.finalize().unwrap();

	println!("done");

	Ok(())
}

fn stamp(out: &mut Vec<i16>, input: &[i16], at: usize) {
	for i in 0..input.len() {
		out[at + i] += input[i];
	}
}

fn count_number_of_output_samples(chunks: &Vec<SampleSpan>) -> usize {
	let mut sum: usize = 0;

	for chunk in chunks {
		sum += chunk.length();
	}

	sum
}