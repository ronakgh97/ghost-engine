use crate::game::utils::*;
use crate::models::*;
use macroquad::prelude::*;

/// Update all enemies (movement, firing, cleanup)
pub fn update_enemies(state: &mut GameState, delta: f32) {
    let enemy_cfg = &state.config.enemy_behavior;

    // Ensure we have timers for all enemies
    while state.enemy_fire_timers.len() < state.enemies.len() {
        state.enemy_fire_timers.push(rand::gen_range(1.0, 3.0));
    }

    // Update all fire timers
    for timer in &mut state.enemy_fire_timers {
        *timer = (*timer - delta).max(0.0);
    }

    // Update each enemy
    for (idx, enemy) in state.enemies.iter_mut().enumerate() {
        // Movement logic
        if enemy.pos.y < enemy_cfg.movement_threshold_y {
            enemy.pos.y += enemy_cfg.fast_descent_speed * delta;
        } else {
            enemy.pos.y += enemy_cfg.slow_hover_speed * delta;
        }

        // Fire based on enemy type
        if idx < state.enemy_fire_timers.len()
            && state.enemy_fire_timers[idx] <= 0.0
            && enemy.pos.y > enemy_cfg.fire_threshold_y
        {
            fire_enemy_weapon(
                enemy,
                state.player.pos,
                &mut state.projectiles,
                enemy_cfg.basic_projectile_speed_y,
                &state.config.weapons,
            );
            state.enemy_fire_timers[idx] =
                enemy.entity_type.get_fire_interval(&state.config.entities);
        }
    }

    // Remove off-screen enemies
    state
        .enemies
        .retain(|e| e.pos.y < enemy_cfg.screen_boundary_bottom);

    // Clean up excess timers
    state.enemy_fire_timers.truncate(state.enemies.len());
}

/// Fire enemy weapon based on type
fn fire_enemy_weapon(
    enemy: &Enemy,
    player_pos: Position,
    projectiles: &mut Vec<Projectile>,
    basic_projectile_speed_y: f32,
    weapons_config: &crate::config::WeaponsConfig,
) {
    if let Some(&weapon) = enemy.weapon.first() {
        let velocity = match enemy.entity_type {
            EntityType::BasicFighter | EntityType::Tank => {
                // Shoot straight down
                Position {
                    x: 0.0,
                    y: basic_projectile_speed_y,
                }
            }
            EntityType::Sniper | EntityType::Boss => {
                // Aim at player (use same speed for consistency)
                calculate_velocity(enemy.pos, player_pos, basic_projectile_speed_y)
            }
        };

        projectiles.push(Projectile {
            pos: enemy.pos,
            velocity,
            damage: weapon.get_weapon_stats(weapons_config).damage * 0.5,
            weapon_type: weapon,
            owner: ProjectileOwner::Enemy,
            piercing: false,        // Enemies don't use piercing
            homing: false,          // Enemies don't use homing
            explosion_radius: 0.0,  // Enemies don't use explosions
            locked_target_index: None,
            lifetime: 0.0,
        });
    }
}
