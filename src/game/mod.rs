pub mod animation; // Easing functions and animation helpers (public for rendering)
mod bezier; // Bezier curve math for enemy paths
mod cancel_summon;
mod collision;
mod combat;
mod enemy;
mod energy;
mod ghost;
mod ghost_animation; // Ghost spawn/despawn animations
mod healer; // Healing system for healer enemies/ghosts
mod input;
mod parry;
mod particles;
mod player;
mod screen_shake;
mod spawn;
mod splitter; // Splitting system for splitter enemies/ghosts
mod utils;
pub mod wave; // Public module for WaveManager
mod weapons;

// Exports (some unused until features implemented)
// pub use cancel_summon::*; // TODO: Enable when cancel summon UI added
pub use bezier::*;
pub use collision::*;
pub use combat::*;
pub use enemy::*;
pub use energy::*;
pub use ghost::*;
pub use input::*;
pub use parry::*;
pub use particles::*;
pub use player::*;
pub use screen_shake::*;
pub use spawn::*;
// pub use wave::*; // WaveManager accessed via game::wave::WaveManager
pub use weapons::*;

use crate::models::*;

/// Main game loop
pub fn update_all_systems(state: &mut GameState, delta: f32) {
    // Handle input
    handle_input(state, delta);

    // Update parry system
    update_parry(state, delta);
    check_parry_projectiles(state);

    // Update entities
    update_player(state, delta);
    update_enemies(state, delta);
    update_ghosts(state, delta);
    
    // Update ghost animations (spawn/despawn effects)
    ghost_animation::update_ghost_animations(&mut state.ghosts, delta, &state.config.animations);

    // Update healing (healers heal allies, healer ghosts heal player)
    healer::update_healer_healing(state, delta);
    healer::update_ghost_healer_healing(state, delta);

    // Update weapons & projectiles
    update_weapons(state, delta);

    // Check collisions
    check_projectile_collisions(state);
    check_entity_collisions(state);

    // Manage resources
    manage_energy(state, delta);
    cleanup_dead_entities(state);

    // Wave-based spawning (replaces spawn_enemies)
    update_wave_system(state, delta);

    // Update visual effects
    update_particles(state, delta);
    update_shake(state, delta);
}

/// Update wave system (replaces random spawning)
fn update_wave_system(state: &mut GameState, delta: f32) {
    if state.config.spawning.wave_mode {
        // ===== WAVE MODE: Lua-based wave progression =====
        use crate::models::WaveState;
        use std::mem;

        // Temporarily take wave_manager out of state to avoid borrow issues
        let mut wave_manager = mem::replace(
            &mut state.wave_manager,
            crate::game::wave::WaveManager::new_dummy(),
        );

        // Check if we need to start the next wave
        if wave_manager.state == WaveState::Ready {
            let config = state.config.clone();
            wave_manager.start_next_wave(&config);
        }

        // Update wave state
        let enemies_alive = state.enemies.len();
        wave_manager.update_state(
            enemies_alive,
            &mut state.player.energy,
            state.player.max_energy,
            delta,
        );

        // Spawn enemies for active wave
        wave_manager.spawn_for_wave(state, delta);

        // Put wave_manager back
        state.wave_manager = wave_manager;
    } else {
        // Random enemy spawning for testing
        spawn_enemies(state, delta);
    }
}
