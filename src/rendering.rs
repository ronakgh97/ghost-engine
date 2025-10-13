use crate::game::get_shake_offset;
use crate::models::*;
use macroquad::prelude::*;

/// Render all game entities
pub fn render_game(state: &GameState) {
    // Apply screen shake offset to camera
    let (shake_x, shake_y) = get_shake_offset(state);

    // Push camera with shake offset
    gl_use_default_material();
    set_camera(&Camera2D {
        zoom: vec2(1.0 / screen_width() * 2.0, 1.0 / screen_height() * 2.0),
        target: vec2(
            screen_width() / 2.0 + shake_x,
            screen_height() / 2.0 + shake_y,
        ),
        ..Default::default()
    });

    draw_player(&state.player, state);
    draw_enemies(&state.enemies);
    draw_ghosts(&state.ghosts);
    draw_projectiles(&state.projectiles);
    draw_particles(&state.particles);

    // Reset camera for UI
    set_default_camera();
}

/// Render UI overlay (health, energy, stats)
pub fn render_ui(state: &GameState) {
    // Health bar
    let health_ratio = state.player.stats.health / state.player.stats.max_health;
    draw_rectangle(10.0, 10.0, 200.0, 20.0, DARKGRAY);
    draw_rectangle(10.0, 10.0, 200.0 * health_ratio, 20.0, RED);
    draw_text(
        &format!(
            "HP: {:.0}/{:.0}",
            state.player.stats.health, state.player.stats.max_health
        ),
        15.0,
        25.0,
        20.0,
        WHITE,
    );

    // Energy bar
    let energy_ratio = state.player.energy / state.player.max_energy;
    draw_rectangle(10.0, 35.0, 200.0, 20.0, DARKGRAY);
    draw_rectangle(10.0, 35.0, 200.0 * energy_ratio, 20.0, DARKPURPLE);
    draw_text(
        &format!(
            "Energy: {:.0}/{:.0}",
            state.player.energy, state.player.max_energy
        ),
        15.0,
        50.0,
        20.0,
        WHITE,
    );

    // Ghost queue count
    draw_text(
        &format!("Ghosts Ready: {}", state.player.available_ghosts.len()),
        10.0,
        75.0,
        20.0,
        GREEN,
    );

    draw_text(
        &format!("Ghosts Available: {:?}", state.player.available_ghosts),
        10.0,
        screen_height() - 25.0,
        18.0,
        GREEN,
    );

    // Active ghosts count
    draw_text(
        &format!("Active Ghosts: {}", state.ghosts.len()),
        10.0,
        95.0,
        20.0,
        SKYBLUE,
    );

    // Enemy count
    draw_text(
        &format!("Enemies: {}", state.enemies.len()),
        10.0,
        115.0,
        20.0,
        RED,
    );

    // Formation status with validation
    let formation_name = match state.ghost_formation {
        GhostFormation::VShape => "V-Shape",
        GhostFormation::Line => "Line",
        GhostFormation::Circle => "Circle",
    };

    let available_count = state.player.available_ghosts.len();
    let min_required = state.ghost_formation.min_ghost_count();
    let optimal = state.ghost_formation.optimal_ghost_count();

    // Formation status
    let formation_color = if available_count >= optimal {
        GREEN // Optimal
    } else if available_count >= min_required {
        YELLOW // Usable but not optimal
    } else {
        RED // Not enough ghosts
    };

    draw_text(
        &format!(
            "Formation: {} ({}/{})",
            formation_name, available_count, optimal
        ),
        10.0,
        135.0,
        18.0,
        formation_color,
    );

    // Energy status for formation spawn
    if available_count >= min_required {
        let mut total_cost = 0.0;
        for i in 0..available_count.min(optimal) {
            total_cost += state.player.available_ghosts[i].get_energy_cost(&state.config.entities);
        }

        let energy_color = if state.player.energy >= total_cost {
            GREEN
        } else {
            RED
        };

        draw_text(
            &format!("Formation Cost: {:.0} energy", total_cost),
            10.0,
            155.0,
            16.0,
            energy_color,
        );
    }

    // Parry status
    let parry_color = if state.player.parry_cooldown > 0.0 {
        RED // On cooldown
    } else {
        GREEN // Ready
    };

    let parry_status = if state.player.parry_active {
        "PARRY ACTIVE!".to_string()
    } else if state.player.parry_cooldown > 0.0 {
        format!("Parry: {:.1}s", state.player.parry_cooldown)
    } else {
        "Parry: READY".to_string()
    };

    draw_text(&parry_status, 10.0, 175.0, 18.0, parry_color);

    // Controls hint
    draw_text("SPACE - Deploy Formation", 10.0, 195.0, 16.0, GRAY);
    draw_text("F1-F4 - Spawn Single Ghost", 10.0, 215.0, 16.0, GRAY);
    draw_text("1-3 - Change Formation", 10.0, 235.0, 16.0, GRAY);
    draw_text("X - Parry Missiles", 10.0, 255.0, 16.0, GRAY);
    draw_text("C - Cancel Summon", 10.0, 275.0, 16.0, GRAY);

    // Controls hint
    draw_controls_hint();
}

/// Draw player entity
fn draw_player(player: &Player, state: &GameState) {
    draw_circle(player.pos.x, player.pos.y, 15.0, WHITE);

    // Draw parry shield if active
    if player.parry_active {
        let parry_radius = state.config.collision.player_radius + 20.0;
        draw_circle_lines(
            player.pos.x,
            player.pos.y,
            parry_radius,
            3.0,
            BLUE, // Bright blue
        );
        // Inner glow
        draw_circle_lines(player.pos.x, player.pos.y, parry_radius - 5.0, 2.0, SKYBLUE);
    }

    // Draw health bar above player
    let health_ratio = player.stats.health / player.stats.max_health;
    draw_rectangle(
        player.pos.x - 20.0,
        player.pos.y - 25.0,
        40.0,
        4.0,
        DARKGRAY,
    );
    draw_rectangle(
        player.pos.x - 20.0,
        player.pos.y - 25.0,
        40.0 * health_ratio,
        4.0,
        GREEN,
    );
}

/// Draw all enemies
fn draw_enemies(enemies: &[Enemy]) {
    for enemy in enemies {
        let color = get_enemy_color(enemy.entity_type);
        draw_circle(enemy.pos.x, enemy.pos.y, 15.0, color);

        // Draw health bar above enemy
        let health_ratio = enemy.stats.health / enemy.stats.max_health;
        draw_rectangle(enemy.pos.x - 15.0, enemy.pos.y - 20.0, 30.0, 3.0, DARKGRAY);
        draw_rectangle(
            enemy.pos.x - 15.0,
            enemy.pos.y - 20.0,
            30.0 * health_ratio,
            3.0,
            RED,
        );
    }
}

/// Draw all ghosts with transparency
fn draw_ghosts(ghosts: &[Ghost]) {
    for ghost in ghosts {
        let color = get_ghost_color(ghost.entity_type);
        draw_circle(ghost.pos.x, ghost.pos.y, 12.0, color);

        // Draw health bar above ghost
        let health_ratio = ghost.stats.health / ghost.stats.max_health;
        draw_rectangle(ghost.pos.x - 12.0, ghost.pos.y - 18.0, 24.0, 3.0, DARKGRAY);
        draw_rectangle(
            ghost.pos.x - 12.0,
            ghost.pos.y - 18.0,
            24.0 * health_ratio,
            3.0,
            SKYBLUE,
        );
    }
}

/// Draw all projectiles with weapon-specific visuals
fn draw_projectiles(projectiles: &[Projectile]) {
    for proj in projectiles {
        // Determine color/size based on weapon type and owner
        let (color, size, glow) = match proj.owner {
            ProjectileOwner::Player | ProjectileOwner::Ghost => match proj.weapon_type {
                WeaponType::Bullet => (YELLOW, 4.0, false),
                WeaponType::Laser => (GREEN, 6.0, true),
                WeaponType::Missile => (ORANGE, 5.0, true),
                WeaponType::Plasma => (PURPLE, 5.0, true),
                WeaponType::Bombs => (RED, 8.0, true),
            },
            ProjectileOwner::Enemy => (RED, 5.0, false),
        };

        // Draw glow effect for special weapons
        if glow {
            draw_circle(
                proj.pos.x,
                proj.pos.y,
                size + 4.0,
                Color::new(color.r, color.g, color.b, 0.3),
            );
        }

        draw_circle(proj.pos.x, proj.pos.y, size, color);

        // Draw beam line for lasers (elongated visual)
        if matches!(proj.weapon_type, WeaponType::Laser) {
            draw_line(
                proj.pos.x,
                proj.pos.y,
                proj.pos.x,
                proj.pos.y + 0.0, // Beam trail
                2.0,
                Color::new(color.r, color.g, color.b, 0.5),
            );
        }
    }
}

/// Get color based on enemy type
fn get_enemy_color(entity_type: EntityType) -> Color {
    match entity_type {
        EntityType::BasicFighter => RED,
        EntityType::Sniper => BLUE,
        EntityType::Tank => GREEN,
        EntityType::Boss => GOLD,
    }
}

/// Get color based on ghost type
fn get_ghost_color(entity_type: EntityType) -> Color {
    match entity_type {
        EntityType::BasicFighter => RED,
        EntityType::Sniper => BLUE,
        EntityType::Tank => GREEN,
        EntityType::Boss => GOLD,
    }
}

/// Draw controls hint
fn draw_controls_hint() {
    let controls = vec!["WASD - Move", "H/J - Fire Weapons", "F1-F4 - Spawn Ghosts"];

    let start_y = screen_height() - 80.0;
    draw_text("Controls:", screen_width() - 180.0, start_y, 18.0, WHITE);

    for (i, control) in controls.iter().enumerate() {
        draw_text(
            control,
            screen_width() - 180.0,
            start_y + 20.0 + (i as f32 * 18.0),
            16.0,
            GRAY,
        );
    }
}

/// Draw particles
fn draw_particles(particles: &[Particle]) {
    for particle in particles {
        // Calculate alpha based on lifetime (fade out)
        let alpha = (particle.lifetime / particle.max_lifetime).clamp(0.0, 1.0);
        let color = Color::new(
            particle.color.r,
            particle.color.g,
            particle.color.b,
            alpha,
        );
        
        // Draw particle as circle with current size
        draw_circle(particle.pos.x, particle.pos.y, particle.size, color);
        
        // Add glow effect for larger particles
        if particle.size > 3.0 {
            draw_circle(
                particle.pos.x,
                particle.pos.y,
                particle.size + 2.0,
                Color::new(color.r, color.g, color.b, alpha * 0.3),
            );
        }
    }
}
