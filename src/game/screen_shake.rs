use crate::models::*;
use macroquad::prelude::*;

/// Trigger screen shake with specific duration and intensity
pub fn trigger_shake(state: &mut GameState, duration: f32, intensity: f32) {
    state.screen_shake_duration = duration;
    state.screen_shake_intensity = intensity;
}

/// Update screen shake (decay over time)
pub fn update_shake(state: &mut GameState, delta: f32) {
    if state.screen_shake_duration > 0.0 {
        state.screen_shake_duration -= delta;
        if state.screen_shake_duration < 0.0 {
            state.screen_shake_duration = 0.0;
            state.screen_shake_intensity = 0.0;
        }
    }
}

/// Get camera offset based on current shake
pub fn get_shake_offset(state: &GameState) -> (f32, f32) {
    if state.screen_shake_duration <= 0.0 {
        return (0.0, 0.0);
    }

    // Random offset within intensity range
    let offset_x = rand::gen_range(-state.screen_shake_intensity, state.screen_shake_intensity);
    let offset_y = rand::gen_range(-state.screen_shake_intensity, state.screen_shake_intensity);

    (offset_x, offset_y)
}

/// Trigger shake on enemy death
pub fn shake_on_enemy_death(state: &mut GameState) {
    let cfg = &state.config.screen_shake;
    trigger_shake(state, cfg.enemy_death_duration, cfg.enemy_death_intensity);
}

/// Trigger shake on parry success
pub fn shake_on_parry(state: &mut GameState) {
    let cfg = &state.config.screen_shake;
    trigger_shake(state, cfg.parry_duration, cfg.parry_intensity);
}

/// Trigger shake on player hit
pub fn shake_on_player_hit(state: &mut GameState) {
    let cfg = &state.config.screen_shake;
    trigger_shake(state, cfg.player_hit_duration, cfg.player_hit_intensity);
}

/// Trigger weapon-specific shake on hit (when player/ghost hits enemy)
pub fn shake_on_weapon_hit(state: &mut GameState, weapon_type: WeaponType) {
    let cfg = &state.config.screen_shake;
    
    let intensity = match weapon_type {
        WeaponType::Bullet => cfg.bullet_hit_intensity,
        WeaponType::Laser => cfg.laser_hit_intensity,
        WeaponType::Missile => cfg.missile_hit_intensity,
        WeaponType::Plasma => cfg.plasma_hit_intensity,
        WeaponType::Bombs => cfg.bomb_hit_intensity,
    };
    
    trigger_shake(state, cfg.weapon_hit_duration, intensity);
}
