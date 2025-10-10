use crate::game::utils::*;
use crate::models::*;
use macroquad::prelude::*;

/// Update all enemies (movement, firing, cleanup)
pub fn update_enemies(state: &mut GameState, delta: f32) {
    // Ensure we have timers for all enemies
    while state.enemy_fire_timers.len() < state.enemies.len() {
        state.enemy_fire_timers.push(rand::gen_range(1.0, 2.0));
    }

    // Update all fire timers
    for timer in &mut state.enemy_fire_timers {
        *timer = (*timer - delta).max(0.0);
    }

    // Update each enemy
    for (idx, enemy) in state.enemies.iter_mut().enumerate() {
        // Movement logic
        if enemy.pos.y < 200.0 {
            enemy.pos.y += 150.0 * delta; // Fast descent
        } else {
            enemy.pos.y += 50.0 * delta; // Slow hover
        }

        // Fire based on enemy type
        if idx < state.enemy_fire_timers.len()
            && state.enemy_fire_timers[idx] <= 0.0
            && enemy.pos.y > 50.0
        {
            fire_enemy_weapon(enemy, state.player.pos, &mut state.projectiles);
            state.enemy_fire_timers[idx] = get_fire_interval(enemy.entity_type);
        }
    }

    // Remove off-screen enemies
    state.enemies.retain(|e| e.pos.y < 650.0);

    // Clean up excess timers
    state.enemy_fire_timers.truncate(state.enemies.len());
}

/// Get fire interval based on enemy type
fn get_fire_interval(entity_type: EntityType) -> f32 {
    match entity_type {
        EntityType::BasicFighter => 2.0,
        EntityType::Sniper => 3.0,
        EntityType::Tank => 1.5,
        EntityType::Boss => 0.8,
    }
}

/// Fire enemy weapon based on type
fn fire_enemy_weapon(enemy: &Enemy, player_pos: Position, projectiles: &mut Vec<Projectile>) {
    if let Some(&weapon) = enemy.weapon.first() {
        let velocity = match enemy.entity_type {
            EntityType::BasicFighter | EntityType::Tank => {
                // Shoot straight down
                Position { x: 0.0, y: 250.0 }
            }
            EntityType::Sniper | EntityType::Boss => {
                // Aim at player
                calculate_velocity(enemy.pos, player_pos, 250.0)
            }
        };

        projectiles.push(Projectile {
            pos: enemy.pos,
            velocity,
            damage: weapon.get_weapon_stats().damage * 0.5,
            weapon_type: weapon,
            owner: ProjectileOwner::Enemy,
        });
    }
}
