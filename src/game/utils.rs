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
    config: &crate::config::FormationSpacingConfig,
) -> Position {
    // If formation is invalid for the number of ghosts, default to scattered
    if !formation.is_valid_for_count(total_ghosts) {
        return calculate_scattered_formation(player_pos, config);
    }

    match formation {
        GhostFormation::VShape => {
            calculate_v_formation(player_pos, ghost_index, total_ghosts, config)
        }
        GhostFormation::Line => {
            calculate_line_formation(player_pos, ghost_index, total_ghosts, config)
        }
        GhostFormation::Circle => {
            calculate_circle_formation(player_pos, ghost_index, total_ghosts, config)
        }
        GhostFormation::Scattered => calculate_scattered_formation(player_pos, config),
    }
}

/// V-shaped formation (classic attack formation)
fn calculate_v_formation(
    player_pos: Position,
    index: usize,
    _total: usize,
    config: &crate::config::FormationSpacingConfig,
) -> Position {
    let spacing = config.v_shape_spacing;
    let side = if index % 2 == 0 { -1.0 } else { 1.0 };
    let offset = (index / 2) as f32 + 1.0;

    Position {
        x: player_pos.x + (side * offset * spacing),
        y: player_pos.y - (offset * spacing * config.v_shape_vertical_factor),
    }
}

/// Horizontal line formation (maximum firepower)
fn calculate_line_formation(
    player_pos: Position,
    index: usize,
    total: usize,
    config: &crate::config::FormationSpacingConfig,
) -> Position {
    let spacing = config.line_spacing;
    let center_offset = (total as f32 - 1.0) / 2.0;
    let x_offset = (index as f32 - center_offset) * spacing;
    let padding = config.screen_edge_padding;

    Position {
        x: (player_pos.x + x_offset).clamp(padding, screen_width() - padding),
        y: player_pos.y - config.line_height_offset,
    }
}

/// Circle formation (defensive)
fn calculate_circle_formation(
    player_pos: Position,
    index: usize,
    total: usize,
    config: &crate::config::FormationSpacingConfig,
) -> Position {
    use std::f32::consts::TAU;
    let radius = config.circle_radius;
    let angle = (index as f32 / total as f32) * TAU;

    Position {
        x: player_pos.x + angle.cos() * radius,
        y: player_pos.y + angle.sin() * radius,
    }
}

/// Scattered formation (random but aesthetic)
fn calculate_scattered_formation(
    player_pos: Position,
    config: &crate::config::FormationSpacingConfig,
) -> Position {
    use macroquad::rand::gen_range;

    Position {
        x: player_pos.x + gen_range(config.scattered_x_min, config.scattered_x_max),
        y: player_pos.y + gen_range(config.scattered_y_min, config.scattered_y_max),
    }
}
