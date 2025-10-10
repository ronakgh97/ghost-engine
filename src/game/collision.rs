use crate::game::utils::circle_collision;
use crate::models::*;

/// Check all projectile collisions and apply damage
pub fn check_projectile_collisions(state: &mut GameState) {
    let mut projectiles_to_remove = Vec::new();

    for (proj_idx, projectile) in state.projectiles.iter().enumerate() {
        match projectile.owner {
            ProjectileOwner::Player | ProjectileOwner::Ghost => {
                // Check collision with enemies
                for enemy in &mut state.enemies {
                    if circle_collision(projectile.pos, enemy.pos, 5.0, 15.0) {
                        enemy.stats.health -= projectile.damage;
                        projectiles_to_remove.push(proj_idx);
                        break;
                    }
                }
            }

            ProjectileOwner::Enemy => {
                // Check collision with player
                if circle_collision(projectile.pos, state.player.pos, 5.0, 15.0) {
                    state.player.stats.health -= projectile.damage;
                    projectiles_to_remove.push(proj_idx);
                }

                // Check collision with ghosts
                for ghost in &mut state.ghosts {
                    if circle_collision(projectile.pos, ghost.pos, 5.0, 12.0) {
                        ghost.stats.health -= projectile.damage;
                        projectiles_to_remove.push(proj_idx);
                        break;
                    }
                }
            }
        }
    }

    // Remove hit projectiles (reverse order to avoid index issues)
    for &idx in projectiles_to_remove.iter().rev() {
        if idx < state.projectiles.len() {
            state.projectiles.remove(idx);
        }
    }
}
