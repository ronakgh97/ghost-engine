use crate::config::AnimationConfig;
use crate::game::animation;
use crate::game::animation::*;
use crate::models::*;

/// Update ghost spawn animation
/// Returns true if animation is complete
pub fn update_ghost_spawn_animation(
    ghost: &mut Ghost,
    delta: f32,
    config: &AnimationConfig,
) -> bool {
    if !ghost.anim.is_spawning {
        return true; // Already spawned
    }

    ghost.anim.spawn_timer -= delta;

    if ghost.anim.spawn_timer > 0.0 {
        // Calculate progress (0.0 at start → 1.0 at end)
        let t = 1.0 - (ghost.anim.spawn_timer / config.ghost_spawn_duration);

        // Apply elastic easing for bouncy entrance
        let eased_t = ease_out_elastic(t);

        // Interpolate scale from start size to full size
        ghost.anim.scale = lerp(config.ghost_spawn_scale_start, 1.0, eased_t);

        // Fade in alpha
        ghost.anim.alpha = ease_out_quad(t); // Smooth fade

        // Rotate while spawning (spin effect)
        ghost.anim.rotation += config.ghost_spawn_rotation_speed * delta;

        false // Still animating
    } else {
        // Animation complete
        ghost.anim.is_spawning = false;
        ghost.anim.scale = 1.0;
        ghost.anim.alpha = 1.0;
        ghost.anim.rotation = 0.0;
        true // Animation done
    }
}

/// Update ghost despawn animation
/// Returns true if animation is complete (ready to remove)
pub fn update_ghost_despawn_animation(
    ghost: &mut Ghost,
    delta: f32,
    config: &AnimationConfig,
) -> bool {
    if !ghost.anim.is_despawning {
        return false; // Not despawning
    }

    ghost.anim.despawn_timer -= delta;

    if ghost.anim.despawn_timer > 0.0 {
        // Calculate progress (0.0 at start → 1.0 at end)
        let t = 1.0 - (ghost.anim.despawn_timer / config.ghost_despawn_duration);

        // Apply ease-in for accelerating shrink
        let eased_t = ease_in_quad(t);

        // Shrink to 0
        ghost.anim.scale = lerp(1.0, 0.0, eased_t);

        // Fade out
        ghost.anim.alpha = 1.0 - eased_t;

        // Spin faster while despawning
        ghost.anim.rotation += config.ghost_despawn_rotation_speed * delta;

        false // Still despawning
    } else {
        // Despawn animation complete
        true // Ready to remove
    }
}

/// Update all ghost animations
pub fn update_ghost_animations(ghosts: &mut Vec<Ghost>, delta: f32, config: &AnimationConfig) {
    for ghost in ghosts.iter_mut() {
        // Update spawn animation if spawning
        if ghost.anim.is_spawning {
            update_ghost_spawn_animation(ghost, delta, config);
        }

        // Update despawn animation if despawning
        if ghost.anim.is_despawning {
            update_ghost_despawn_animation(ghost, delta, config);
        }

        // Update hit flash (always runs, counts down to 0)
        animation::update_hit_flash(
            &mut ghost.anim.hit_flash_timer,
            delta,
            config.hit_flash_duration,
        );
    }

    // Remove ghosts that finished despawning
    ghosts.retain(|g| {
        // Keep if not despawning OR still has time left
        !g.anim.is_despawning || g.anim.despawn_timer > 0.0
    });
}
