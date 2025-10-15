use crate::game::get_shake_offset;
use crate::models::*;
use macroquad::prelude::*;
use std::collections::HashMap;

/// Render all game entities with scrolling background
pub fn render_game(state: &GameState, space_texture: &Option<Texture2D>) {
    // Draw scrolling background BEFORE camera setup
    draw_scrolling_background(space_texture, state.bg_scroll_offset);

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

    // Render entities
    draw_player(&state.player, state);
    draw_enemies(&state.enemies);
    draw_ghosts(&state.ghosts);
    draw_projectiles(&state.projectiles);
    draw_particles(&state.particles);

    // Reset camera for UI
    set_default_camera();
}

/// Draw SCROLLING background
fn draw_scrolling_background(space_texture: &Option<Texture2D>, scroll_offset: f32) {
    if let Some(texture) = space_texture {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let tex_h = texture.height();

        // Start at bottom (max_scroll) and move toward 0
        let max_scroll = (tex_h - screen_h).max(0.0);
        let clamped_offset = max_scroll - (scroll_offset % max_scroll).max(0.0);

        // Draw texture with vertical offset
        draw_texture_ex(
            texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_w, screen_h)),
                source: Some(Rect::new(0.0, clamped_offset, screen_w, screen_h)),
                ..Default::default()
            },
        );
    } else {
        // Fallback: solid gradient (if texture fails to load)
        for y in 0..screen_height() as i32 {
            let ratio = y as f32 / screen_height();
            let color = Color::new(
                0.02 + ratio * 0.03,
                0.02 + ratio * 0.06,
                0.12 + ratio * 0.18,
                1.0,
            );
            draw_line(0.0, y as f32, screen_width(), y as f32, 1.0, color);
        }
    }
}

/// Render UI overlay
pub fn render_ui(state: &GameState) {
    // Player Stats (Top Left)
    draw_panel(10.0, 10.0, 210.0, 150.0);

    let mut ui_y = 20.0;

    // Health bar
    draw_stat_bar(
        20.0,
        ui_y,
        190.0,
        20.0,
        state.player.stats.health / state.player.stats.max_health,
        RED,
        &format!(
            "HP: {:.0}/{:.0}",
            state.player.stats.health, state.player.stats.max_health
        ),
    );
    ui_y += 25.0;

    // Energy bar
    draw_stat_bar(
        20.0,
        ui_y,
        190.0,
        20.0,
        state.player.energy / state.player.max_energy,
        SKYBLUE,
        &format!(
            "NRG: {:.0}/{:.0}",
            state.player.energy, state.player.max_energy
        ),
    );
    ui_y += 35.0;

    // Ghost in queue
    draw_text(
        &format!(
            "Ghosts: {}/{}",
            state.ghosts.len(),
            state.player.available_ghosts.len()
        ),
        20.0,
        ui_y,
        18.0,
        GREEN,
    );
    ui_y += 22.0;

    // Formation status
    let formation_name = match state.ghost_formation {
        GhostFormation::VShape => "V-Shape",
        GhostFormation::Line => "Line",
        GhostFormation::Circle => "Circle",
    };

    let available_count = state.player.available_ghosts.len();
    let optimal = state.ghost_formation.optimal_ghost_count();

    // Calculate total energy cost for full formation
    let mut formation_cost = 0.0;
    for i in 0..available_count.min(optimal) {
        formation_cost += state.player.available_ghosts[i].get_energy_cost(&state.config.entities);
    }

    // Determine color based on formation readiness AND energy
    let formation_color = if available_count < state.ghost_formation.min_ghost_count() {
        RED // Not enough ghosts
    } else if state.player.energy < formation_cost {
        ORANGE // Not enough energy
    } else if available_count >= optimal {
        GREEN // Ready to deploy!
    } else {
        YELLOW // Can deploy but not optimal
    };

    // Display formation name and ghost count
    draw_text(
        &format!("{formation_name} ({available_count}/{optimal})"),
        20.0,
        ui_y,
        16.0,
        formation_color,
    );
    ui_y += 20.0;

    // Display energy cost
    let energy_color = if state.player.energy >= formation_cost {
        GREEN
    } else {
        RED
    };

    draw_text(
        &format!("Cost: {formation_cost:.0} NRG"),
        20.0,
        ui_y,
        16.0,
        energy_color,
    );
    ui_y += 22.0;

    // Parry status
    let (parry_text, parry_color) = if state.player.parry_active {
        ("PARRY ACTIVE", BLUE)
    } else if state.player.parry_cooldown > 0.0 {
        ("Parry Cooldown", RED)
    } else {
        ("Parry Ready", GREEN)
    };

    draw_text(parry_text, 20.0, ui_y, 16.0, parry_color);

    // Available Ghosts (Bottom Left)
    let ghost_panel_y = screen_height() - 180.0;
    draw_panel(10.0, ghost_panel_y, 170.0, 170.0);

    draw_text("Available Ghosts:", 20.0, ghost_panel_y + 20.0, 18.0, WHITE);

    // Count each ghost type
    let mut ghost_counts: HashMap<EntityType, usize> = HashMap::new();

    for ghost_type in &state.player.available_ghosts {
        *ghost_counts.entry(*ghost_type).or_insert(0) += 1;
    }

    let mut line_y = ghost_panel_y + 45.0;

    // BasicFighter
    let count = ghost_counts.get(&EntityType::BasicFighter);
    draw_circle(25.0, line_y - 5.0, 6.0, RED);
    draw_text(&format!("Fighter: {count:?}"), 40.0, line_y, 16.0, WHITE);
    line_y += 22.0;

    // Sniper
    let count = ghost_counts.get(&EntityType::Sniper);
    draw_circle(25.0, line_y - 5.0, 6.0, BLUE);
    draw_text(&format!("Sniper: {count:?}"), 40.0, line_y, 16.0, WHITE);
    line_y += 22.0;

    // Tank
    let count = ghost_counts.get(&EntityType::Tank);
    draw_circle(25.0, line_y - 5.0, 6.0, GREEN);
    draw_text(&format!("Tank: {count:?}"), 40.0, line_y, 16.0, WHITE);
    line_y += 22.0;

    // Healer
    let count = ghost_counts.get(&EntityType::Healer);
    draw_circle(25.0, line_y - 5.0, 6.0, LIME);
    draw_text(&format!("Healer: {count:?}"), 40.0, line_y, 16.0, WHITE);
    line_y += 22.0;

    // Splitter
    let count = ghost_counts.get(&EntityType::Splitter);
    draw_circle(25.0, line_y - 5.0, 6.0, ORANGE);
    draw_text(&format!("Splitter: {count:?}"), 40.0, line_y, 16.0, WHITE);
    line_y += 22.0;

    // Elite
    let count = ghost_counts.get(&EntityType::Elite);
    draw_circle(25.0, line_y - 5.0, 6.0, GOLD);
    draw_text(&format!("Elite: {count:?}"), 40.0, line_y, 16.0, WHITE);
}

/// Draw modern panel with shadow
fn draw_panel(x: f32, y: f32, w: f32, h: f32) {
    // Shadow
    draw_rectangle(x + 4.0, y + 4.0, w, h, Color::from_rgba(0, 0, 0, 100));
    // Background
    draw_rectangle(x, y, w, h, Color::from_rgba(0, 0, 0, 50));
    // Border
    draw_rectangle_lines(x, y, w, h, 4.0, Color::from_rgba(100, 150, 200, 180));
}

/// Draw stat bar with modern style
fn draw_stat_bar(x: f32, y: f32, w: f32, h: f32, ratio: f32, color: Color, label: &str) {
    // Background
    draw_rectangle(x, y, w, h, DARKGRAY);
    // Filled portion
    draw_rectangle(x, y, w * ratio.clamp(0.0, 1.0), h, color);
    // Border
    draw_rectangle_lines(x, y, w, h, 2.0, WHITE);
    // Label
    draw_text(label, x + 5.0, y + h - 5.0, 16.0, WHITE);
}

/// Draw player entity with enhanced visuals
fn draw_player(player: &Player, state: &GameState) {
    // Player glow effect
    draw_circle(
        player.pos.x,
        player.pos.y,
        20.0,
        Color::new(1.0, 1.0, 1.0, 0.2),
    );

    // Main player body
    draw_circle(player.pos.x, player.pos.y, 15.0, WHITE);

    // Inner core
    draw_circle(player.pos.x, player.pos.y, 10.0, SKYBLUE);

    // Draw parry shield if active
    if player.parry_active {
        let parry_radius = state.config.collision.player_radius + 25.0;
        let pulse = (macroquad::time::get_time() * 10.0).sin() as f32 * 3.0;
        draw_circle_lines(player.pos.x, player.pos.y, parry_radius + pulse, 4.0, BLUE);
        draw_circle_lines(
            player.pos.x,
            player.pos.y,
            parry_radius + pulse - 5.0,
            2.0,
            SKYBLUE,
        );
    }

    // Health bar above player
    let health_ratio = player.stats.health / player.stats.max_health;
    draw_rectangle(player.pos.x - 20.0, player.pos.y - 27.0, 40.0, 4.0, BLACK);
    draw_rectangle(
        player.pos.x - 20.0,
        player.pos.y - 30.0,
        40.0 * health_ratio,
        4.0,
        GREEN,
    );
}

/// Draw all enemies with enhanced visuals
fn draw_enemies(enemies: &[Enemy]) {
    for enemy in enemies {
        // Apply animation state
        let anim = &enemy.anim;
        
        // Draw healing field for healers (pulsing green circle)
        if enemy.entity_type == EntityType::Healer {
            // Pulse effect using sine wave
            let pulse = (macroquad::time::get_time() * 2.0).sin() as f32 * 0.1 + 0.9;
            let heal_radius = 150.0 * pulse; // From config

            // Draw healing radius (transparent green circle)
            draw_circle_lines(
                enemy.pos.x,
                enemy.pos.y,
                heal_radius,
                2.0,
                Color::new(0.2, 1.0, 0.2, 0.3),
            );

            // Inner healing glow
            draw_circle(
                enemy.pos.x,
                enemy.pos.y,
                heal_radius * 0.5,
                Color::new(0.2, 1.0, 0.2, 0.05),
            );
        }

        let base_color = get_enemy_color(enemy.entity_type);
        
        // Apply hit flash (lerp toward white when hit)
        let flash_intensity = anim.hit_flash_timer / 0.15; // Normalize (assumes 0.15s duration)
        let color = Color::new(
            base_color.r + (1.0 - base_color.r) * flash_intensity * 0.8, // Lerp R to white
            base_color.g + (1.0 - base_color.g) * flash_intensity * 0.8, // Lerp G to white
            base_color.b + (1.0 - base_color.b) * flash_intensity * 0.8, // Lerp B to white
            base_color.a,
        );

        // Glow effect (with hit flash)
        draw_circle(
            enemy.pos.x,
            enemy.pos.y,
            20.0,
            Color::new(color.r, color.g, color.b, 0.2),
        );

        // Main body (with hit flash)
        draw_circle(enemy.pos.x, enemy.pos.y, 15.0, color);

        // Inner detail (darker, with hit flash)
        draw_circle(
            enemy.pos.x,
            enemy.pos.y,
            10.0,
            Color::new(color.r * 0.7, color.g * 0.7, color.b * 0.7, 1.0),
        );

        // Health bar
        let health_ratio = enemy.stats.health / enemy.stats.max_health;
        draw_rectangle(enemy.pos.x - 15.0, enemy.pos.y - 22.0, 30.0, 3.0, BLACK);
        draw_rectangle(
            enemy.pos.x - 15.0,
            enemy.pos.y - 22.0,
            30.0 * health_ratio,
            3.0,
            RED,
        );
    }
}

/// Draw all ghosts with transparency
fn draw_ghosts(ghosts: &[Ghost]) {
    for ghost in ghosts {
        // Apply animation state
        let anim = &ghost.anim;
        let base_radius = 12.0;
        let glow_radius = 18.0;
        
        // Apply scale and alpha from animation
        let radius = base_radius * anim.scale;
        let glow_rad = glow_radius * anim.scale;
        
        // Draw healing field for healer ghosts (pulsing green circle)
        if ghost.entity_type == EntityType::Healer {
            // Pulse effect using sine wave
            let pulse = (macroquad::time::get_time() * 2.0).sin() as f32 * 0.1 + 0.9;
            let heal_radius = 150.0 * pulse * anim.scale; // Scale healing field too

            // Draw healing radius (transparent green circle)
            draw_circle_lines(
                ghost.pos.x,
                ghost.pos.y,
                heal_radius,
                2.0,
                Color::new(0.2, 1.0, 0.2, 0.3 * anim.alpha), // Apply alpha
            );

            // Inner healing glow
            draw_circle(
                ghost.pos.x,
                ghost.pos.y,
                heal_radius * 0.5,
                Color::new(0.2, 1.0, 0.2, 0.05 * anim.alpha), // Apply alpha
            );
        }

        let base_color = get_ghost_color(ghost.entity_type);
        
        // Apply hit flash (lerp toward white when hit)
        let flash_intensity = anim.hit_flash_timer / 0.15; // Normalize (assumes 0.15s duration)
        let color = Color::new(
            base_color.r + (1.0 - base_color.r) * flash_intensity * 0.8, // Lerp R to white
            base_color.g + (1.0 - base_color.g) * flash_intensity * 0.8, // Lerp G to white
            base_color.b + (1.0 - base_color.b) * flash_intensity * 0.8, // Lerp B to white
            base_color.a,
        );

        // Ghost glow (with animation alpha and hit flash)
        draw_circle(
            ghost.pos.x,
            ghost.pos.y,
            glow_rad,
            Color::new(color.r, color.g, color.b, 0.3 * anim.alpha),
        );

        // Ghost body (with animation scale, alpha, and hit flash)
        draw_circle(
            ghost.pos.x,
            ghost.pos.y,
            radius,
            Color::new(color.r, color.g, color.b, 0.7 * anim.alpha),
        );

        // Health bar (only show if not despawning and alpha > 0.5)
        if !anim.is_despawning && anim.alpha > 0.5 {
            let health_ratio = ghost.stats.health / ghost.stats.max_health;
            let bar_width = 24.0 * anim.scale;
            let bar_offset = radius + 7.0;
            
            draw_rectangle(
                ghost.pos.x - bar_width / 2.0,
                ghost.pos.y - bar_offset,
                bar_width,
                2.0,
                Color::new(0.0, 0.0, 0.0, anim.alpha),
            );
            draw_rectangle(
                ghost.pos.x - bar_width / 2.0,
                ghost.pos.y - bar_offset,
                bar_width * health_ratio,
                2.0,
                Color::new(0.53, 0.81, 0.92, anim.alpha), // SKYBLUE with alpha
            );
        }
    }
}

/// Draw all projectiles with weapon-specific visuals
fn draw_projectiles(projectiles: &[Projectile]) {
    for proj in projectiles {
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

        // Draw glow effect
        if glow {
            draw_circle(
                proj.pos.x,
                proj.pos.y,
                size + 5.0,
                Color::new(color.r, color.g, color.b, 0.3),
            );
        }

        // Main projectile
        draw_circle(proj.pos.x, proj.pos.y, size, color);

        // Laser beam trail
        if matches!(proj.weapon_type, WeaponType::Laser) {
            draw_line(
                proj.pos.x,
                proj.pos.y,
                proj.pos.x + 5.0,
                proj.pos.y + 5.0,
                2.0,
                Color::new(color.r, color.g, color.b, 0.5),
            );
        }

        // Missile fins
        if matches!(proj.weapon_type, WeaponType::Missile) {
            draw_circle(proj.pos.x + 5.0, proj.pos.y + 5.0, 2.0, GRAY);
            draw_circle(proj.pos.x + 5.0, proj.pos.y + 5.0, 2.0, GRAY);
        }
    }
}

/// Draw particles with enhanced effects
fn draw_particles(particles: &[Particle]) {
    for particle in particles {
        let alpha = (particle.lifetime / particle.max_lifetime).clamp(0.0, 1.0);
        let color = Color::new(particle.color.r, particle.color.g, particle.color.b, alpha);

        // Determine particle shape based on position (pseudo-random but consistent)
        let shape_variant = (particle.pos.x * 123.45 + particle.pos.y * 67.89) as i32 % 3;

        match shape_variant {
            0 => {
                // Circle (default) - brightest core
                draw_circle(particle.pos.x, particle.pos.y, particle.size, color);
            }
            1 => {
                // Star shape
                draw_star(particle.pos.x, particle.pos.y, particle.size, color);
            }
            _ => {
                // Rotated square
                draw_rotated_square(particle.pos.x, particle.pos.y, particle.size, color);
            }
        }

        // Enhanced glow with multiple layers (creates additive-like effect)
        if particle.size > 2.0 {
            let glow_alpha = alpha * 0.5;

            // Outer glow
            draw_circle(
                particle.pos.x,
                particle.pos.y,
                particle.size + 1.5,
                Color::new(color.r, color.g, color.b, glow_alpha * 0.2),
            );

            // Mid glow
            draw_circle(
                particle.pos.x,
                particle.pos.y,
                particle.size + 1.0,
                Color::new(color.r, color.g, color.b, glow_alpha * 0.4),
            );

            // Inner glow
            draw_circle(
                particle.pos.x,
                particle.pos.y,
                particle.size + 0.5,
                Color::new(color.r, color.g, color.b, glow_alpha),
            );
        }
    }
}

/// Draw a 4-pointed star
fn draw_star(x: f32, y: f32, size: f32, color: Color) {
    let points = 4;
    let inner_radius = size * 0.4;
    let outer_radius = size;

    for i in 0..points {
        let angle1 =
            (i as f32 * std::f32::consts::PI * 2.0 / points as f32) - std::f32::consts::PI / 2.0;
        let angle2 = ((i as f32 + 0.5) * std::f32::consts::PI * 2.0 / points as f32)
            - std::f32::consts::PI / 2.0;
        let angle3 = ((i as f32 + 1.0) * std::f32::consts::PI * 2.0 / points as f32)
            - std::f32::consts::PI / 2.0;

        let x1 = x + angle1.cos() * outer_radius;
        let y1 = y + angle1.sin() * outer_radius;
        let x2 = x + angle2.cos() * inner_radius;
        let y2 = y + angle2.sin() * inner_radius;
        let x3 = x + angle3.cos() * outer_radius;
        let y3 = y + angle3.sin() * outer_radius;

        draw_triangle(vec2(x, y), vec2(x1, y1), vec2(x2, y2), color);
        draw_triangle(vec2(x, y), vec2(x2, y2), vec2(x3, y3), color);
    }
}

/// Draw a rotated square (diamond shape)
fn draw_rotated_square(x: f32, y: f32, size: f32, color: Color) {
    let half = size * 0.7; // Slightly smaller for visual balance
    let points = [
        vec2(x, y - half), // Top
        vec2(x + half, y), // Right
        vec2(x, y + half), // Bottom
        vec2(x - half, y), // Left
    ];

    // Draw as two triangles
    draw_triangle(points[0], points[1], points[2], color);
    draw_triangle(points[0], points[2], points[3], color);
}

/// Get color based on enemy type
fn get_enemy_color(entity_type: EntityType) -> Color {
    match entity_type {
        EntityType::BasicFighter => RED,
        EntityType::Sniper => BLUE,
        EntityType::Tank => GREEN,
        EntityType::Elite => GOLD,
        EntityType::Healer => LIME,
        EntityType::Splitter => BROWN,
    }
}

/// Get color based on ghost type
fn get_ghost_color(entity_type: EntityType) -> Color {
    match entity_type {
        EntityType::BasicFighter => RED,
        EntityType::Sniper => BLUE,
        EntityType::Tank => GREEN,
        EntityType::Elite => GOLD,
        EntityType::Healer => LIME,
        EntityType::Splitter => BROWN,
    }
}
