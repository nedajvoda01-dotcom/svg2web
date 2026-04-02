use crate::model::geometry::{PathData, PathSegment, Point};

pub fn simplify_path(path: &PathData, tolerance: f64) -> PathData {
	if tolerance <= 0.0 {
		return path.clone();
	}

	let (points, closed) = to_polyline(path);
	if points.len() <= 2 {
		return path.clone();
	}

	let simplified = douglas_peucker(&points, tolerance);
	if simplified.len() < 2 {
		return path.clone();
	}

	let mut segments = Vec::with_capacity(simplified.len() + usize::from(closed));
	segments.push(PathSegment::MoveTo(simplified[0]));
	for point in simplified.iter().skip(1) {
		segments.push(PathSegment::LineTo(*point));
	}
	if closed {
		segments.push(PathSegment::ClosePath);
	}

	PathData { segments }
}

fn to_polyline(path: &PathData) -> (Vec<Point>, bool) {
	let mut points = Vec::new();
	let mut closed = false;

	for segment in &path.segments {
		match *segment {
			PathSegment::MoveTo(p) => points.push(p),
			PathSegment::LineTo(p) => points.push(p),
			PathSegment::CurveTo { to, .. } => points.push(to),
			PathSegment::ClosePath => closed = true,
		}
	}

	(points, closed)
}

fn douglas_peucker(points: &[Point], tolerance: f64) -> Vec<Point> {
	let n = points.len();
	let mut keep = vec![false; n];
	keep[0] = true;
	keep[n - 1] = true;

	let mut stack = vec![(0usize, n - 1usize)];
	while let Some((start, end)) = stack.pop() {
		if end <= start + 1 {
			continue;
		}

		let mut max_distance = 0.0;
		let mut max_index = start;

		for idx in (start + 1)..end {
			let distance = perpendicular_distance(points[idx], points[start], points[end]);
			if distance > max_distance {
				max_distance = distance;
				max_index = idx;
			}
		}

		if max_distance > tolerance {
			keep[max_index] = true;
			stack.push((start, max_index));
			stack.push((max_index, end));
		}
	}

	points
		.iter()
		.enumerate()
		.filter_map(|(idx, point)| keep[idx].then_some(*point))
		.collect()
}

fn perpendicular_distance(point: Point, line_start: Point, line_end: Point) -> f64 {
	let dx = line_end.x - line_start.x;
	let dy = line_end.y - line_start.y;

	if dx == 0.0 && dy == 0.0 {
		let px = point.x - line_start.x;
		let py = point.y - line_start.y;
		return (px * px + py * py).sqrt();
	}

	let numerator = ((dy * point.x) - (dx * point.y) + (line_end.x * line_start.y)
		- (line_end.y * line_start.x))
		.abs();
	let denominator = (dx * dx + dy * dy).sqrt();

	numerator / denominator
}
