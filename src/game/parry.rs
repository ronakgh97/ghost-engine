use crate::game::particles::spawn_parry_effect;
use crate::game::screen_shake::shake_on_parry;
use crate::models::*;
use macroquad::prelude::*;

/// Attempt to activate parry
pub fn attempt_parry(state: &mut GameState) {
    // Check cooldown
    if state.player.parry_cooldown > 0.0 {
        println!(
            "✘ Parry on cooldown ({:.1}s remaining)",
            state.player.parry_cooldown
        );
        return;
    }

    // Check energy cost
    let parry_cost = state.config.player.parry_energy_cost;
    if state.player.energy < parry_cost {
        println!("✘ Not enough energy to parry!");
        return;
    }

    // Activate parry
    state.player.parry_active = true;
    state.player.parry_window = state.config.player.parry_window;
    state.player.energy -= parry_cost;

    // Start stance glow animation (lasts longer than parry window!)
    state.player.parry_stance_glow_timer = state.config.animations.parry_stance_glow_duration;

    // TODO: Play parry animation/sound
}

/// Update parry timers
pub fn update_parry(state: &mut GameState, delta: f32) {
    // Update parry window
    if state.player.parry_active {
        state.player.parry_window -= delta;
        if state.player.parry_window <= 0.0 {
            state.player.parry_active = false;
            state.player.parry_cooldown = state.config.player.parry_cooldown;

            // Trigger failed parry animation (shrink + desaturation)
            state.player.parry_failed_timer = state.config.animations.parry_failed_duration;

            println!("✘ Parry window missed!");
        }
    }

    // Update cooldown
    if state.player.parry_cooldown > 0.0 {
        state.player.parry_cooldown -= delta;
    }

    // Update animation timers
    if state.player.parry_success_scale_timer > 0.0 {
        state.player.parry_success_scale_timer -= delta;
    }
    if state.player.parry_failed_timer > 0.0 {
        state.player.parry_failed_timer -= delta;
    }
    if state.player.parry_stance_glow_timer > 0.0 {
        state.player.parry_stance_glow_timer -= delta;
    }
}

/// Check for parryable projectiles and deflect them
pub fn check_parry_projectiles(state: &mut GameState) {
    if !state.player.parry_active {
        return;
    }

    let parry_radius = state.config.collision.player_radius + 20.0; // Slightly larger than hitbox
    let mut parried_count = 0;

    // Check enemy projectiles near player
    for projectile in &mut state.projectiles {
        // Only MISSILES can be parried (slow-moving, visible, high-skill reward)
        if projectile.owner == ProjectileOwner::Enemy
            && projectile.weapon_type == WeaponType::Missile
        {
            let dx = projectile.pos.x - state.player.pos.x;
            let dy = projectile.pos.y - state.player.pos.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < parry_radius {
                // PARRY SUCCESS
                projectile.owner = ProjectileOwner::Player; // Now damages enemies!
                projectile.velocity.x *= -1.5; // Reverse and boost speed
                projectile.velocity.y *= -1.5;

                // Re-enable homing for parried missile (will track nearest enemy)
                projectile.homing = true;
                projectile.locked_target_index = None; // Find new target (nearest enemy)

                parried_count += 1;
            }
        }
    }

    // If successful parry, deactivate and set cooldown
    if parried_count > 0 {
        println!("✔ Parry ({parried_count} projectiles deflected)");
        state.player.parry_active = false;
        state.player.parry_cooldown = state.config.player.parry_cooldown;

        // Trigger success animation (elastic bounce)
        state.player.parry_success_scale_timer = state.config.animations.parry_success_duration;

        // BOOST stance glow for burst effect! (extends and intensifies the glow)
        let burst_duration = state.config.animations.parry_success_duration;
        if state.player.parry_stance_glow_timer < burst_duration {
            state.player.parry_stance_glow_timer = burst_duration; // Extend to at least bounce duration
        }

        shake_on_parry(state);
        spawn_parry_effect(state, state.player.pos); // Particle burst!
        // TODO: Sound effect
    }
}
