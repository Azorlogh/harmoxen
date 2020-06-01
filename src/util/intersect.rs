use druid::{kurbo::Line, Point, Rect};

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

	Some(l0.p0 + t * b)
}

pub fn line_rect(l: Line, r: Rect) -> Option<Point> {
	let c00 = Point::new(r.x0, r.y0);
	let c01 = Point::new(r.x0, r.y1);
	let c10 = Point::new(r.x1, r.y0);
	let c11 = Point::new(r.x1, r.y1);
	let mut out = None;
	out = out.or(line_line(l, Line::new(c00, c01)));
	out = out.or(line_line(l, Line::new(c01, c11)));
	out = out.or(line_line(l, Line::new(c11, c10)));
	out = out.or(line_line(l, Line::new(c10, c00)));
	out
}
