mod collision;
mod combat;
mod enemy;
mod energy;
mod ghost;
mod input;
mod player;
mod spawn;
mod utils;
mod weapons;

pub use collision::*;
pub use combat::*;
pub use enemy::*;
pub use energy::*;
pub use ghost::*;
pub use input::*;
pub use player::*;
pub use spawn::*;
pub use weapons::*;

use crate::models::*;

/// Main game loop - orchestrates all systems
pub fn update_all_systems(state: &mut GameState, delta: f32) {
    // 1. Handle input
    handle_input(state, delta);

    // 2. Update entities
    update_player(state, delta);
    update_enemies(state, delta);
    update_ghosts(state, delta);

    // 3. Update weapons & projectiles
    update_weapons(state, delta);

    // 4. Check collisions
    check_projectile_collisions(state);
    check_entity_collisions(state);

    // 5. Manage resources
    manage_energy(state, delta);
    cleanup_dead_entities(state);

    // 6. Spawn new enemies
    spawn_enemies(state, delta);
}
