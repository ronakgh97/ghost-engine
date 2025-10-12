use crate::models::*;
use macroquad::prelude::*;

/// Spawn enemies at regular intervals
pub fn spawn_enemies(state: &mut GameState, delta: f32) {
    state.spawn_timer += delta;

    // Spawn enemy every 2 seconds
    if state.spawn_timer >= state.config.spawning.enemy_spawn_interval {
        state.spawn_timer = 0.0;

        // Pick random enemy type
        let enemy_types = [
            EntityType::BasicFighter,
            EntityType::BasicFighter,
            EntityType::Sniper,
            EntityType::Tank,
        ];

        let random_idx = rand::gen_range(0, enemy_types.len());
        let entity_type = enemy_types[random_idx];

        // Get entity stats from config
        let entity_stats = entity_type.get_stats(&state.config.entities);
        let entity_config = match entity_type {
            EntityType::BasicFighter => &state.config.entities.basic_fighter,
            EntityType::Sniper => &state.config.entities.sniper,
            EntityType::Tank => &state.config.entities.tank,
            EntityType::Boss => &state.config.entities.boss,
        };

        // Parse weapons from config
        let weapons: Vec<WeaponType> = entity_config
            .weapons
            .iter()
            .filter_map(|w| WeaponType::from_string(w))
            .collect();

        // Fallback to Bullet if no valid weapons configured
        let final_weapons = if weapons.is_empty() {
            vec![WeaponType::Bullet]
        } else {
            weapons
        };

        // Create enemy at random X position
        let enemy = Enemy {
            pos: Position {
                x: rand::gen_range(30.0, screen_width() - 30.0),
                y: -30.0, // Above screen
            },
            stats: entity_stats,
            entity_type,
            weapon: final_weapons,
        };

        state.enemies.push(enemy);
    }
}
