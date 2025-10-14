use crate::game::particles::spawn_death_explosion;
use crate::game::screen_shake::shake_on_enemy_death;
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
    // Collect dead enemies for splitting logic
    let dead_splitters: Vec<Enemy> = state
        .enemies
        .iter()
        .filter(|e| e.stats.health <= 0.0 && e.entity_type == EntityType::Splitter)
        .cloned()
        .collect();

    // Remove dead enemies and convert to ghosts
    let mut i = 0;
    while i < state.enemies.len() {
        if state.enemies[i].stats.health <= 0.0 {
            let enemy_type = state.enemies[i].entity_type;
            let enemy_pos = state.enemies[i].pos;
            state.player.available_ghosts.push(enemy_type);
            state.enemies.remove(i);
            shake_on_enemy_death(state);
            spawn_death_explosion(state, enemy_pos); // Particle explosion!
        } else {
            i += 1;
        }
    }

    // Handle splitter enemies - spawn splits
    let new_splits = crate::game::splitter::handle_enemy_splits(&dead_splitters, &state.config);
    for split in new_splits {
        state.enemies.push(split);
    }

    // Spawn split animation particles for each dead splitter
    for splitter in &dead_splitters {
        let split_count = state.config.entities.splitter.split_count;
        crate::game::splitter::spawn_split_particles(state, splitter.pos, split_count, false);
    }

    // Collect dead ghost splitters before removing
    let dead_ghost_splitters: Vec<Ghost> = state
        .ghosts
        .iter()
        .filter(|g| g.stats.health <= 0.0 && g.entity_type == EntityType::Splitter)
        .cloned()
        .collect();

    // Remove dead ghosts
    state.ghosts.retain(|g| g.stats.health > 0.0);

    // Handle ghost splitter splitting - spawn new ghost splits
    let new_ghost_splits =
        crate::game::splitter::handle_ghost_splits(&dead_ghost_splitters, &state.config);
    for split in new_ghost_splits {
        state.ghosts.push(split);
    }

    // Spawn split animation particles for each dead ghost splitter
    for ghost_splitter in &dead_ghost_splitters {
        let split_count = state.config.entities.splitter.split_count;
        crate::game::splitter::spawn_split_particles(state, ghost_splitter.pos, split_count, true);
    }
}
