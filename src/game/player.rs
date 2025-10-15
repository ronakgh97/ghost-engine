use crate::models::*;
use macroquad::prelude::*;

/// Update player position and state
pub fn update_player(state: &mut GameState, delta: f32) {
    // Calculate velocity from position change (for lead targeting)
    if delta > 0.0 {
        state.player.velocity.x = (state.player.pos.x - state.player.last_pos.x) / delta;
        state.player.velocity.y = (state.player.pos.y - state.player.last_pos.y) / delta;
    }
    
    // Store current position for next frame
    state.player.last_pos = state.player.pos;
    
    // Update dash logic
    update_dash(state, delta);
    
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
        
        // Apply dash movement
        state.player.pos.x += state.player.dash_direction.x * dash_speed * delta;
        state.player.pos.y += state.player.dash_direction.y * dash_speed * delta;
        
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
        }
    }
}
