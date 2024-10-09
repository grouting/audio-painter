use std::time::Duration;
use crate::debug::Stopwatch;

pub struct TimeAverage {
	durations: Vec<f32>, // in seconds,
	current_stopwatch: Option<Stopwatch>
}

impl TimeAverage {
	pub fn new() -> Self {
		Self {
			durations: vec![],
			current_stopwatch: None,
		}
	}

	pub fn start(&mut self) {
		self.current_stopwatch = Some(Stopwatch::start());
	}

	pub fn stop(&mut self) {
		if let Some(stopwatch) = &self.current_stopwatch {
			self.push(&stopwatch.stop())
		}
	}

	fn push(&mut self, time: &Duration) {
		self.durations.push(time.as_secs_f32())
	}

	pub fn average_seconds(&self) -> f32 {
		let sum: f32 = self.durations.iter().sum(); // what about overflows
		sum / self.durations.len() as f32
	}
}

