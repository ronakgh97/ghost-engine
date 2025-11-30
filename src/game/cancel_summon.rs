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

    // Trigger despawn animation for all ghosts instead of instant clear
    for ghost in &mut state.ghosts {
        if !ghost.anim.is_despawning {
            ghost
                .anim
                .start_despawn(state.config.animations.ghost_despawn_duration);
        }
    }

    println!("✔ Cancel Deployed, {ghost_count} ghost(s)!");
    // TODO: Play despawn sound/effect
}
