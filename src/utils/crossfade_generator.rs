use crate::utils::SampleSpan;

pub fn generate_fades(samples: &[i16], clip_range: &SampleSpan, crossfade_length: usize) -> (Vec<i16>, Vec<i16>) {
	// If the intended leading crossfade exceeds the bounds of the samples provided, 
	// snap to the closest available sample (0)
	let head_position: usize = if let (position, false) = clip_range.start().overflowing_sub(crossfade_length) {
		position
	} else {
		0
	};

	// If the intended closing crossfade exceeds the bounds of the samples provided, 
	// there's no more samples to choose from, so the tail is 0 in length
	let tail_length: usize = if (clip_range.end() + crossfade_length) < samples.len() {
		crossfade_length
	} else {
		0
	};
	
	let head_range = SampleSpan::new(head_position, clip_range.start() - head_position).range();
	let tail_range = SampleSpan::new(clip_range.end(), tail_length).range();

	let head_length = head_range.len();
	let tail_length = tail_range.len();

	let head = samples[head_range].enumerate(|(i, s)| apply(s, fade(i, head_length)));
	let tail = samples[tail_range].enumerate(|(i, s)| apply(s, 1. - fade(i, tail_length)));

	(head, tail)
}

fn apply(sample: &i16, multiplier: f32) -> i16 {
	(*sample as f32 * multiplier) as i16
}

fn fade(i: usize, length: usize) -> f32 {
	i as f32 / length as f32
}

trait Enumerate {
	fn enumerate<F: Fn((usize, &i16)) -> i16>(self, f: F) -> Vec<i16>;
}

impl Enumerate for &[i16] {
	fn enumerate<F: Fn((usize, &i16)) -> i16>(self, f: F) -> Vec<i16> {
		self.iter().enumerate().map(f).collect()
	}
}