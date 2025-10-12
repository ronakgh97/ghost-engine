mod cancel_summon;
mod collision;
mod combat;
mod enemy;
mod energy;
mod ghost;
mod input;
mod parry;
mod player;
mod spawn;
mod utils;
mod weapons;

pub use cancel_summon::*;
pub use collision::*;
pub use combat::*;
pub use enemy::*;
pub use energy::*;
pub use ghost::*;
pub use input::*;
pub use parry::*;
pub use player::*;
pub use spawn::*;
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

    // Update weapons & projectiles
    update_weapons(state, delta);

    // Check collisions
    check_projectile_collisions(state);
    check_entity_collisions(state);

    // Manage resources
    manage_energy(state, delta);
    cleanup_dead_entities(state);

    // Spawn new enemies
    spawn_enemies(state, delta);
}
