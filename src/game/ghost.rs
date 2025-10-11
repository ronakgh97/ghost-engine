use crate::game::utils::*;
use crate::models::*;

/// Update all ghosts (movement, auto-fire, cleanup)
pub fn update_ghosts(state: &mut GameState, delta: f32) {
    let ghost_cfg = &state.config.ghost_behavior;

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
        if ghost.pos.y > ghost_cfg.movement_threshold_y {
            ghost.pos.y -= ghost_cfg.fast_ascent_speed * delta;
        } else {
            ghost.pos.y -= ghost_cfg.slow_hover_speed * delta;
        }

        // Auto-fire at nearest enemy
        if idx < state.ghost_fire_timers.len()
            && state.ghost_fire_timers[idx] <= 0.0
            && !state.enemies.is_empty()
        {
            if let Some(target) = find_nearest_enemy(ghost.pos, &state.enemies) {
                fire_ghost_weapon(
                    ghost,
                    target,
                    &mut state.projectiles,
                    ghost_cfg.projectile_speed,
                    &state.config.weapons,
                );
                state.ghost_fire_timers[idx] = ghost_cfg.fire_interval;
            }
        }
    }

    // Remove off-screen ghosts
    state
        .ghosts
        .retain(|g| g.pos.y > ghost_cfg.screen_boundary_top);

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
fn fire_ghost_weapon(
    ghost: &Ghost,
    target: Position,
    projectiles: &mut Vec<Projectile>,
    projectile_speed: f32,
    weapons_config: &crate::config::WeaponsConfig,
) {
    if let Some(&weapon) = ghost.weapon_type.first() {
        let weapon_stats = weapon.get_weapon_stats(weapons_config);

        match weapon {
            WeaponType::Bullet => {
                let velocity = calculate_velocity(ghost.pos, target, projectile_speed);

                projectiles.push(Projectile {
                    pos: ghost.pos,
                    velocity,
                    damage: weapon_stats.damage,
                    weapon_type: weapon,
                    owner: ProjectileOwner::Ghost,
                    piercing: false,
                    homing: false,
                    explosion_radius: 0.0,
                    locked_target_index: None,
                    lifetime: 0.0,
                });
            }
            WeaponType::Laser => {
                let velocity = calculate_velocity(ghost.pos, target, weapon_stats.projectile_speed);

                projectiles.push(Projectile {
                    pos: ghost.pos,
                    velocity,
                    damage: weapon_stats.damage,
                    weapon_type: weapon,
                    owner: ProjectileOwner::Ghost,
                    piercing: true, // Ghost lasers also pierce
                    homing: false,
                    explosion_radius: 0.0,
                    locked_target_index: None,
                    lifetime: 0.0,
                });
            }
            WeaponType::Missile => {
                // Ghosts don't use homing (too powerful with formations)
                let velocity = calculate_velocity(ghost.pos, target, weapon_stats.projectile_speed);

                projectiles.push(Projectile {
                    pos: ghost.pos,
                    velocity,
                    damage: weapon_stats.damage,
                    weapon_type: weapon,
                    owner: ProjectileOwner::Ghost,
                    piercing: false,
                    homing: false, // Aimed shot, not homing
                    explosion_radius: 0.0,
                    locked_target_index: None,
                    lifetime: 0.0,
                });
            }
            WeaponType::Plasma => {
                // Ghost plasma: 3-projectile spread toward target
                let spread_angle = 15.0_f32.to_radians();
                let angles = [-spread_angle, 0.0, spread_angle];

                for &angle in &angles {
                    let base_dir_x = target.x - ghost.pos.x;
                    let base_dir_y = target.y - ghost.pos.y;
                    let base_distance = (base_dir_x * base_dir_x + base_dir_y * base_dir_y).sqrt();

                    if base_distance > 0.1 {
                        let norm_x = base_dir_x / base_distance;
                        let norm_y = base_dir_y / base_distance;

                        let rotated_x = norm_x * angle.cos() - norm_y * angle.sin();
                        let rotated_y = norm_x * angle.sin() + norm_y * angle.cos();

                        projectiles.push(Projectile {
                            pos: ghost.pos,
                            velocity: Position {
                                x: rotated_x * weapon_stats.projectile_speed,
                                y: rotated_y * weapon_stats.projectile_speed,
                            },
                            damage: weapon_stats.damage,
                            weapon_type: weapon,
                            owner: ProjectileOwner::Ghost,
                            piercing: false,
                            homing: false,
                            explosion_radius: 0.0,
                            locked_target_index: None,
                            lifetime: 0.0,
                        });
                    }
                }
            }
            WeaponType::Bombs => {
                let velocity = calculate_velocity(ghost.pos, target, weapon_stats.projectile_speed);

                projectiles.push(Projectile {
                    pos: ghost.pos,
                    velocity,
                    damage: weapon_stats.damage,
                    weapon_type: weapon,
                    owner: ProjectileOwner::Ghost,
                    piercing: false,
                    homing: false,
                    explosion_radius: 70.0, // Ghost bomb AOE
                    locked_target_index: None,
                    lifetime: 0.0,
                });
            }
        }
    }
}
