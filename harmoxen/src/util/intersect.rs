use crate::data::{Line, Point, Rect};

pub fn line_line(l0: Line, l1: Line) -> Option<Point> {
	let b = l0.p1.to_vec2() - l0.p0.to_vec2();
	let d = l1.p1.to_vec2() - l1.p0.to_vec2();
	let b_outer_d = b.x * d.y - b.y * d.x;

	if b_outer_d == 0.0 {
		return None;
	}

	let c = l1.p0 - l0.p0;
	let t = (c.x * d.y - c.y * d.x) / b_outer_d;
	if t < 0.0 || t > 1.0 {
		return None;
	}

	let u = (c.x * b.y - c.y * b.x) / b_outer_d;
	if u < 0.0 || u > 1.0 {
		return None;
	}

	Some(l0.p0 + b * t)
}

pub fn line_rect(l: Line, r: Rect) -> bool {
	let c00 = Point::new(r.x0, r.y0);
	let c01 = Point::new(r.x0, r.y1);
	let c10 = Point::new(r.x1, r.y0);
	let c11 = Point::new(r.x1, r.y1);
	let mut out = false;
	out = out || line_line(l, Line::new(c00, c01)).is_some();
	out = out || line_line(l, Line::new(c01, c11)).is_some();
	out = out || line_line(l, Line::new(c11, c10)).is_some();
	out = out || line_line(l, Line::new(c10, c00)).is_some();
	out
}

pub fn rect_rect(r0: Rect, r1: Rect) -> Option<Rect> {
	r0.intersection(&r1)
}
