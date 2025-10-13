use crate::models::*;
use macroquad::prelude::*;

/// Spawn explosion particles at position
pub fn spawn_explosion(state: &mut GameState, pos: Position, count: usize, color: Color) {
    let cfg = &state.config.particles;

    for _ in 0..count {
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let speed = rand::gen_range(cfg.explosion_speed_min, cfg.explosion_speed_max);
        let lifetime = rand::gen_range(cfg.explosion_lifetime_min, cfg.explosion_lifetime_max);

        let particle = Particle {
            pos,
            velocity: Position {
                x: angle.cos() * speed,
                y: angle.sin() * speed,
            },
            lifetime,
            max_lifetime: cfg.explosion_lifetime_max,
            color,
            size: rand::gen_range(cfg.explosion_size_min, cfg.explosion_size_max),
            size_decay: cfg.size_decay,
        };

        state.particles.push(particle);
    }
}

/// Spawn hit sparks at position (directional)
pub fn spawn_hit_sparks(state: &mut GameState, pos: Position, direction: Position) {
    let cfg = &state.config.particles;

    for _ in 0..cfg.spark_count {
        // Particles fly in reverse direction from impact
        let angle_offset = rand::gen_range(-0.5, 0.5);
        let base_angle = (-direction.y).atan2(-direction.x);
        let angle = base_angle + angle_offset;
        let speed = rand::gen_range(cfg.spark_speed_min, cfg.spark_speed_max);
        let lifetime = rand::gen_range(cfg.spark_lifetime_min, cfg.spark_lifetime_max);

        let particle = Particle {
            pos,
            velocity: Position {
                x: angle.cos() * speed,
                y: angle.sin() * speed,
            },
            lifetime,
            max_lifetime: cfg.spark_lifetime_max,
            color: YELLOW,
            size: rand::gen_range(cfg.spark_size_min, cfg.spark_size_max),
            size_decay: cfg.size_decay,
        };

        state.particles.push(particle);
    }
}

/// Spawn weapon-specific particles
pub fn spawn_weapon_particles(state: &mut GameState, pos: Position, weapon_type: WeaponType) {
    let cfg = &state.config.particles;

    // Extract config values before calling spawn functions (borrow checker)
    let laser_count = cfg.laser_particle_count;
    let missile_count = cfg.missile_particle_count;
    let plasma_count = cfg.plasma_particle_count;
    let bomb_red_count = cfg.bomb_red_particle_count;
    let bomb_orange_count = cfg.bomb_orange_particle_count;

    match weapon_type {
        WeaponType::Bullet => {
            // Small yellow sparks
            spawn_hit_sparks(state, pos, Position { x: 0.0, y: -1.0 });
        }
        WeaponType::Laser => {
            // Cyan energy burst
            spawn_explosion(state, pos, laser_count, SKYBLUE);
        }
        WeaponType::Missile => {
            // Orange explosion
            spawn_explosion(state, pos, missile_count, ORANGE);
        }
        WeaponType::Plasma => {
            // Purple energy
            spawn_explosion(state, pos, plasma_count, PURPLE);
        }
        WeaponType::Bombs => {
            // HUGE red/orange explosion
            spawn_explosion(state, pos, bomb_red_count, RED);
            spawn_explosion(state, pos, bomb_orange_count, ORANGE);
        }
    }
}

/// Spawn enemy death explosion
pub fn spawn_death_explosion(state: &mut GameState, pos: Position) {
    let cfg = &state.config.particles;

    // Extract config values (borrow checker)
    let red_count = cfg.death_red_count;
    let orange_count = cfg.death_orange_count;
    let yellow_count = cfg.death_yellow_count;

    // Red/orange/yellow explosion
    spawn_explosion(state, pos, red_count, RED);
    spawn_explosion(state, pos, orange_count, ORANGE);
    spawn_explosion(state, pos, yellow_count, YELLOW);
}

/// Spawn parry deflection effect
pub fn spawn_parry_effect(state: &mut GameState, pos: Position) {
    let cfg = &state.config.particles;

    // Extract config values (borrow checker)
    let blue_count = cfg.parry_blue_count;
    let white_count = cfg.parry_white_count;

    // Blue/white energy burst
    spawn_explosion(state, pos, blue_count, SKYBLUE);
    spawn_explosion(state, pos, white_count, WHITE);
}

/// Update all particles (movement, lifetime, cleanup)
pub fn update_particles(state: &mut GameState, delta: f32) {
    let friction = state.config.particles.friction;

    // Update particles
    for particle in &mut state.particles {
        particle.pos.x += particle.velocity.x * delta;
        particle.pos.y += particle.velocity.y * delta;
        particle.lifetime -= delta;
        particle.size -= particle.size_decay * delta;

        // Slow down over time (friction)
        particle.velocity.x *= friction;
        particle.velocity.y *= friction;
    }

    // Remove dead particles
    state.particles.retain(|p| p.lifetime > 0.0 && p.size > 0.0);
}
