use crate::game::utils::circle_collision;
use crate::models::*;

/// Check entity-to-entity collisions
pub fn check_entity_collisions(state: &mut GameState) {
    // Player collision with enemies (gradual damage)
    for enemy in &state.enemies {
        if circle_collision(state.player.pos, enemy.pos, 15.0, 15.0) {
            state.player.stats.health -= enemy.stats.damage * 0.016;
        }
    }
}

/// Clean up dead entities and add enemies to ghost queue
pub fn cleanup_dead_entities(state: &mut GameState) {
    // Remove dead enemies and convert to ghosts
    let mut i = 0;
    while i < state.enemies.len() {
        if state.enemies[i].stats.health <= 0.0 {
            let enemy_type = state.enemies[i].entity_type;
            state.player.available_ghosts.push(enemy_type);
            state.enemies.remove(i);
            // TODO: Spawn death particle effect
        } else {
            i += 1;
        }
    }

    // Remove dead ghosts
    state.ghosts.retain(|g| g.stats.health > 0.0);
}
