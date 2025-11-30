use crate::game::utils::*;
use crate::models::*;

/// Update all ghosts (movement, auto-fire, cleanup)
pub fn update_ghosts(state: &mut GameState, delta: f32) {
    let ghost_cfg = &state.config.ghost_behavior;

    // Update each ghost (fire timers now embedded in Ghost struct!)
    let total_ghosts = state.ghosts.len();
    for ghost in state.ghosts.iter_mut() {
        // Update fire timer
        ghost.fire_timer = (ghost.fire_timer - delta).max(0.0);
    }

    // Second pass for firing and movement (need index for formation position)
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
        .retain(|g| g.pos.y > ghost_cfg.screen_boundary_top && g.pos.y < 1000.0);
}

/// Auto-fire ghosts at enemies (called separately to avoid borrow issues)
pub fn update_ghost_firing(state: &mut GameState) {
    let ghost_cfg = &state.config.ghost_behavior;

    // Collect firing data first to avoid borrow issues
    let mut fire_events: Vec<(usize, Position)> = Vec::new();

    for (idx, ghost) in state.ghosts.iter().enumerate() {
        if ghost.fire_timer <= 0.0 && !state.enemies.is_empty() {
            if let Some(target) = find_nearest_enemy(ghost.pos, &state.enemies) {
                fire_events.push((idx, target));
            }
        }
    }

    // Execute firing
    for (idx, target) in fire_events {
        let ghost = &state.ghosts[idx];
        fire_ghost_weapon(
            ghost,
            target,
            &mut state.projectiles,
            &state.enemies,
            ghost_cfg.projectile_speed,
            &state.config.weapons,
        );
        state.ghosts[idx].fire_timer = ghost_cfg.fire_interval;
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

/// Find the index of the nearest enemy for missile locking
fn find_nearest_enemy_index(ghost_pos: Position, enemies: &[Enemy]) -> Option<usize> {
    enemies
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            let dist_a = (a.pos - ghost_pos).length_squared();
            let dist_b = (b.pos - ghost_pos).length_squared();
            dist_a.partial_cmp(&dist_b).unwrap()
        })
        .map(|(idx, _)| idx)
}

/// Fire ghost weapon at target
fn fire_ghost_weapon(
    ghost: &Ghost,
    target: Position,
    projectiles: &mut Vec<Projectile>,
    enemies: &[Enemy], // Need enemies for missile targeting
    projectile_speed: f32,
    weapons_config: &crate::config::WeaponsConfig,
) {
    use macroquad::math::Vec2;

    // Pick random weapon from arsenal
    if ghost.weapon_type.is_empty() {
        return; // No weapons available
    }

    use macroquad::rand;
    let random_idx = rand::gen_range(0, ghost.weapon_type.len());
    let weapon = ghost.weapon_type[random_idx];

    let weapon_stats = weapon.get_weapon_stats(weapons_config);

    match weapon {
        WeaponType::Bullet => {
            let velocity = calculate_velocity(ghost.pos, target, projectile_speed);

            projectiles.push(Projectile {
                pos: ghost.pos,
                velocity,
                damage: weapon_stats.damage * 0.5, // Ghost bullets do 50% damage
                weapon_type: weapon,
                owner: ProjectileOwner::Ghost,
                piercing: false,
                homing: false,
                explosion_radius: 0.0,
                locked_target_index: None,
                lifetime: 0.0,
                trail_timer: 0.0,
            });
        }
        WeaponType::Laser => {
            let velocity = calculate_velocity(ghost.pos, target, weapon_stats.projectile_speed);

            projectiles.push(Projectile {
                pos: ghost.pos,
                velocity,
                damage: weapon_stats.damage * 0.5, // Ghost lasers do 50% damage
                weapon_type: weapon,
                owner: ProjectileOwner::Ghost,
                piercing: true, // Ghost lasers also pierce
                homing: false,
                explosion_radius: 0.0,
                locked_target_index: None,
                lifetime: 0.0,
                trail_timer: 0.0,
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
                damage: weapon_stats.damage * 0.5, // Ghost missiles do 50% damage
                weapon_type: weapon,
                owner: ProjectileOwner::Ghost,
                piercing: false,
                homing: true,
                explosion_radius: 0.0,
                locked_target_index: nearest_idx, // Lock onto nearest enemy
                lifetime: 0.0,
                trail_timer: 0.0,
            });
        }
        WeaponType::Plasma => {
            // Ghost plasma: 3-projectile spread toward target
            let spread_angle = 15.0_f32.to_radians();
            let angles = [-spread_angle, 0.0, spread_angle];

            for &angle in &angles {
                let base_dir = target - ghost.pos;
                let base_distance = base_dir.length();

                if base_distance > 0.1 {
                    let norm = base_dir / base_distance;

                    let rotated_x = norm.x * angle.cos() - norm.y * angle.sin();
                    let rotated_y = norm.x * angle.sin() + norm.y * angle.cos();

                    projectiles.push(Projectile {
                        pos: ghost.pos,
                        velocity: Vec2::new(
                            rotated_x * weapon_stats.projectile_speed,
                            rotated_y * weapon_stats.projectile_speed,
                        ),
                        damage: weapon_stats.damage * 0.5, // Ghost plasma does 50% damage
                        weapon_type: weapon,
                        owner: ProjectileOwner::Ghost,
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
            let velocity = calculate_velocity(ghost.pos, target, weapon_stats.projectile_speed);

            projectiles.push(Projectile {
                pos: ghost.pos,
                velocity,
                damage: weapon_stats.damage * 0.5, // Ghost bombs do 50% damage
                weapon_type: weapon,
                owner: ProjectileOwner::Ghost,
                piercing: false,
                homing: false,
                explosion_radius: 70.0, // Ghost bomb AOE
                locked_target_index: None,
                lifetime: 0.0,
                trail_timer: 0.0,
            });
        }
    }
}
