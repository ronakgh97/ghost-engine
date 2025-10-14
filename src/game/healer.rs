use crate::models::*;

/// Apply healing from healer enemies to other enemies in range
pub fn update_healer_healing(state: &mut GameState, delta: f32) {
    // Track healing info (healer_idx, enemy_idx, heal_amount)
    let mut healing_events: Vec<(usize, usize, f32)> = Vec::new();

    // Check each healer enemy
    for (healer_idx, healer) in state.enemies.iter().enumerate() {
        if healer.entity_type != EntityType::Healer {
            continue;
        }

        let heal_rate = state.config.entities.healer.heal_rate;
        let heal_radius = state.config.entities.healer.heal_radius;
        let heal_amount = heal_rate * delta;

        // Find enemies in range (excluding self)
        for (enemy_idx, enemy) in state.enemies.iter().enumerate() {
            if healer_idx == enemy_idx {
                continue; // Don't heal self
            }

            let dx = enemy.pos.x - healer.pos.x;
            let dy = enemy.pos.y - healer.pos.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= heal_radius {
                healing_events.push((healer_idx, enemy_idx, heal_amount));
            }
        }
    }

    // Apply healing (don't exceed max health)
    for (_healer_idx, enemy_idx, heal_amount) in healing_events {
        if let Some(enemy) = state.enemies.get_mut(enemy_idx) {
            enemy.stats.health = (enemy.stats.health + heal_amount).min(enemy.stats.max_health);
        }
    }
}

/// Apply healing from healer ghosts to player
pub fn update_ghost_healer_healing(state: &mut GameState, delta: f32) {
    let heal_rate = state.config.entities.healer.heal_rate;
    let heal_radius = state.config.entities.healer.heal_radius;
    let heal_amount = heal_rate * delta;

    // Check each ghost
    for ghost in &state.ghosts {
        if ghost.entity_type != EntityType::Healer {
            continue;
        }

        // Check if player is in range
        let dx = state.player.pos.x - ghost.pos.x;
        let dy = state.player.pos.y - ghost.pos.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance <= heal_radius {
            // Heal player (don't exceed max health)
            state.player.stats.health =
                (state.player.stats.health + heal_amount).min(state.player.stats.max_health);
        }
    }
}
