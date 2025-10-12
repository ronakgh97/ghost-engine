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
    let total_ghosts = state.ghosts.len();
    for (idx, ghost) in state.ghosts.iter_mut().enumerate() {
        // Formation following - calculate target position based on current formation
        let target_pos = calculate_formation_position(
            state.player.pos,
            idx,
            total_ghosts,
            state.ghost_formation,
            &state.config.formation_spacing,
        );

        // Calculate distance to target
        let dx = target_pos.x - ghost.pos.x;
        let dy = target_pos.y - ghost.pos.y;
        let distance = (dx * dx + dy * dy).sqrt();

        // Only move if far enough from target (prevents jitter/oscillation)
        if distance > 2.0 {
            // Smoothly interpolate to formation position (feels more natural than instant)
            let follow_speed = 1.0; // Reduced from 5.0 for smoother movement
            ghost.pos.x += dx * follow_speed * delta;
            ghost.pos.y += dy * follow_speed * delta;
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
                    &state.enemies, // Pass enemies for missile targeting
                    ghost_cfg.projectile_speed,
                    &state.config.weapons,
                );
                state.ghost_fire_timers[idx] = ghost_cfg.fire_interval;
            }
        }
    }

    // Remove off-screen ghosts (safety cleanup, shouldn't happen with formation following)
    state
        .ghosts
        .retain(|g| g.pos.y > ghost_cfg.screen_boundary_top && g.pos.y < 1000.0);

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

/// Find the index of the nearest enemy for missile locking
fn find_nearest_enemy_index(ghost_pos: Position, enemies: &[Enemy]) -> Option<usize> {
    enemies
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            let dist_a = distance_sq(ghost_pos, a.pos);
            let dist_b = distance_sq(ghost_pos, b.pos);
            dist_a.partial_cmp(&dist_b).unwrap()
        })
        .map(|(idx, _)| idx)
}

/// Fire ghost weapon at target
fn fire_ghost_weapon(
    ghost: &Ghost,
    target: Position,
    projectiles: &mut Vec<Projectile>,
    enemies: &[Enemy], // ✅ NEW: Need enemies for missile targeting
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
                // Ghost missiles ARE homing (exact copy of enemy behavior!)
                // Find nearest enemy to lock onto
                let nearest_idx = find_nearest_enemy_index(ghost.pos, enemies);
                let velocity = calculate_velocity(ghost.pos, target, weapon_stats.projectile_speed);

                projectiles.push(Projectile {
                    pos: ghost.pos,
                    velocity,
                    damage: weapon_stats.damage,
                    weapon_type: weapon,
                    owner: ProjectileOwner::Ghost,
                    piercing: false,
                    homing: true, // ✅ Ghosts use homing missiles (just like enemies)
                    explosion_radius: 0.0,
                    locked_target_index: nearest_idx, // Lock onto nearest enemy
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
