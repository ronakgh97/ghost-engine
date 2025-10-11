use crate::game::utils::calculate_formation_position;
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

    // Formation switching
    change_formation(game_state);

    if is_key_pressed(KeyCode::Space) {
        spawn_formation(game_state);
    }

    // Ghost spawning
    spawn_ghost_from_queue(game_state);
}

/// Spawn single ghost when F-keys pressed
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

fn change_formation(state: &mut GameState) {
    use crate::models::GhostFormation;

    let current_ghost_count = state.player.available_ghosts.len();
    let new_formation = if is_key_pressed(KeyCode::Key1) {
        Some(GhostFormation::Scattered)
    } else if is_key_pressed(KeyCode::Key2) {
        Some(GhostFormation::Line)
    } else if is_key_pressed(KeyCode::Key3) {
        Some(GhostFormation::Circle)
    } else if is_key_pressed(KeyCode::Key4) {
        Some(GhostFormation::VShape)
    } else {
        None
    };

    // Validate formation can be used
    if let Some(formation) = new_formation {
        if formation.is_valid_for_count(current_ghost_count) {
            state.ghost_formation = formation;
            // TODO: Play formation switch sound
            println!("Switched to formation: {:?}", formation);
        } else {
            println!("Not enough ghosts to form : {:?}", formation);
            // TODO: Play error sound / show message
            // Can't switch - not enough ghosts in queue
        }
    }
}

/// Attempt to spawn a single ghost of specific type
fn try_spawn_ghost(state: &mut GameState, desired_type: EntityType) {
    // Find this ghost type in queue
    if let Some(index) = state
        .player
        .available_ghosts
        .iter()
        .position(|&t| t == desired_type)
    {
        let energy_cost = desired_type.get_energy_cost(&state.config.entities);

        // Check if player has enough energy
        if state.player.energy < energy_cost {
            // TODO: Show "Not enough energy!" message
            return;
        }

        // Calculate spawn position
        let spawn_pos = calculate_formation_position(
            state.player.pos,
            state.ghosts.len(),
            state.ghosts.len() + 1,
            state.ghost_formation,
            &state.config.formation_spacing,
        );

        // Create ghost
        let template_enemy = Enemy {
            pos: spawn_pos,
            stats: desired_type.get_stats(&state.config.entities),
            entity_type: desired_type,
            weapon: vec![WeaponType::Bullet],
        };

        let ghost = Ghost::from_enemy(&template_enemy, &state.config.entities);

        // All checks passed - spawn and deduct
        state.ghosts.push(ghost);
        state.player.available_ghosts.remove(index);
        state.player.energy -= energy_cost;
    }
}

fn spawn_formation(state: &mut GameState) {
    let formation = state.ghost_formation;
    let available_count = state.player.available_ghosts.len();

    // Check minimum requirement
    if !formation.is_valid_for_count(available_count) {
        // TODO: Show error message to player
        return;
    }

    // Determine how many ghosts to spawn
    let spawn_count = available_count.min(formation.optimal_ghost_count());

    // Calculate total energy cost
    let mut total_energy_cost = 0.0;
    for i in 0..spawn_count {
        if i < state.player.available_ghosts.len() {
            let ghost_type = state.player.available_ghosts[i];
            total_energy_cost += ghost_type.get_energy_cost(&state.config.entities);
        }
    }

    // Validate player has enough energy
    if state.player.energy < total_energy_cost {
        // TODO: Show "Not enough energy!" message
        return;
    }

    // Spawn all ghosts in formation
    for i in 0..spawn_count {
        if state.player.available_ghosts.is_empty() {
            break;
        }

        let ghost_type = state.player.available_ghosts.remove(0);

        // Calculate spawn position
        let spawn_pos = calculate_formation_position(
            state.player.pos,
            state.ghosts.len(),
            state.ghosts.len() + (spawn_count - i),
            formation,
            &state.config.formation_spacing,
        );

        // Create ghost
        let template_enemy = Enemy {
            pos: spawn_pos,
            stats: ghost_type.get_stats(&state.config.entities),
            entity_type: ghost_type,
            weapon: vec![WeaponType::Bullet],
        };

        let ghost = Ghost::from_enemy(&template_enemy, &state.config.entities);
        state.ghosts.push(ghost);
    }

    // Deduct energy AFTER successful spawn
    state.player.energy -= total_energy_cost;
}
