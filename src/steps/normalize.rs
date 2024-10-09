pub fn normalize(samples: &Vec<i16>) -> Vec<i16> {
	let mut highest_value = 0;

	for sample in samples {
		if sample.abs() > highest_value {
			highest_value = *sample;
		}
	}

	let gain = i16::MAX as f32 / highest_value as f32;

	let mut out = Vec::<i16>::new();

	for sample in samples {
		out.push((*sample as f32 * gain) as i16);
	}

	out
}