use crate::models::{GhostFormation, Position};
use macroquad::math::Vec2;

/// Circle-to-circle collision detection
pub fn circle_collision(pos1: Position, pos2: Position, radius1: f32, radius2: f32) -> bool {
    let distance_sq = (pos1 - pos2).length_squared();
    let radii_sum = radius1 + radius2;
    distance_sq < radii_sum * radii_sum
}

/// Calculate squared distance between two positions (faster than sqrt)
#[allow(dead_code)]
pub fn distance_sq(a: Position, b: Position) -> f32 {
    (a - b).length_squared()
}

/// Normalize a direction vector and return velocity
pub fn calculate_velocity(from: Position, to: Position, speed: f32) -> Position {
    let dir = to - from;
    let distance = dir.length();

    if distance > 0.0 {
        dir.normalize() * speed
    } else {
        Vec2::ZERO
    }
}

/// Calculate lead targeting velocity (predictive shooting)
/// Predicts where the target will be when the projectile arrives
///
/// Solves the intercept problem using quadratic equation
/// - Target moves at constant velocity
/// - Projectile moves at constant speed toward intercept point
/// - Find time 't' when distance(shooter + proj_vel*t) == distance(target + target_vel*t)
pub fn calculate_lead_velocity(
    from: Position,
    target_pos: Position,
    target_velocity: Position,
    projectile_speed: f32,
) -> Position {
    // Calculate relative position
    let d = target_pos - from;

    // Get target velocity components
    let v = target_velocity;

    // Quadratic equation coefficients: a*t² + b*t + c = 0
    // Where t is the time to intercept
    let a = v.length_squared() - projectile_speed * projectile_speed;
    let b = 2.0 * d.dot(v);
    let c = d.length_squared();

    // Solve quadratic equation
    let discriminant = b * b - 4.0 * a * c;

    // If no solution or target not moving, fall back to direct aim
    if discriminant < 0.0 || a.abs() < 0.001 {
        return calculate_velocity(from, target_pos, projectile_speed);
    }

    // Get smallest positive solution (earliest intercept time)
    let sqrt_discriminant = discriminant.sqrt();
    let t1 = (-b + sqrt_discriminant) / (2.0 * a);
    let t2 = (-b - sqrt_discriminant) / (2.0 * a);

    let t = if t1 > 0.0 && t2 > 0.0 {
        t1.min(t2) // Both positive, take smallest
    } else if t1 > 0.0 {
        t1
    } else if t2 > 0.0 {
        t2
    } else {
        // No positive solution, fall back to direct aim
        return calculate_velocity(from, target_pos, projectile_speed);
    };

    // Calculate predicted intercept position
    let intercept = target_pos + target_velocity * t;

    // Aim at intercept point
    calculate_velocity(from, intercept, projectile_speed)
}

use macroquad::prelude::rand;
use macroquad::prelude::screen_width;

/// Calculate ghost spawn position based on formation
pub fn calculate_formation_position(
    player_pos: Position,
    ghost_index: usize,
    total_ghosts: usize,
    formation: GhostFormation,
    config: &crate::config::FormationSpacingConfig,
) -> Position {
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
    }
}

/// Generate a center-biased random X position for enemy spawning
/// Uses a triangular distribution to favor center positions
pub fn biased_random_x(min: f32, max: f32) -> f32 {
    // Generate two random numbers and average them
    // This creates a triangular distribution that peaks at the center
    let r1 = rand::gen_range(min, max);
    let r2 = rand::gen_range(min, max);
    (r1 + r2) / 2.0
}

/// V-shaped formation (classic attack formation)
fn calculate_v_formation(
    player_pos: Position,
    index: usize,
    _total: usize,
    config: &crate::config::FormationSpacingConfig,
) -> Position {
    let spacing = config.v_shape_spacing;
    let side = if index.is_multiple_of(2) { -1.0 } else { 1.0 };
    let offset = (index / 2) as f32 + 1.0;

    Vec2::new(
        player_pos.x + (side * offset * spacing),
        player_pos.y - (offset * spacing * config.v_shape_vertical_factor),
    )
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

    Vec2::new(
        (player_pos.x + x_offset).clamp(padding, screen_width() - padding),
        player_pos.y - config.line_height_offset,
    )
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

    Vec2::new(
        player_pos.x + angle.cos() * radius,
        player_pos.y + angle.sin() * radius,
    )
}
