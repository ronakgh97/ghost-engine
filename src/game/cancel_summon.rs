use crate::models::*;

/// Cancel all summoned ghosts - they return to available queue
pub fn cancel_summon(state: &mut GameState) {
    if state.ghosts.is_empty() {
        println!("✘ No ghosts to dismiss!");
        return;
    }

    let ghost_count = state.ghosts.len();

    // Return ghosts to available queue
    for ghost in &state.ghosts {
        state.player.available_ghosts.push(ghost.entity_type);
    }

    // Clear all ghosts and their timers
    state.ghosts.clear();
    state.ghost_fire_timers.clear();

    println!("✔ Cancel Deployed, {} ghost(s)!", ghost_count);
    // TODO: Play despawn sound/effect
}
