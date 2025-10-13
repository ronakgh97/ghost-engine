use crate::models::*;
use macroquad::prelude::*;

/// Spawn explosion particles at position
pub fn spawn_explosion(state: &mut GameState, pos: Position, count: usize, color: Color) {
    for _ in 0..count {
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let speed = rand::gen_range(50.0, 150.0);

        let particle = Particle {
            pos,
            velocity: Position {
                x: angle.cos() * speed,
                y: angle.sin() * speed,
            },
            lifetime: rand::gen_range(0.3, 0.6),
            max_lifetime: 0.6,
            color,
            size: rand::gen_range(3.0, 6.0),
            size_decay: 8.0, // Shrinks 8 pixels per second
        };

        state.particles.push(particle);
    }
}

/// Spawn hit sparks at position (directional)
pub fn spawn_hit_sparks(state: &mut GameState, pos: Position, direction: Position, count: usize) {
    for _ in 0..count {
        // Particles fly in reverse direction from impact
        let angle_offset = rand::gen_range(-0.5, 0.5);
        let base_angle = (-direction.y).atan2(-direction.x);
        let angle = base_angle + angle_offset;
        let speed = rand::gen_range(80.0, 200.0);

        let particle = Particle {
            pos,
            velocity: Position {
                x: angle.cos() * speed,
                y: angle.sin() * speed,
            },
            lifetime: rand::gen_range(0.15, 0.3),
            max_lifetime: 0.3,
            color: YELLOW,
            size: rand::gen_range(2.0, 4.0),
            size_decay: 10.0,
        };

        state.particles.push(particle);
    }
}

/// Spawn weapon-specific particles
pub fn spawn_weapon_particles(state: &mut GameState, pos: Position, weapon_type: WeaponType) {
    match weapon_type {
        WeaponType::Bullet => {
            // Small yellow sparks
            spawn_hit_sparks(state, pos, Position { x: 0.0, y: -1.0 }, 3);
        }
        WeaponType::Laser => {
            // Cyan energy burst
            spawn_explosion(state, pos, 8, SKYBLUE);
        }
        WeaponType::Missile => {
            // Orange explosion
            spawn_explosion(state, pos, 12, ORANGE);
        }
        WeaponType::Plasma => {
            // Purple energy
            spawn_explosion(state, pos, 6, PURPLE);
        }
        WeaponType::Bombs => {
            // HUGE red/orange explosion
            spawn_explosion(state, pos, 20, RED);
            spawn_explosion(state, pos, 15, ORANGE);
        }
    }
}

/// Spawn enemy death explosion
pub fn spawn_death_explosion(state: &mut GameState, pos: Position) {
    // Red/orange explosion
    spawn_explosion(state, pos, 15, RED);
    spawn_explosion(state, pos, 10, ORANGE);
    spawn_explosion(state, pos, 5, YELLOW);
}

/// Spawn parry deflection effect
pub fn spawn_parry_effect(state: &mut GameState, pos: Position) {
    // Blue/white energy burst
    spawn_explosion(state, pos, 12, SKYBLUE);
    spawn_explosion(state, pos, 8, WHITE);
}

/// Update all particles (movement, lifetime, cleanup)
pub fn update_particles(state: &mut GameState, delta: f32) {
    // Update particles
    for particle in &mut state.particles {
        particle.pos.x += particle.velocity.x * delta;
        particle.pos.y += particle.velocity.y * delta;
        particle.lifetime -= delta;
        particle.size -= particle.size_decay * delta;

        // Slow down over time
        particle.velocity.x *= 0.95;
        particle.velocity.y *= 0.95;
    }

    // Remove dead particles
    state.particles.retain(|p| p.lifetime > 0.0 && p.size > 0.0);
}
