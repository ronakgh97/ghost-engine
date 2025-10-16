use crate::models::Position;

/// Cubic Bezier curve interpolation
/// P(t) = (1-t)³*P0 + 3(1-t)²*t*P1 + 3(1-t)*t²*P2 + t³*P3
///
/// # Arguments
/// * `p0` - Start point
/// * `p1` - First control point
/// * `p2` - Second control point
/// * `p3` - End point
/// * `t` - Progress along curve (0.0 to 1.0)
pub fn cubic_bezier(p0: Position, p1: Position, p2: Position, p3: Position, t: f32) -> Position {
    let t = t.clamp(0.0, 1.0);
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;

    Position {
        x: mt3 * p0.x + 3.0 * mt2 * t * p1.x + 3.0 * mt * t2 * p2.x + t3 * p3.x,
        y: mt3 * p0.y + 3.0 * mt2 * t * p1.y + 3.0 * mt * t2 * p2.y + t3 * p3.y,
    }
}

/// Quadratic Bezier curve interpolation (simpler, 3 points)
/// P(t) = (1-t)²*P0 + 2(1-t)*t*P1 + t²*P2
///
/// # Arguments
/// * `p0` - Start point
/// * `p1` - Control point
/// * `p2` - End point
/// * `t` - Progress along curve (0.0 to 1.0)
pub fn quadratic_bezier(p0: Position, p1: Position, p2: Position, t: f32) -> Position {
    let t = t.clamp(0.0, 1.0);
    let t2 = t * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;

    Position {
        x: mt2 * p0.x + 2.0 * mt * t * p1.x + t2 * p2.x,
        y: mt2 * p0.y + 2.0 * mt * t * p1.y + t2 * p2.y,
    }
}

/// Get the tangent (direction) of a cubic Bezier curve at point t
/// Useful for orienting sprites along the path
pub fn cubic_bezier_tangent(
    p0: Position,
    p1: Position,
    p2: Position,
    p3: Position,
    t: f32,
) -> Position {
    let t = t.clamp(0.0, 1.0);
    let t2 = t * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;

    // Derivative of cubic Bezier
    Position {
        x: 3.0 * mt2 * (p1.x - p0.x) + 6.0 * mt * t * (p2.x - p1.x) + 3.0 * t2 * (p3.x - p2.x),
        y: 3.0 * mt2 * (p1.y - p0.y) + 6.0 * mt * t * (p2.y - p1.y) + 3.0 * t2 * (p3.y - p2.y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_bezier_endpoints() {
        let p0 = Position { x: 0.0, y: 0.0 };
        let p1 = Position { x: 100.0, y: 100.0 };
        let p2 = Position { x: 200.0, y: 100.0 };
        let p3 = Position { x: 300.0, y: 0.0 };

        // At t=0, should be at start point
        let start = cubic_bezier(p0, p1, p2, p3, 0.0);
        assert_eq!(start.x, 0.0);
        assert_eq!(start.y, 0.0);

        // At t=1, should be at end point
        let end = cubic_bezier(p0, p1, p2, p3, 1.0);
        assert_eq!(end.x, 300.0);
        assert_eq!(end.y, 0.0);
    }

    #[test]
    fn test_quadratic_bezier_midpoint() {
        let p0 = Position { x: 0.0, y: 0.0 };
        let p1 = Position { x: 50.0, y: 100.0 };
        let p2 = Position { x: 100.0, y: 0.0 };

        // At t=0.5, should be pulled toward control point
        let mid = quadratic_bezier(p0, p1, p2, 0.5);
        assert_eq!(mid.x, 50.0);
        assert_eq!(mid.y, 50.0); // Pulled up by control point
    }
}
