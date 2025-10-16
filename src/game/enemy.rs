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
        // Update hit flash animation
        crate::game::animation::update_hit_flash(
            &mut enemy.anim.hit_flash_timer,
            delta,
            state.config.animations.hit_flash_duration,
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
                        path.p0,
                        path.p1,
                        path.p2,
                        path.p3,
                        *progress,
                    );
                } else {
                    enemy.pos = crate::game::bezier::quadratic_bezier(
                        path.p0,
                        path.p1,
                        path.p2,
                        *progress,
                    );
                }

                // Transition to free movement when path complete
                if *progress >= 1.0 {
                    enemy.movement_state = EnemyMovementState::FreeMovement;
                }
            }
            EnemyMovementState::FreeMovement => {
                // Normal descent movement (original behavior)
                if enemy.pos.y < enemy_cfg.movement_threshold_y {
                    enemy.pos.y += enemy_cfg.fast_descent_speed * delta;
                } else {
                    enemy.pos.y += enemy_cfg.slow_hover_speed * delta;
                }
            }
        }

        // Fire based on enemy type (only in free movement or near end of path)
        let can_fire = match &enemy.movement_state {
            EnemyMovementState::FollowingPath { progress, .. } => *progress > 0.7, // Can fire near end of path
            EnemyMovementState::FreeMovement => true,
        };

        if can_fire
            && idx < state.enemy_fire_timers.len()
            && state.enemy_fire_timers[idx] <= 0.0
            && enemy.pos.y > enemy_cfg.fire_threshold_y
        {
            fire_enemy_weapon(
                enemy,
                state.player.pos,
                state.player.velocity,
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
    player_velocity: Position,
    projectiles: &mut Vec<Projectile>,
    basic_projectile_speed_y: f32,
    weapons_config: &crate::config::WeaponsConfig,
) {
    // Pick random weapon from enemy's arsenal
    if enemy.weapon.is_empty() {
        return;
    }

    let random_idx = rand::gen_range(0, enemy.weapon.len());
    let weapon = enemy.weapon[random_idx];
    let weapon_stats = weapon.get_weapon_stats(weapons_config);

    match weapon {
        WeaponType::Bullet => {
            // Shoot straight down or aimed based on enemy type
            let velocity = match enemy.entity_type {
                EntityType::BasicFighter => Position {
                    x: 0.0,
                    y: basic_projectile_speed_y,
                },

                EntityType::Tank | EntityType::Sniper | EntityType::Elite => {
                    calculate_lead_velocity(
                        enemy.pos,
                        player_pos,
                        player_velocity,
                        basic_projectile_speed_y,
                    )
                }
                EntityType::Healer | EntityType::Splitter => {
                    calculate_velocity(enemy.pos, player_pos, basic_projectile_speed_y)
                }
            };

            projectiles.push(Projectile {
                pos: enemy.pos,
                velocity,
                damage: weapon_stats.damage * 0.75, // Enemies deal 75% damage
                weapon_type: weapon,
                owner: ProjectileOwner::Enemy,
                piercing: false,
                homing: false,
                explosion_radius: 0.0,
                locked_target_index: None,
                lifetime: 0.0,
                trail_timer: 0.0,
            });
        }
        WeaponType::Laser => {
            // Lasers always aim at player
            let velocity = calculate_lead_velocity(
                enemy.pos,
                player_pos,
                player_velocity,
                weapon_stats.projectile_speed,
            );

            projectiles.push(Projectile {
                pos: enemy.pos,
                velocity,
                damage: weapon_stats.damage * 0.75,
                weapon_type: weapon,
                owner: ProjectileOwner::Enemy,
                piercing: true, // Enemy lasers also pierce
                homing: false,
                explosion_radius: 0.0,
                locked_target_index: None,
                lifetime: 0.0,
                trail_timer: 0.0,
            });
        }
        WeaponType::Missile => {
            // Enemy missiles home in on the player
            let velocity = calculate_velocity(enemy.pos, player_pos, weapon_stats.projectile_speed);

            projectiles.push(Projectile {
                pos: enemy.pos,
                velocity,
                damage: weapon_stats.damage * 0.75,
                weapon_type: weapon,
                owner: ProjectileOwner::Enemy,
                piercing: false,
                homing: true, // Enemy missiles now track the player!
                explosion_radius: 0.0,
                locked_target_index: Some(0), // Lock onto player (index 0 for enemy projectiles)
                lifetime: 0.0,
                trail_timer: 0.0,
            });
        }
        WeaponType::Plasma => {
            // Fire 3 projectiles toward player
            let spread_angle = 15.0_f32.to_radians();
            let angles = [-spread_angle, 0.0, spread_angle];

            for &angle in &angles {
                // Calculate direction to player, then apply spread
                let base_dir_x = player_pos.x - enemy.pos.x;
                let base_dir_y = player_pos.y - enemy.pos.y;
                let base_distance = (base_dir_x * base_dir_x + base_dir_y * base_dir_y).sqrt();

                if base_distance > 0.1 {
                    // Normalize base direction
                    let norm_x = base_dir_x / base_distance;
                    let norm_y = base_dir_y / base_distance;

                    // Apply angle rotation to spread pattern
                    let rotated_x = norm_x * angle.cos() - norm_y * angle.sin();
                    let rotated_y = norm_x * angle.sin() + norm_y * angle.cos();

                    projectiles.push(Projectile {
                        pos: enemy.pos,
                        velocity: Position {
                            x: rotated_x * weapon_stats.projectile_speed,
                            y: rotated_y * weapon_stats.projectile_speed,
                        },
                        damage: weapon_stats.damage * 0.75,
                        weapon_type: weapon,
                        owner: ProjectileOwner::Enemy,
                        piercing: false,
                        homing: false,
                        explosion_radius: 0.0,
                        locked_target_index: None,
                        lifetime: 0.0,
                        trail_timer: 0.0,
                    });
                }
            }
        }
        WeaponType::Bombs => {
            // AOE threat aimed at player
            let velocity = calculate_velocity(enemy.pos, player_pos, weapon_stats.projectile_speed);

            projectiles.push(Projectile {
                pos: enemy.pos,
                velocity,
                damage: weapon_stats.damage * 0.75,
                weapon_type: weapon,
                owner: ProjectileOwner::Enemy,
                piercing: false,
                homing: false,
                explosion_radius: 60.0, // Enemy bomb AOE
                locked_target_index: None,
                lifetime: 0.0,
                trail_timer: 0.0,
            });
        }
    }
}
