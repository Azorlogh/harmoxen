//! haha, I don't know what I'm doing

const LENGTH: usize = 2048;

pub struct Limiter {
	buf_pos: usize,
	buf: [f64; LENGTH],
	rms: f64,
}

impl Limiter {
	pub fn new() -> Limiter {
		Limiter {
			buf_pos: 0,
			buf: [0.0; LENGTH],
			rms: 0.0,
		}
	}

	pub fn eval(&mut self, input: f64) -> f64 {
		self.buf_pos = (self.buf_pos + 1) % LENGTH;
		let old = self.buf[self.buf_pos];
		self.buf[self.buf_pos] = input;
		self.rms = ((self.rms * self.rms * LENGTH as f64 - old * old + input * input) / LENGTH as f64)
			.max(0.0)
			.sqrt();
		input / self.rms.max(1.0)
	}
}
