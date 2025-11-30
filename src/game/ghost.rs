use crate::game::utils::calculate_formation_position;
use crate::game::weapons::{FireWeaponParams, FiringDirection, fire_weapon};
use crate::models::*;
use macroquad::prelude::*;

/// Update all ghosts (movement, auto-fire, cleanup)
pub fn update_ghosts(state: &mut GameState, delta: f32) {
    let screen_boundary_top = state.config.ghost_behavior.screen_boundary_top;

    // Update each ghost (fire timers now embedded in Ghost struct!)
    let total_ghosts = state.ghosts.len();
    for ghost in state.ghosts.iter_mut() {
        // Update fire timer
        ghost.fire_timer = (ghost.fire_timer - delta).max(0.0);
    }

    // Second pass for movement (need index for formation position)
    for (idx, ghost) in state.ghosts.iter_mut().enumerate() {
        // Calculate target position using current formation
        let target_pos = calculate_formation_position(
            state.player.pos,
            idx,
            total_ghosts,
            state.ghost_formation,
            &state.config.formation_spacing,
        );

        // Calculate distance to target
        let diff = target_pos - ghost.pos;
        let distance = diff.length();

        // Only move if far enough from target (prevents jitter/oscillation)
        if distance > 2.0 {
            // Smoothly interpolate to formation position (feels more natural than instant)
            let follow_speed = 3.0; // Smooth movement
            ghost.pos += diff * follow_speed * delta;
        }
    }

    // Remove off-screen ghosts (safety cleanup, shouldn't happen with formation following)
    state
        .ghosts
        .retain(|g| g.pos.y > screen_boundary_top && g.pos.y < 1000.0);
}

/// Auto-fire ghosts at enemies (called separately to avoid borrow issues)
pub fn update_ghost_firing(state: &mut GameState) {
    let fire_interval = state.config.ghost_behavior.fire_interval;

    // Collect firing data first to avoid borrow issues
    let mut fire_events: Vec<(Vec2, WeaponType, Vec2)> = Vec::new(); // (pos, weapon, target)

    // Collect enemy positions for missile targeting
    let enemy_positions: Vec<Vec2> = state.enemies.iter().map(|e| e.pos).collect();

    for ghost in state.ghosts.iter_mut() {
        if ghost.fire_timer <= 0.0 && !state.enemies.is_empty() {
            // Find nearest enemy
            if let Some(target) = find_nearest_enemy(ghost.pos, &state.enemies) {
                // Pick random weapon from arsenal
                if !ghost.weapon_type.is_empty() {
                    let random_idx = rand::gen_range(0, ghost.weapon_type.len());
                    let weapon = ghost.weapon_type[random_idx];

                    fire_events.push((ghost.pos, weapon, target));

                    // Reset fire timer
                    ghost.fire_timer = fire_interval;
                }
            }
        }
    }

    // Execute firing using unified system
    for (shooter_pos, weapon, target) in fire_events {
        fire_weapon(
            FireWeaponParams {
                shooter_pos,
                owner: ProjectileOwner::Ghost,
                weapon,
                direction: FiringDirection::AtTarget(target),
                damage_multiplier: 0.5, // Ghosts deal 50% damage
                enemies: Some(enemy_positions.clone()), // For missile targeting
            },
            state,
        );
    }
}

/// Find the nearest enemy to a ghost
fn find_nearest_enemy(ghost_pos: Position, enemies: &[Enemy]) -> Option<Position> {
    enemies.iter().map(|e| e.pos).min_by(|a, b| {
        let dist_a = (*a - ghost_pos).length_squared();
        let dist_b = (*b - ghost_pos).length_squared();
        dist_a.partial_cmp(&dist_b).unwrap()
    })
}
