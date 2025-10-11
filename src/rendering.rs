use crate::models::*;
use macroquad::prelude::*;

/// Render all game entities
pub fn render_game(state: &GameState) {
    draw_player(&state.player);
    draw_enemies(&state.enemies);
    draw_ghosts(&state.ghosts);
    draw_projectiles(&state.projectiles);
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
        15.0,
        GRAY,
    );

    // Active ghosts count
    draw_text(
        &format!("Active Ghosts: {}", state.ghosts.len()),
        10.0,
        95.0,
        20.0,
        SKYBLUE,
    );

    // Enemy count (debug)
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
        GhostFormation::Scattered => "Scattered",
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

    // Controls hint
    draw_text("SPACE - Deploy Formation", 10.0, 175.0, 16.0, GRAY);
    draw_text("F1-F4 - Spawn Single Ghost", 10.0, 195.0, 16.0, GRAY);
    draw_text("1-4 - Change Formation", 10.0, 215.0, 16.0, GRAY);

    // Controls hint
    draw_controls_hint();
}

/// Draw player entity
fn draw_player(player: &Player) {
    draw_circle(player.pos.x, player.pos.y, 15.0, BLUE);

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

/// Draw all projectiles
fn draw_projectiles(projectiles: &[Projectile]) {
    for proj in projectiles {
        let color = match proj.owner {
            ProjectileOwner::Player | ProjectileOwner::Ghost => SKYBLUE,
            ProjectileOwner::Enemy => RED,
        };
        draw_circle(proj.pos.x, proj.pos.y, 5.0, color);
    }
}

/// Get color based on enemy type
fn get_enemy_color(entity_type: EntityType) -> Color {
    match entity_type {
        EntityType::BasicFighter => RED,
        EntityType::Sniper => ORANGE,
        EntityType::Tank => PURPLE,
        EntityType::Boss => GOLD,
    }
}

/// Get color based on ghost type (semi-transparent)
fn get_ghost_color(entity_type: EntityType) -> Color {
    match entity_type {
        EntityType::BasicFighter => Color::new(0.5, 1.0, 0.5, 0.6),
        EntityType::Sniper => Color::new(1.0, 0.7, 0.3, 0.6),
        EntityType::Tank => Color::new(0.7, 0.5, 1.0, 0.6),
        EntityType::Boss => Color::new(1.0, 1.0, 0.5, 0.6),
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
