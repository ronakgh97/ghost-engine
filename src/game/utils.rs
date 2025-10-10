use crate::models::Position;

/// Circle-to-circle collision detection
pub fn circle_collision(pos1: Position, pos2: Position, radius1: f32, radius2: f32) -> bool {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    let distance_sq = dx * dx + dy * dy;
    let radii_sum = radius1 + radius2;
    distance_sq < radii_sum * radii_sum
}

/// Calculate squared distance between two positions (faster than sqrt)
pub fn distance_sq(a: Position, b: Position) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}

/// Normalize a direction vector and return velocity
pub fn calculate_velocity(from: Position, to: Position, speed: f32) -> Position {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance > 0.0 {
        Position {
            x: (dx / distance) * speed,
            y: (dy / distance) * speed,
        }
    } else {
        Position { x: 0.0, y: 0.0 }
    }
}
