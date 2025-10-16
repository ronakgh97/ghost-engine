use crate::game::particles::spawn_weapon_particles;
use crate::game::screen_shake::{shake_on_player_hit, shake_on_weapon_hit};
use crate::game::utils::circle_collision;
use crate::models::*;

/// Check all projectile collisions and apply damage
pub fn check_projectile_collisions(state: &mut GameState) {
    let mut projectiles_to_remove = Vec::new();
    let collision_cfg = &state.config.collision;
    let mut player_was_hit = false; // Track if player took damage
    let mut player_hit_position: Option<Position> = None; // Track hit position for particles
    let mut weapon_hits: Vec<(WeaponType, Position)> = Vec::new(); // Track weapon hits with positions

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
                            enemy.anim.hit_flash_timer = state.config.animations.hit_flash_duration; // Flash on hit!
                            hit_any_enemy = true;
                        }
                    }

                    // Bomb explodes if it hits ANY enemy (AOE damage applied to all in radius)
                    if hit_any_enemy {
                        weapon_hits.push((projectile.weapon_type, projectile.pos)); // Track bomb hit
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
                            enemy.anim.hit_flash_timer = state.config.animations.hit_flash_duration; // Flash on hit!
                            weapon_hits.push((projectile.weapon_type, projectile.pos)); // Track weapon hit

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

                    // Check player (skip if i-frames active!)
                    let distance_to_player = ((projectile.pos.x - state.player.pos.x).powi(2)
                        + (projectile.pos.y - state.player.pos.y).powi(2))
                    .sqrt();

                    if state.player.i_frame_timer <= 0.0
                        && distance_to_player <= projectile.explosion_radius
                    {
                        state.player.stats.health -= projectile.damage;
                        state.player.hit_flash_timer = state.config.animations.hit_flash_duration; // Flash on AOE hit!
                        player_hit_position = Some(state.player.pos); // Track for particles
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
                            ghost.anim.hit_flash_timer = state.config.animations.hit_flash_duration; // Flash on hit!
                            hit_player_or_ghost = true;
                        }
                    }

                    if hit_player_or_ghost {
                        projectiles_to_remove.push(proj_idx);
                    }
                } else {
                    // Standard enemy projectile collision
                    // Check collision with player (skip if i-frames active!)
                    if state.player.i_frame_timer <= 0.0
                        && circle_collision(
                            projectile.pos,
                            state.player.pos,
                            collision_cfg.projectile_radius,
                            collision_cfg.player_radius,
                        )
                    {
                        state.player.stats.health -= projectile.damage;
                        state.player.hit_flash_timer = state.config.animations.hit_flash_duration; // Flash on hit!
                        player_hit_position = Some(state.player.pos); // Save position for particle spawn
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
                            ghost.anim.hit_flash_timer = state.config.animations.hit_flash_duration; // Flash on hit!
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

    // Spawn player hit particles if player was damaged
    if let Some(hit_pos) = player_hit_position {
        crate::game::particles::spawn_player_hit_effect(state, hit_pos);
    }

    // Trigger weapon-specific shake and particles for each hit
    if let Some(&(strongest_weapon, hit_pos)) = weapon_hits.iter().max_by_key(|(w, _)| {
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
        spawn_weapon_particles(state, hit_pos, strongest_weapon);
    }
}
