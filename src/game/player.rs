use crate::models::*;
use macroquad::prelude::*;

/// Update player position and state
pub fn update_player(state: &mut GameState, delta: f32) {
    // Update dash logic FIRST (sets velocity during dash)
    update_dash(state, delta);

    // Apply physics-based movement (after dash, only if not dashing)
    if !state.player.is_dashing {
        apply_physics_movement(state, delta);
    }

    // Store current position after calculating velocity
    let old_pos = state.player.last_pos;
    state.player.last_pos = state.player.pos;

    // Calculate velocity from position change (for lead targeting)
    // This happens after all movement, so it captures the actual movement
    if delta > 0.0 {
        state.player.velocity.x = (state.player.pos.x - old_pos.x) / delta;
        state.player.velocity.y = (state.player.pos.y - old_pos.y) / delta;
    }

    // Keep player on screen
    state.player.pos.x = state.player.pos.x.clamp(15.0, screen_width() - 15.0);

    // Player can only move in bottom half of screen
    state.player.pos.y = state
        .player
        .pos
        .y
        .clamp(screen_height() / 2.0, screen_height() - 15.0);

    // Update hit flash timer
    if state.player.hit_flash_timer > 0.0 {
        state.player.hit_flash_timer -= delta;
    }

    // Check for game over
    if state.player.stats.health <= 0.0 {
        // TODO: Implement game over screen
        state.player.stats.health = 0.0;
    }
}

/// Physics-based movement with acceleration, friction, and momentum
fn apply_physics_movement(state: &mut GameState, delta: f32) {
    let cfg = &state.config.player;
    let input = &state.player.input_direction;

    // Calculate target velocity based on input
    let target_velocity = Vec2::new(input.x * cfg.movement_speed, input.y * cfg.movement_speed);

    // Apply acceleration force towards target velocity (responsiveness controls turn speed)
    let acceleration_force = Vec2::new(
        (target_velocity.x - state.player.velocity.x) * cfg.responsiveness,
        (target_velocity.y - state.player.velocity.y) * cfg.responsiveness,
    );

    // Update velocity with acceleration
    state.player.velocity.x += acceleration_force.x * delta;
    state.player.velocity.y += acceleration_force.y * delta;

    // Apply friction (momentum decay)
    state.player.velocity.x *= cfg.friction;
    state.player.velocity.y *= cfg.friction;

    // Move player by velocity
    state.player.pos.x += state.player.velocity.x * delta;
    state.player.pos.y += state.player.velocity.y * delta;
}

/// Update dash movement and timers
fn update_dash(state: &mut GameState, delta: f32) {
    let dash_cfg = &state.config.dash;

    // Update cooldown timer
    if state.player.dash_cooldown_timer > 0.0 {
        state.player.dash_cooldown_timer -= delta;
    }

    // Update i-frame timer
    if state.player.i_frame_timer > 0.0 {
        state.player.i_frame_timer -= delta;
    }

    // Handle active dash
    if state.player.is_dashing {
        state.player.dash_timer -= delta;

        // Calculate dash speed (distance / duration)
        let dash_speed = dash_cfg.distance / dash_cfg.duration;

        // Apply dash movement (velocity-based for momentum integration)
        state.player.velocity.x = state.player.dash_direction.x * dash_speed;
        state.player.velocity.y = state.player.dash_direction.y * dash_speed;

        state.player.pos.x += state.player.velocity.x * delta;
        state.player.pos.y += state.player.velocity.y * delta;

        // Spawn trail particles
        state.player.dash_trail_timer += delta;
        let trail_interval = 1.0 / dash_cfg.trail_spawn_rate;
        if state.player.dash_trail_timer >= trail_interval {
            state.player.dash_trail_timer -= trail_interval;
            crate::game::particles::spawn_dash_trail(state, state.player.pos);
        }

        // End dash when timer expires
        if state.player.dash_timer <= 0.0 {
            state.player.is_dashing = false;
            state.player.dash_timer = 0.0;
            // Velocity carries over after dash (momentum preserved)
            // Friction will naturally slow down the player
        }
    }
}
