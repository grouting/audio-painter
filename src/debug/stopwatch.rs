use std::time::{Duration, SystemTime};

pub struct Stopwatch {
	start: SystemTime,
}

impl Stopwatch {
	pub fn start() -> Self {
		Self {
			start: SystemTime::now()
		}
	}

	pub fn stop(&self) -> Duration {
		self.start.elapsed().unwrap()
	}
}