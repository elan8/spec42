use elk_core::{Point, PortSide};

pub fn point_along_tangent(point: Point, side: PortSide, offset: f32) -> Point {
    match side {
        PortSide::East | PortSide::West => Point::new(point.x, point.y + offset),
        PortSide::North | PortSide::South => Point::new(point.x + offset, point.y),
    }
}

pub fn simplify_orthogonal_points(points: Vec<Point>) -> Vec<Point> {
    let mut out = Vec::with_capacity(points.len());
    for point in points {
        if out.last().copied() == Some(point) {
            continue;
        }
        out.push(point);
        while out.len() >= 3 {
            let len = out.len();
            let a = out[len - 3];
            let b = out[len - 2];
            let c = out[len - 1];
            let collinear_x = (a.x - b.x).abs() <= 1e-5 && (b.x - c.x).abs() <= 1e-5;
            let collinear_y = (a.y - b.y).abs() <= 1e-5 && (b.y - c.y).abs() <= 1e-5;
            if collinear_x || collinear_y {
                out.remove(len - 2);
            } else {
                break;
            }
        }
    }
    out
}

pub fn force_orthogonal_points(points: Vec<Point>) -> Vec<Point> {
    if points.len() < 2 {
        return points;
    }
    let mut out = vec![points[0]];
    for next in points.into_iter().skip(1) {
        let current = *out.last().unwrap_or(&next);
        let dx = (current.x - next.x).abs();
        let dy = (current.y - next.y).abs();
        if dx > 1e-5 && dy > 1e-5 {
            let via = if dx >= dy {
                Point::new(next.x, current.y)
            } else {
                Point::new(current.x, next.y)
            };
            if out.last().copied() != Some(via) {
                out.push(via);
            }
        }
        if out.last().copied() != Some(next) {
            out.push(next);
        }
    }
    out
}

pub fn sanitize_orthogonal_path(
    mut points: Vec<Point>,
    actual_start: Point,
    actual_end: Point,
    start_lead: Point,
    end_lead: Point,
    route_start: Point,
    route_end: Point,
) -> Result<Vec<Point>, String> {
    if points.is_empty() {
        points.push(route_start);
        points.push(route_end);
    }
    if points.first().copied() != Some(route_start) {
        points.insert(0, route_start);
    }
    if points.last().copied() != Some(route_end) {
        points.push(route_end);
    }
    if points.len() < 2 {
        return Err("route returned fewer than two points".to_string());
    }

    let mut out = Vec::with_capacity(points.len() + 4);
    out.push(actual_start);
    if out.last().copied() != Some(start_lead) {
        out.push(start_lead);
    }
    if out.last().copied() != Some(route_start) {
        out.push(route_start);
    }
    for pair in points.windows(2) {
        let a = pair[0];
        let b = pair[1];
        if (a.x - b.x).abs() < 1e-5 || (a.y - b.y).abs() < 1e-5 {
            out.push(b);
            continue;
        }
        let dx = (b.x - a.x).abs();
        let dy = (b.y - a.y).abs();
        let via = if dx >= dy {
            Point::new(b.x, a.y)
        } else {
            Point::new(a.x, b.y)
        };
        if out.last().copied() != Some(via) {
            out.push(via);
        }
        out.push(b);
    }
    if out.last().copied() != Some(route_end) {
        out.push(route_end);
    }
    if out.last().copied() != Some(end_lead) {
        out.push(end_lead);
    }
    if out.last().copied() != Some(actual_end) {
        out.push(actual_end);
    }

    out = simplify_orthogonal_points(out);
    out = force_orthogonal_points(out);
    out = simplify_orthogonal_points(out);

    let has_diag = out.windows(2).any(|pair| {
        let a = pair[0];
        let b = pair[1];
        (a.x - b.x).abs() > 1e-5 && (a.y - b.y).abs() > 1e-5
    });
    if has_diag {
        return Err("non-orthogonal segment remained after sanitization".to_string());
    }
    Ok(out)
}
