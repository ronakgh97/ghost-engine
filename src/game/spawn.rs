use crate::game::utils::*;
use crate::models::*;
use macroquad::prelude::*;

/// Spawn random enemies (classic mode - not wave-based)
pub fn spawn_enemies(state: &mut GameState, delta: f32) {
    state.spawn_timer -= delta;

    if state.spawn_timer <= 0.0 {
        // Reset timer
        state.spawn_timer = state.config.spawning.enemy_spawn_interval;

        //  Spawn pool
        let enemy_types = vec![
            EntityType::BasicFighter,
            EntityType::Sniper,
            EntityType::Tank,
            EntityType::Healer,
            EntityType::Healer,
            EntityType::Healer,
        ];

        let random_idx = rand::gen_range(0, enemy_types.len());
        let entity_type = enemy_types[random_idx];

        // Get entity stats from config
        let entity_stats = entity_type.get_stats(&state.config.entities);
        
        // Get weapons list
        let weapons_list = match entity_type {
            EntityType::BasicFighter => &state.config.entities.basic_fighter.weapons,
            EntityType::Sniper => &state.config.entities.sniper.weapons,
            EntityType::Tank => &state.config.entities.tank.weapons,
            EntityType::Boss => &state.config.entities.boss.weapons,
            EntityType::Healer => &state.config.entities.healer.weapons,
        };

        // Parse weapons from config
        let weapons: Vec<WeaponType> = weapons_list
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
                x: rand::gen_range(40.0, screen_width() - 40.0),
                y: -30.0, // Above screen
            },
            stats: entity_stats,
            entity_type,
            weapon: final_weapons,
        };

        state.enemies.push(enemy);
    }
}
