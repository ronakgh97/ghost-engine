use crate::game::screen_shake::{shake_on_player_hit, shake_on_weapon_hit};
use crate::game::utils::circle_collision;
use crate::models::*;

/// Check all projectile collisions and apply damage
pub fn check_projectile_collisions(state: &mut GameState) {
    let mut projectiles_to_remove = Vec::new();
    let collision_cfg = &state.config.collision;
    let mut player_was_hit = false; // Track if player took damage
    let mut weapon_hits: Vec<WeaponType> = Vec::new(); // Track weapon types that hit

    for (proj_idx, projectile) in state.projectiles.iter().enumerate() {
        match projectile.owner {
            ProjectileOwner::Player | ProjectileOwner::Ghost => {
                // Check if this is a bomb (AOE explosion)
                if projectile.explosion_radius > 0.0 {
                    let mut hit_any_enemy = false;

                    // Check all enemies within explosion radius
                    for enemy in &mut state.enemies {
                        let distance = ((projectile.pos.x - enemy.pos.x).powi(2)
                            + (projectile.pos.y - enemy.pos.y).powi(2))
                        .sqrt();

                        if distance <= projectile.explosion_radius {
                            enemy.stats.health -= projectile.damage;
                            hit_any_enemy = true;
                        }
                    }

                    // Bomb explodes if it hits ANY enemy (AOE damage applied to all in radius)
                    if hit_any_enemy {
                        weapon_hits.push(projectile.weapon_type); // Track bomb hit for shake
                        projectiles_to_remove.push(proj_idx);
                    }
                } else {
                    // Standard projectile collision (Bullet, Laser, Missile, Plasma)
                    for enemy in &mut state.enemies {
                        if circle_collision(
                            projectile.pos,
                            enemy.pos,
                            collision_cfg.projectile_radius,
                            collision_cfg.enemy_radius,
                        ) {
                            enemy.stats.health -= projectile.damage;
                            weapon_hits.push(projectile.weapon_type); // Track weapon hit for shake

                            // Only mark for removal if NOT piercing (lasers pierce through)
                            if !projectile.piercing {
                                projectiles_to_remove.push(proj_idx);
                                break;
                            }
                            // Piercing projectiles continue after hit (no break)
                        }
                    }
                }
            }

            ProjectileOwner::Enemy => {
                // Enemy bombs can also have AOE (if configured)
                if projectile.explosion_radius > 0.0 {
                    let mut hit_player_or_ghost = false;

                    // Check player
                    let distance_to_player = ((projectile.pos.x - state.player.pos.x).powi(2)
                        + (projectile.pos.y - state.player.pos.y).powi(2))
                    .sqrt();

                    if distance_to_player <= projectile.explosion_radius {
                        state.player.stats.health -= projectile.damage;
                        player_was_hit = true; // Mark for shake trigger
                        hit_player_or_ghost = true;
                    }

                    // Check ghosts
                    for ghost in &mut state.ghosts {
                        let distance = ((projectile.pos.x - ghost.pos.x).powi(2)
                            + (projectile.pos.y - ghost.pos.y).powi(2))
                        .sqrt();

                        if distance <= projectile.explosion_radius {
                            ghost.stats.health -= projectile.damage;
                            hit_player_or_ghost = true;
                        }
                    }

                    if hit_player_or_ghost {
                        projectiles_to_remove.push(proj_idx);
                    }
                } else {
                    // Standard enemy projectile collision
                    // Check collision with player
                    if circle_collision(
                        projectile.pos,
                        state.player.pos,
                        collision_cfg.projectile_radius,
                        collision_cfg.player_radius,
                    ) {
                        state.player.stats.health -= projectile.damage;
                        player_was_hit = true; // Mark for shake trigger
                        // Enemy projectiles never pierce
                        projectiles_to_remove.push(proj_idx);
                    }

                    // Check collision with ghosts
                    for ghost in &mut state.ghosts {
                        if circle_collision(
                            projectile.pos,
                            ghost.pos,
                            collision_cfg.projectile_radius,
                            collision_cfg.ghost_radius,
                        ) {
                            ghost.stats.health -= projectile.damage;
                            projectiles_to_remove.push(proj_idx);
                            break;
                        }
                    }
                }
            }
        }
    }

    // Remove hit projectiles (reverse order to avoid index issues)
    projectiles_to_remove.sort_unstable();
    projectiles_to_remove.dedup(); // Remove duplicates (in case same projectile marked multiple times)
    for &idx in projectiles_to_remove.iter().rev() {
        if idx < state.projectiles.len() {
            state.projectiles.remove(idx);
        }
    }

    // Trigger screen shake if player was hit
    if player_was_hit {
        shake_on_player_hit(state);
    }
    
    // Trigger weapon-specific shake for each hit (strongest weapon wins if multiple)
    if let Some(&strongest_weapon) = weapon_hits.iter().max_by_key(|w| {
        // Priority order: Bombs > Missile > Laser > Plasma > Bullet
        match w {
            WeaponType::Bombs => 5,
            WeaponType::Missile => 4,
            WeaponType::Laser => 3,
            WeaponType::Plasma => 2,
            WeaponType::Bullet => 1,
        }
    }) {
        shake_on_weapon_hit(state, strongest_weapon);
    }
}
