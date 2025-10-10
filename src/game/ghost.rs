use crate::game::utils::*;
use crate::models::*;

/// Update all ghosts (movement, auto-fire, cleanup)
pub fn update_ghosts(state: &mut GameState, delta: f32) {
    // Ensure we have timers for all ghosts
    while state.ghost_fire_timers.len() < state.ghosts.len() {
        state.ghost_fire_timers.push(0.0);
    }

    // Update all fire timers
    for timer in &mut state.ghost_fire_timers {
        *timer = (*timer - delta).max(0.0);
    }

    // Update each ghost
    for (idx, ghost) in state.ghosts.iter_mut().enumerate() {
        // Movement logic - ghosts rise upward
        if ghost.pos.y > 200.0 {
            ghost.pos.y -= 150.0 * delta; // Fast ascent
        } else {
            ghost.pos.y -= 50.0 * delta; // Slow hover
        }

        // Auto-fire at nearest enemy
        if idx < state.ghost_fire_timers.len()
            && state.ghost_fire_timers[idx] <= 0.0
            && !state.enemies.is_empty()
        {
            if let Some(target) = find_nearest_enemy(ghost.pos, &state.enemies) {
                fire_ghost_weapon(ghost, target, &mut state.projectiles);
                state.ghost_fire_timers[idx] = 1.0; // Fire every second
            }
        }
    }

    // Remove off-screen ghosts
    state.ghosts.retain(|g| g.pos.y > -50.0);

    // Clean up excess timers
    state.ghost_fire_timers.truncate(state.ghosts.len());
}

/// Find the nearest enemy to a ghost
fn find_nearest_enemy(ghost_pos: Position, enemies: &[Enemy]) -> Option<Position> {
    enemies.iter().map(|e| e.pos).min_by(|a, b| {
        let dist_a = distance_sq(ghost_pos, *a);
        let dist_b = distance_sq(ghost_pos, *b);
        dist_a.partial_cmp(&dist_b).unwrap()
    })
}

/// Fire ghost weapon at target
fn fire_ghost_weapon(ghost: &Ghost, target: Position, projectiles: &mut Vec<Projectile>) {
    if let Some(&weapon) = ghost.weapon_type.first() {
        let velocity = calculate_velocity(ghost.pos, target, 350.0);

        projectiles.push(Projectile {
            pos: ghost.pos,
            velocity,
            damage: weapon.get_weapon_stats().damage,
            weapon_type: weapon,
            owner: ProjectileOwner::Ghost,
        });
    }
}
