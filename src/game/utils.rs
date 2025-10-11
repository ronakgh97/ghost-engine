use crate::models::{GhostFormation, Position};

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

use macroquad::prelude::screen_width;

/// Calculate ghost spawn position based on formation
pub fn calculate_formation_position(
    player_pos: Position,
    ghost_index: usize,
    total_ghosts: usize,
    formation: GhostFormation,
) -> Position {
    // If formation is invalid for the number of ghosts, default to scattered
    if !formation.is_valid_for_count(total_ghosts) {
        return calculate_scattered_formation(player_pos);
    }

    match formation {
        GhostFormation::VShape => calculate_v_formation(player_pos, ghost_index, total_ghosts),
        GhostFormation::Line => calculate_line_formation(player_pos, ghost_index, total_ghosts),
        GhostFormation::Circle => calculate_circle_formation(player_pos, ghost_index, total_ghosts),
        GhostFormation::Scattered => calculate_scattered_formation(player_pos),
    }
}

/// V-shaped formation (classic attack formation)
fn calculate_v_formation(player_pos: Position, index: usize, total: usize) -> Position {
    let spacing = 40.0;
    let side = if index % 2 == 0 { -1.0 } else { 1.0 };
    let offset = (index / 2) as f32 + 1.0;

    Position {
        x: player_pos.x + (side * offset * spacing),
        y: player_pos.y - (offset * spacing * 0.8), // Rise upward in V
    }
}

/// Horizontal line formation (maximum firepower)
fn calculate_line_formation(player_pos: Position, index: usize, total: usize) -> Position {
    let spacing = 50.0;
    let center_offset = (total as f32 - 1.0) / 2.0;
    let x_offset = (index as f32 - center_offset) * spacing;

    Position {
        x: (player_pos.x + x_offset).clamp(30.0, screen_width() - 30.0),
        y: player_pos.y - 80.0, // All same height above player
    }
}

/// Circle formation (defensive)
fn calculate_circle_formation(player_pos: Position, index: usize, total: usize) -> Position {
    use std::f32::consts::TAU;
    let radius = 70.0;
    let angle = (index as f32 / total as f32) * TAU;

    Position {
        x: player_pos.x + angle.cos() * radius,
        y: player_pos.y + angle.sin() * radius,
    }
}

/// Scattered formation (random but aesthetic)
fn calculate_scattered_formation(player_pos: Position) -> Position {
    use macroquad::rand::gen_range;

    Position {
        x: player_pos.x + gen_range(-80.0, 80.0),
        y: player_pos.y + gen_range(-100.0, -40.0),
    }
}
