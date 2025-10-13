use crate::models::*;
use macroquad::prelude::*;

/// Update all weapon timers and projectiles
pub fn update_weapons(state: &mut GameState, delta: f32) {
    // Countdown fire cooldown
    state.player_fire_timer = (state.player_fire_timer - delta).max(0.0);

    // Update all projectiles
    update_projectiles(state, delta);
}

/// Fire player weapon if cooldown allows
pub fn player_fire_weapon(state: &mut GameState, weapon_index: usize) {
    if weapon_index >= state.player.weapon.len() {
        return;
    }

    let weapon = state.player.weapon[weapon_index];

    // Check cooldown
    if state.player_fire_timer > 0.0 {
        return;
    }

    let weapon_stats = weapon.get_weapon_stats(&state.config.weapons);
    state.player_fire_timer = weapon_stats.fire_rate;

    // Fire weapon based on type (different behaviors)
    match weapon {
        WeaponType::Bullet => fire_bullet(state, weapon_stats),
        WeaponType::Laser => fire_laser(state, weapon_stats),
        WeaponType::Missile => fire_missile(state, weapon_stats),
        WeaponType::Plasma => fire_plasma(state, weapon_stats),
        WeaponType::Bombs => fire_bombs(state, weapon_stats),
    }
}

/// Fire a standard bullet projectile
fn fire_bullet(state: &mut GameState, weapon_stats: crate::models::WeaponStats) {
    let projectile = Projectile {
        pos: state.player.pos,
        velocity: Position {
            x: 0.0,
            y: -weapon_stats.projectile_speed, // Shoot upward
        },
        damage: weapon_stats.damage,
        weapon_type: WeaponType::Bullet,
        owner: ProjectileOwner::Player,
        piercing: false,
        homing: false,
        explosion_radius: 0.0,
        locked_target_index: None,
        lifetime: 0.0,
    };

    state.projectiles.push(projectile);
}

/// Fire a piercing laser beam (doesn't despawn on hit)
fn fire_laser(state: &mut GameState, weapon_stats: crate::models::WeaponStats) {
    let projectile = Projectile {
        pos: state.player.pos,
        velocity: Position {
            x: 0.0,
            y: -weapon_stats.projectile_speed, // Shoot upward
        },
        damage: weapon_stats.damage,
        weapon_type: WeaponType::Laser,
        owner: ProjectileOwner::Player,
        piercing: true, // KEY: Doesn't despawn on hit
        homing: false,
        explosion_radius: 0.0,
        locked_target_index: None,
        lifetime: 0.0,
    };

    state.projectiles.push(projectile);
}

/// Fire a homing missile that tracks enemies
fn fire_missile(state: &mut GameState, weapon_stats: crate::models::WeaponStats) {
    // Find nearest enemy to lock onto immediately
    let nearest_idx = find_nearest_enemy_index(state.player.pos, &state.enemies);

    let projectile = Projectile {
        pos: state.player.pos,
        velocity: Position {
            x: 0.0,
            y: -weapon_stats.projectile_speed, // Initial upward velocity
        },
        damage: weapon_stats.damage,
        weapon_type: WeaponType::Missile,
        owner: ProjectileOwner::Player,
        piercing: false,
        homing: true, // KEY: Will track locked enemy
        explosion_radius: 0.0,
        locked_target_index: nearest_idx, // Lock onto target at spawn
        lifetime: 0.0,
    };

    state.projectiles.push(projectile);
}

/// Fire plasma spread shot (3 projectiles in cone pattern)
fn fire_plasma(state: &mut GameState, weapon_stats: crate::models::WeaponStats) {
    let spread_angle = 15.0_f32.to_radians(); // ±15 degrees
    let angles = [-spread_angle, 0.0, spread_angle]; // Left, center, right

    for &angle in &angles {
        let projectile = Projectile {
            pos: state.player.pos,
            velocity: Position {
                x: weapon_stats.projectile_speed * angle.sin(),
                y: -weapon_stats.projectile_speed * angle.cos(), // Spread pattern
            },
            damage: weapon_stats.damage,
            weapon_type: WeaponType::Plasma,
            owner: ProjectileOwner::Player,
            piercing: false,
            homing: false,
            explosion_radius: 0.0,
            locked_target_index: None,
            lifetime: 0.0,
        };

        state.projectiles.push(projectile);
    }
}

/// Fire bomb with AOE explosion on impact
fn fire_bombs(state: &mut GameState, weapon_stats: crate::models::WeaponStats) {
    let projectile = Projectile {
        pos: state.player.pos,
        velocity: Position {
            x: 0.0,
            y: -weapon_stats.projectile_speed, // Shoot upward
        },
        damage: weapon_stats.damage,
        weapon_type: WeaponType::Bombs,
        owner: ProjectileOwner::Player,
        piercing: false,
        homing: false,
        explosion_radius: 80.0, // AOE damage radius
        locked_target_index: None,
        lifetime: 0.0,
    };

    state.projectiles.push(projectile);
}

/// Update all projectile positions and remove off-screen ones
fn update_projectiles(state: &mut GameState, delta: f32) {
    // Update lifetimes first
    for projectile in &mut state.projectiles {
        projectile.lifetime += delta;
    }

    // Update homing missiles with proper target tracking
    for projectile in &mut state.projectiles {
        if projectile.homing {
            let target_pos = match projectile.owner {
                ProjectileOwner::Player | ProjectileOwner::Ghost => {
                    // Player/Ghost missiles track enemies
                    if let Some(target_idx) = projectile.locked_target_index {
                        // Validate target index
                        if target_idx < state.enemies.len() {
                            Some(state.enemies[target_idx].pos)
                        } else {
                            // Target was destroyed, find new one
                            projectile.locked_target_index =
                                find_nearest_enemy_index(projectile.pos, &state.enemies);
                            projectile
                                .locked_target_index
                                .map(|idx| state.enemies[idx].pos)
                        }
                    } else {
                        // No target locked, find one
                        projectile.locked_target_index =
                            find_nearest_enemy_index(projectile.pos, &state.enemies);
                        projectile
                            .locked_target_index
                            .map(|idx| state.enemies[idx].pos)
                    }
                }
                ProjectileOwner::Enemy => {
                    // Enemy missiles track the player
                    Some(state.player.pos)
                }
            };

            // If we have a target, steer towards it
            if let Some(target) = target_pos {
                homing_behavior(projectile, target, delta);
            }
        }

        // Normal movement for all projectiles
        projectile.pos.x += projectile.velocity.x * delta;
        projectile.pos.y += projectile.velocity.y * delta;
    }

    // Remove projectiles that are:
    // Off-screen
    // Exceeded max lifetime (5 seconds - prevents stuck missiles)
    let padding = state.config.projectile_bounds.off_screen_padding;
    let max_lifetime = 5.0;

    state.projectiles.retain(|p| {
        let in_bounds = p.pos.y > -padding
            && p.pos.y < screen_height() + padding
            && p.pos.x > -padding
            && p.pos.x < screen_width() + padding;

        let alive = p.lifetime < max_lifetime;

        in_bounds && alive
    });
}

/// Homing behavior: Smoothly steer projectile towards target
fn homing_behavior(projectile: &mut Projectile, target: Position, delta: f32) {
    // Calculate direction to target
    let dx = target.x - projectile.pos.x;
    let dy = target.y - projectile.pos.y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance < 1.0 {
        return; // Already on target
    }

    // Desired velocity (pointing at target)
    let desired_speed = 300.0; // Constant homing speed (CONFIG HERE)
    let desired_vel_x = (dx / distance) * desired_speed;
    let desired_vel_y = (dy / distance) * desired_speed;

    // Smoothly interpolate current velocity towards desired velocity
    let turn_speed = 8.0; // How fast missile can turn (higher = sharper turns)

    projectile.velocity.x += (desired_vel_x - projectile.velocity.x) * turn_speed * delta;
    projectile.velocity.y += (desired_vel_y - projectile.velocity.y) * turn_speed * delta;

    // Clamp speed to prevent over-acceleration
    let current_speed = (projectile.velocity.x * projectile.velocity.x
        + projectile.velocity.y * projectile.velocity.y)
        .sqrt();

    if current_speed > desired_speed * 1.5 {
        let scale = (desired_speed * 1.5) / current_speed;
        projectile.velocity.x *= scale;
        projectile.velocity.y *= scale;
    }
}

/// Find the nearest enemy to a given position and return its index
fn find_nearest_enemy_index(pos: Position, enemies: &[crate::models::Enemy]) -> Option<usize> {
    if enemies.is_empty() {
        return None;
    }

    enemies
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            let dist_a = (a.pos.x - pos.x).powi(2) + (a.pos.y - pos.y).powi(2);
            let dist_b = (b.pos.x - pos.x).powi(2) + (b.pos.y - pos.y).powi(2);
            dist_a.partial_cmp(&dist_b).unwrap()
        })
        .map(|(idx, _)| idx)
}

/// Find the nearest enemy to a given position (for backward compatibility)
fn find_nearest_enemy(pos: Position, enemies: &[crate::models::Enemy]) -> Option<Position> {
    find_nearest_enemy_index(pos, enemies).map(|idx| enemies[idx].pos)
}
