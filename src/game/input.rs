use crate::game::weapons;
use crate::models::{Enemy, EntityType, GameState, Ghost, WeaponType};
use macroquad::input::*;

/// Handle all player input
pub fn handle_input(game_state: &mut GameState, delta_time: f32) {
    // Player movement
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        game_state.player.pos.y += 200.0 * delta_time;
    }

    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        game_state.player.pos.y -= 200.0 * delta_time;
    }

    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        game_state.player.pos.x -= 200.0 * delta_time;
    }

    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        game_state.player.pos.x += 200.0 * delta_time;
    }

    // Weapon firing
    if is_key_down(KeyCode::H) {
        weapons::player_fire_weapon(game_state, 0);
    }

    if is_key_down(KeyCode::J) {
        weapons::player_fire_weapon(game_state, 1);
    }

    // Ghost spawning
    spawn_ghost_from_queue(game_state);
}

/// Spawn ghost when F-keys pressed
fn spawn_ghost_from_queue(state: &mut GameState) {
    let desired_type = if is_key_pressed(KeyCode::F1) {
        Some(EntityType::BasicFighter)
    } else if is_key_pressed(KeyCode::F2) {
        Some(EntityType::Sniper)
    } else if is_key_pressed(KeyCode::F3) {
        Some(EntityType::Tank)
    } else if is_key_pressed(KeyCode::F4) {
        Some(EntityType::Boss)
    } else {
        None
    };

    if let Some(ghost_type) = desired_type {
        try_spawn_ghost(state, ghost_type);
    }
}

/// Attempt to spawn a ghost of specific type
fn try_spawn_ghost(state: &mut GameState, desired_type: EntityType) {
    // Find this ghost type in queue
    if let Some(index) = state.player.available_ghosts.iter().position(|&t| t == desired_type) {
        let energy_cost = desired_type.get_energy_cost();

        // Check if player has enough energy
        if state.player.energy >= energy_cost {
            // Create a "template" enemy to convert (hacky but works)
            let template_enemy = Enemy {
                pos: state.player.pos,
                stats: desired_type.get_stats(),
                entity_type: desired_type,
                weapon: vec![WeaponType::Bullet],
            };

            // Use Ghost::from_enemy to preserve identity
            let ghost = Ghost::from_enemy(&template_enemy);

            state.ghosts.push(ghost);
            state.player.available_ghosts.remove(index);
        }
    }
}

