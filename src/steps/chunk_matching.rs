use crate::debug::{Stopwatch, TimeAverage};
use crate::utils::{self, SampleSpan};
use clap::error::Error;
use indicatif::{ProgressBar, ProgressStyle};

pub fn make_chunk_matches(
	target_samples: &[i16],
	paint_samples: &[i16],
	chunk_size: usize,
	search_jump: usize,
	sample_rate: u32,
) -> clap::error::Result<Vec<SampleSpan>, Error> {
	let number_of_chunks = utils::div_ceil(target_samples.len(), chunk_size); // for now

	let progress_bar = ProgressBar::new(number_of_chunks as u64);
	progress_bar.set_style(ProgressStyle::with_template("{bar:50} {msg}").unwrap().progress_chars("=>-"));

	let target_chunks = target_samples.chunks(chunk_size);

	let mut chunk_matches = Vec::<SampleSpan>::new();

	let mut time_average = TimeAverage::new();
	let total_time = Stopwatch::start();

	for chunk in target_chunks {
		time_average.start();

		let chunk_length = chunk.len();

		let mut best_match: (usize, i128) = (0, i128::MAX);

		let paint_search_extent = paint_samples.len() - chunk_length;

		for i in (0..paint_search_extent).step_by(search_jump) {
			let read_range = SampleSpan::new(i, chunk_length).range(); // could squeeze a tiny optimisation out of this (using slide)
			let paint_comparison = &paint_samples[read_range];

			let mut sum: i128 = 0;

			for j in 0..chunk_length {
				let a = paint_comparison[j] as i128;
				let b = chunk[j] as i128;

				let delta = a - b;
				sum += delta * delta;
			}

			if sum < best_match.1 {
				best_match = (i, sum);
			}
		}

		chunk_matches.push(SampleSpan::new(best_match.0, chunk_length));

		time_average.stop();

		let eta = (number_of_chunks - chunk_matches.len()) as f32 * time_average.average_seconds();

		progress_bar.inc(1);
		progress_bar.set_message(format!("{} chunks of {}, eta {:.0} s", chunk_matches.len(), number_of_chunks, eta));
	}

	let total_duration = total_time.stop();

	let realtime_multiplier =
		(target_samples.len() as f32 / sample_rate as f32) / total_duration.as_secs_f32();

	progress_bar.finish_and_clear();

	println!(
		"finished matching chunks ({:.0} s, {:.2}x realtime)",
		total_duration.as_secs_f32(),
		realtime_multiplier
	);

	Ok(chunk_matches)
}
