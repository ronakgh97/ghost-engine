use crate::models::*;

/// Manage player energy (drain from ghosts, regenerate over time)
pub fn manage_energy(state: &mut GameState, delta: f32) {
    // Drain energy from active ghosts
    for ghost in &state.ghosts {
        state.player.energy -= ghost.energy_drain_per_sec * delta;
    }

    // Despawn all ghosts if energy depleted
    if state.player.energy <= 0.0 {
        // Trigger despawn animation for all ghosts
        for ghost in &mut state.ghosts {
            if !ghost.anim.is_despawning {
                ghost.anim.start_despawn(state.config.animations.ghost_despawn_duration);
            }
        }
        state.player.energy = 0.0;
    }

    // Regenerate energy (slower when ghosts active)
    let regen_rate = if state.ghosts.is_empty() {
        state.config.energy.regen_rate_idle // Fast regen
    } else {
        state.config.energy.regen_rate_active // Slow regen
    };

    state.player.energy = (state.player.energy + regen_rate * delta).min(state.player.max_energy);
}
