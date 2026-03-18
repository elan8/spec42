use elk_core::{Point, Rect, Size};

#[must_use]
pub fn rect_center(r: &Rect) -> Point {
    Point::new(r.origin.x + r.size.width / 2.0, r.origin.y + r.size.height / 2.0)
}

#[must_use]
pub fn rect_from_min_max(min: Point, max: Point) -> Rect {
    Rect::new(min, Size::new(max.x - min.x, max.y - min.y))
}

/// Remove consecutive duplicate points.
#[must_use]
pub fn dedup_points(mut points: Vec<Point>) -> Vec<Point> {
    points.dedup_by(|a, b| (a.x - b.x).abs() < f32::EPSILON && (a.y - b.y).abs() < f32::EPSILON);
    points
}

/// Simplify an orthogonal polyline by removing collinear points.
#[must_use]
pub fn simplify_orthogonal_points(points: &[Point]) -> Vec<Point> {
    if points.len() <= 2 {
        return points.to_vec();
    }
    let mut out = Vec::with_capacity(points.len());
    out.push(points[0]);
    for window in points.windows(3) {
        let a = window[0];
        let b = window[1];
        let c = window[2];
        let collinear = (a.x - b.x).abs() < f32::EPSILON && (b.x - c.x).abs() < f32::EPSILON
            || (a.y - b.y).abs() < f32::EPSILON && (b.y - c.y).abs() < f32::EPSILON;
        if !collinear {
            out.push(b);
        }
    }
    out.push(*points.last().unwrap());
    out
}

/// Convenience wrapper taking ownership.
#[must_use]
pub fn simplify_orthogonal_points_vec(points: Vec<Point>) -> Vec<Point> {
    simplify_orthogonal_points(&points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplify_removes_collinear_vertices() {
        let pts = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(20.0, 0.0),
            Point::new(20.0, 10.0),
        ];
        let simplified = simplify_orthogonal_points(&pts);
        assert_eq!(simplified, vec![pts[0], pts[2], pts[3]]);
    }
}

