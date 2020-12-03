#![allow(non_snake_case)]
#![allow(dead_code)]
use nalgebra::*;

#[derive(Debug)]
pub struct Kernel {
	pub A: Matrix2<f32>,
	pub B: Vector2<f32>,
	pub C: Vector3<f32>,
	pub y: Vector2<f32>,
}

impl Kernel {
	pub fn eval(&mut self, x: f32) -> f32 {
		let out = self.C.dot(&Vector3::new(x, self.y[0], self.y[1]));
		self.y = self.B * x + self.A * self.y;
		out
	}
}

fn svf_core(f: f32, res: f32, m: [f32; 3]) -> Kernel {
	let k = 2.0 - 2.0 * res;
	let g = (std::f32::consts::PI * f).tan() * 1.0;
	let a1 = 1.0 / (1.0 + g * (g + k));
	let a2 = g * a1;
	let a3 = g * a2;
	let A = Matrix2::new(2.0 * a1 - 1.0, -2.0 * a2, 2.0 * a2, 1.0 - 2.0 * a3);
	let B = Vector2::new(2.0 * a2, 2.0 * a3);
	let C_v0 = Vector3::new(1.0, 0.0, 0.0);
	let C_v1 = Vector3::new(a2, a1, -a2);
	let C_v2 = Vector3::new(a3, a2, 1.0 - a3);
	let C = m[0] * C_v0 + m[1] * C_v1 + m[2] * C_v2;
	Kernel { A, B, C, y: zero() }
}

pub fn lowpass(f: f32, res: f32) -> Kernel {
	svf_core(f, res, [0.0, 0.0, 1.0])
}

pub fn bandpass(f: f32, res: f32) -> Kernel {
	svf_core(f, res, [0.0, 1.0, 0.0])
}

pub fn highpass(f: f32, res: f32) -> Kernel {
	let k = 2.0 - 2.0 * res;
	svf_core(f, res, [1.0, -k, -1.0])
}
