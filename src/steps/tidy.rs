pub fn normalize(samples: &Vec<i16>) -> Vec<i16> {
	let mut highest_value: i16 = 0;

	for sample in samples {
		if sample.abs() > highest_value {
			highest_value = *sample;
		}
	}

	let gain = i16::MAX as f32 / highest_value as f32; // TODO: this right?

	let mut out = Vec::<i16>::new();

	for sample in samples {
		out.push((*sample as f32 * gain) as i16);
	}

	out
}

pub fn flatten(samples: Vec<i16>, channels: u16) -> Vec<i16> {
	if channels > 1 {
		let mut new_samples = Vec::<i16>::new();

		for i in (0..samples.len()).step_by(channels as usize) {
			let mut sum: i16 = 0;

			for j in 0..channels {
				sum += samples[i + j as usize];
			}

			new_samples.push((sum as f32 / channels as f32) as i16);
		}

		new_samples
	} else {
		samples
	}
}
