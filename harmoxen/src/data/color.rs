#![allow(unused)]

use druid::Color;
const TAU: f64 = std::f64::consts::PI * 2.0;

fn rgb_to_hsl(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
	let cmax = r.max(g).max(b);
	let cmin = r.min(g).min(b);
	let delta = cmax - cmin;
	let hue = if delta == 0.0 {
		0.0
	} else if cmax == r {
		(TAU / 6.0) * ((g - b) / delta).rem_euclid(6.0)
	} else if cmax == g {
		(TAU / 6.0) * ((b - r) / delta + 2.0)
	} else if cmax == b {
		(TAU / 6.0) * ((r - g) / delta + 4.0)
	} else {
		unreachable!()
	};

	let lig = (cmax + cmin) / 2.0;

	let sat = if delta == 0.0 {
		0.0
	} else {
		delta / (1.0 - (2.0 * lig - 1.0).abs())
	};

	(hue, sat, lig)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
	let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
	let x = c * (1.0 - ((h / (TAU / 6.0)).rem_euclid(2.0) - 1.0).abs());
	let m = l - c / 2.0;
	let (rp, gp, bp) = match ((h / TAU) * 6.0).rem_euclid(6.0) as usize {
		0 => (c, x, 0.0),
		1 => (x, c, 0.0),
		2 => (0.0, c, x),
		3 => (0.0, x, c),
		4 => (x, 0.0, c),
		5 => (c, 0.0, x),
		_ => unreachable!(),
	};
	(rp + m, gp + m, bp + m)
}

pub fn lighten(col: &Color, intensity: f64) -> Color {
	let (r, g, b, a) = col.as_rgba();
	let (h, s, mut l) = rgb_to_hsl(r, g, b);
	l = (l + intensity).max(0.0).min(1.0);
	let (r, g, b) = hsl_to_rgb(h, s, l);
	Color::rgba(r, g, b, a)
}

pub fn invert_hue(col: &Color) -> Color {
	let (r, g, b, a) = col.as_rgba();
	let (mut h, s, l) = rgb_to_hsl(r, g, b);
	h = (h + TAU / 2.0).rem_euclid(TAU);
	let (r, g, b) = hsl_to_rgb(h, s, l);
	Color::rgba(r, g, b, a)
}

pub fn mul_alpha(col: &Color, f: f64) -> Color {
	let (r, g, b, a) = col.as_rgba();
	Color::rgba(r, g, b, a * f)
}
