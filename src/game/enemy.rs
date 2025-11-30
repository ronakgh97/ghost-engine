use crate::game::weapons::{FireWeaponParams, FiringDirection, fire_weapon};
use crate::models::*;
use macroquad::prelude::*;

/// Update all enemies (movement, firing, cleanup)
pub fn update_enemies(state: &mut GameState, delta: f32) {
    // Extract config values upfront to avoid borrow issues
    let movement_threshold_y = state.config.enemy_behavior.movement_threshold_y;
    let fast_descent_speed = state.config.enemy_behavior.fast_descent_speed;
    let slow_hover_speed = state.config.enemy_behavior.slow_hover_speed;
    let fire_threshold_y = state.config.enemy_behavior.fire_threshold_y;
    let screen_boundary_bottom = state.config.enemy_behavior.screen_boundary_bottom;
    let hit_flash_duration = state.config.animations.hit_flash_duration;

    // Collect firing events first to avoid borrow issues
    // Store all data needed to fire (no references to state)
    let mut fire_events: Vec<(Vec2, WeaponType, FiringDirection)> = Vec::new();

    // Capture player data for firing direction
    let player_pos = state.player.pos;
    let player_vel = state.player.velocity;

    // Update each enemy (fire timers now embedded in Enemy struct!)
    for enemy in state.enemies.iter_mut() {
        // Update fire timer
        enemy.fire_timer = (enemy.fire_timer - delta).max(0.0);

        // Update hit flash animation
        crate::game::animation::update_hit_flash(
            &mut enemy.anim.hit_flash_timer,
            delta,
            hit_flash_duration,
        );

        // Movement logic - Check if following Bezier path or free movement
        match &mut enemy.movement_state {
            EnemyMovementState::FollowingPath {
                path,
                progress,
                elapsed_time,
            } => {
                // Update path progress
                *elapsed_time += delta;
                *progress = (*elapsed_time / path.duration).min(1.0);

                // Interpolate position along Bezier curve
                if path.use_cubic {
                    enemy.pos = crate::game::bezier::cubic_bezier(
                        path.p0, path.p1, path.p2, path.p3, *progress,
                    );
                } else {
                    enemy.pos =
                        crate::game::bezier::quadratic_bezier(path.p0, path.p1, path.p2, *progress);
                }

                // Transition to free movement when path complete
                if *progress >= 1.0 {
                    enemy.movement_state = EnemyMovementState::FreeMovement;
                }
            }
            EnemyMovementState::FreeMovement => {
                // Normal descent movement (original behavior)
                if enemy.pos.y < movement_threshold_y {
                    enemy.pos.y += fast_descent_speed * delta;
                } else {
                    enemy.pos.y += slow_hover_speed * delta;
                }
            }
        }

        // Fire based on enemy type (only in free movement or near end of path)
        let can_fire = match &enemy.movement_state {
            EnemyMovementState::FollowingPath { progress, .. } => *progress > 0.7,
            EnemyMovementState::FreeMovement => true,
        };

        if can_fire && enemy.fire_timer <= 0.0 && enemy.pos.y > fire_threshold_y {
            // Pick random weapon from enemy's arsenal
            if !enemy.weapon.is_empty() {
                let random_idx = rand::gen_range(0, enemy.weapon.len());
                let weapon = enemy.weapon[random_idx];

                // Determine firing direction based on enemy type
                let direction = match enemy.entity_type {
                    EntityType::BasicFighter => {
                        FiringDirection::Down // Shoots straight down
                    }
                    EntityType::Tank | EntityType::Sniper | EntityType::Elite => {
                        FiringDirection::LeadTarget {
                            target_pos: player_pos,
                            target_vel: player_vel,
                        }
                    }
                    EntityType::Healer | EntityType::Splitter => {
                        FiringDirection::AtTarget(player_pos)
                    }
                };

                fire_events.push((enemy.pos, weapon, direction));

                // Reset fire timer immediately
                enemy.fire_timer = enemy.entity_type.get_fire_interval(&state.config.entities);
            }
        }
    }

    // Execute firing using unified system (no borrows active now)
    for (shooter_pos, weapon, direction) in fire_events {
        fire_weapon(
            FireWeaponParams {
                shooter_pos,
                owner: ProjectileOwner::Enemy,
                weapon,
                direction,
                damage_multiplier: 0.75, // Enemies deal 75% damage
                enemies: None,           // Enemies don't need enemy positions for targeting
            },
            state,
        );
    }

    // Remove off-screen enemies
    state.enemies.retain(|e| e.pos.y < screen_boundary_bottom);
}
