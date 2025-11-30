use macroquad::prelude::*;

/// Easing functions for smooth animations
/// All functions take normalized time t (0.0 to 1.0) and return eased value (0.0 to 1.0)
/// Linear interpolation - no easing
#[allow(dead_code)] //TODO: may be useful later
pub fn linear(t: f32) -> f32 {
    t
}

/// Ease in quadratic - slow start
pub fn ease_in_quad(t: f32) -> f32 {
    t * t
}

/// Ease out quadratic - slow end
pub fn ease_out_quad(t: f32) -> f32 {
    t * (2.0 - t)
}

/// Ease in-out quadratic - smooth both ends
#[allow(dead_code)] //TODO: may be useful later
pub fn ease_in_out_quad(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

/// Ease in cubic - stronger acceleration
#[allow(dead_code)] //TODO: may be useful later
pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

/// Ease out cubic - stronger deceleration
#[allow(dead_code)] //TODO: may be useful later
pub fn ease_out_cubic(t: f32) -> f32 {
    let t1 = t - 1.0;
    t1 * t1 * t1 + 1.0
}

/// Ease out elastic - overshoot with spring effect
pub fn ease_out_elastic(t: f32) -> f32 {
    if t == 0.0 || t == 1.0 {
        return t;
    }

    let p = 0.3;
    let s = p / 4.0;
    2.0_f32.powf(-10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin() + 1.0
}

/// Ease out bounce - bouncy landing
#[allow(dead_code)] //TODO: may be useful later
pub fn ease_out_bounce(t: f32) -> f32 {
    if t < 1.0 / 2.75 {
        7.5625 * t * t
    } else if t < 2.0 / 2.75 {
        let t1 = t - 1.5 / 2.75;
        7.5625 * t1 * t1 + 0.75
    } else if t < 2.5 / 2.75 {
        let t1 = t - 2.25 / 2.75;
        7.5625 * t1 * t1 + 0.9375
    } else {
        let t1 = t - 2.625 / 2.75;
        7.5625 * t1 * t1 + 0.984375
    }
}

/// Linear interpolation between two values
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Linear interpolation between two colors
#[allow(dead_code)] //TODO: may be useful later
pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        lerp(a.r, b.r, t),
        lerp(a.g, b.g, t),
        lerp(a.b, b.b, t),
        lerp(a.a, b.a, t),
    )
}

/// Oscillate using sine wave
#[allow(dead_code)] //TODO: may be useful later
pub fn oscillate(time: f32, frequency: f32, amplitude: f32, offset: f32) -> f32 {
    offset + amplitude * (time * frequency * std::f32::consts::TAU).sin()
}

/// Simple noise approximation for wiggle effects
#[allow(dead_code)] //TODO: may be useful later
pub fn wiggle(time: f32, seed: f32) -> f32 {
    ((time * 10.0 + seed).sin() * 1000.0).sin()
}

/// Update hit flash timer (counts down to 0)
/// Returns the flash intensity (0.0 = no flash, 1.0 = full flash)
pub fn update_hit_flash(hit_flash_timer: &mut f32, delta: f32, duration: f32) -> f32 {
    if *hit_flash_timer > 0.0 {
        *hit_flash_timer -= delta;
        if *hit_flash_timer < 0.0 {
            *hit_flash_timer = 0.0;
        }
        // Return normalized intensity (1.0 at start, 0.0 at end)
        *hit_flash_timer / duration
    } else {
        0.0
    }
}
