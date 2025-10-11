use crate::game::utils::circle_collision;
use crate::models::*;

/// Check all projectile collisions and apply damage
pub fn check_projectile_collisions(state: &mut GameState) {
    let mut projectiles_to_remove = Vec::new();
    let collision_cfg = &state.config.collision;

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
}
