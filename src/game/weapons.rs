use crate::models::*;
use macroquad::prelude::*;

/// Update all weapon timers and projectiles
pub fn update_weapons(state: &mut GameState, delta: f32) {
    // Countdown fire cooldown
    state.player_fire_timer = (state.player_fire_timer - delta).max(0.0);

    // Update all projectiles
    update_projectiles(state, delta);
}

/// Fire player weapon if cooldown allows
pub fn player_fire_weapon(state: &mut GameState, weapon_index: usize) {
    if weapon_index >= state.player.weapon.len() {
        return;
    }

    let weapon = state.player.weapon[weapon_index];

    // Check cooldown
    if state.player_fire_timer > 0.0 {
        return;
    }

    state.player_fire_timer = weapon.get_weapon_stats(&state.config.weapons).fire_rate;

    let projectile = Projectile {
        pos: state.player.pos,
        velocity: Position {
            x: 0.0,
            y: state.config.projectile_bounds.player_projectile_speed_y,
        },
        damage: weapon.get_weapon_stats(&state.config.weapons).damage,
        weapon_type: weapon,
        owner: ProjectileOwner::Player,
    };

    state.projectiles.push(projectile);
}

/// Update all projectile positions and remove off-screen ones
fn update_projectiles(state: &mut GameState, delta: f32) {
    for projectile in &mut state.projectiles {
        projectile.pos.x += projectile.velocity.x * delta;
        projectile.pos.y += projectile.velocity.y * delta;
    }

    // Remove off-screen projectiles
    let padding = state.config.projectile_bounds.off_screen_padding;
    state.projectiles.retain(|p| {
        p.pos.y > -padding
            && p.pos.y < screen_height() + padding
            && p.pos.x > -padding
            && p.pos.x < screen_width() + padding
    });
}
