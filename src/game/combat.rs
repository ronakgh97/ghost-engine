use crate::game::particles::spawn_death_explosion;
use crate::game::screen_shake::shake_on_enemy_death;
use crate::game::utils::circle_collision;
use crate::models::*;

/// Check entity-to-entity collisions
#[allow(dead_code)] //TODO: may be useful later
pub fn check_entity_collisions(_state: &mut GameState) {
    // TODO: Implement player-enemy collision damage later
    // When implemented, add hit flash + particles:
    // - state.player.hit_flash_timer = state.config.animations.hit_flash_duration;
    // - spawn_player_hit_effect(state, player.pos);

    // let mut player_hit_position: Option<Position> = None; // Track for hit effects
    //
    // // Player collision with enemies (gradual damage)
    // for enemy in &state.enemies {
    //     if circle_collision(state.player.pos, enemy.pos, 15.0, 15.0) {
    //         state.player.stats.health -= enemy.stats.damage * 0.016;
    //         state.player.hit_flash_timer = state.config.animations.hit_flash_duration; // Flash on contact!
    //         player_hit_position = Some(state.player.pos); // Track for particles (only once per frame)
    //     }
    // }
    //
    // // Spawn player hit particles if contact damage occurred
    // if let Some(hit_pos) = player_hit_position {
    //     crate::game::particles::spawn_player_hit_effect(state, hit_pos);
    // }
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
    // Only collect NEWLY dead ghosts (not already despawning) to prevent infinite splitting!
    let dead_ghost_splitters: Vec<Ghost> = state
        .ghosts
        .iter()
        .filter(|g| {
            g.stats.health <= 0.0 && g.entity_type == EntityType::Splitter && !g.anim.is_despawning
        })
        .cloned()
        .collect();

    // Remove dead ghosts
    // Trigger despawn animation instead of instant removal
    for ghost in state.ghosts.iter_mut() {
        if ghost.stats.health <= 0.0 && !ghost.anim.is_despawning {
            ghost
                .anim
                .start_despawn(state.config.animations.ghost_despawn_duration);
        }
    }

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
