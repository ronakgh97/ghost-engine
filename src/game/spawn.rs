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
            EntityType::BasicFighter, // More common
            EntityType::Sniper,
            EntityType::Tank,
        ];

        let random_idx = rand::gen_range(0, enemy_types.len());
        let entity_type = enemy_types[random_idx];

        // Create enemy at random X position
        let enemy = Enemy {
            pos: Position {
                x: rand::gen_range(30.0, screen_width() - 30.0),
                y: -30.0, // Above screen
            },
            stats: entity_type.get_stats(&state.config.entities),
            entity_type,
            weapon: vec![WeaponType::Bullet],
        };

        state.enemies.push(enemy);
    }
}
