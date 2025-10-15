use crate::models::*;
use macroquad::prelude::*;

/// Handle enemy splitting when killed
/// Returns a vector of new split enemies to spawn
pub fn handle_enemy_splits(
    dead_enemies: &[Enemy],
    config: &crate::config::GameConfig,
) -> Vec<Enemy> {
    let mut new_enemies = Vec::new();

    for enemy in dead_enemies {
        // Only splitters split when killed
        if enemy.entity_type != EntityType::Splitter {
            continue;
        }

        let split_count = config.entities.splitter.split_count;
        let split_health_ratio = config.entities.splitter.split_health_ratio;
        let original_max_hp = enemy.stats.max_health;
        let split_hp = original_max_hp * split_health_ratio;

        // Spawn splits in a horizontal line (side-by-side)
        for i in 0..split_count {
            let spacing = 40.0; // Horizontal distance between splits
            let x_offset = if split_count == 1 {
                0.0
            } else {
                // Center the line around death position
                (i as f32 - (split_count as f32 - 1.0) / 2.0) * spacing
            };

            let split_enemy = Enemy {
                pos: Position {
                    x: enemy.pos.x + x_offset,
                    y: enemy.pos.y, // Same Y position (horizontal line)
                },
                stats: Stats {
                    health: split_hp,
                    max_health: split_hp,
                    damage: enemy.stats.damage, // Same damage as parent
                },
                weapon: enemy.weapon.clone(),          // Inherit weapons
                entity_type: EntityType::BasicFighter, // Splits become basic fighters (don't split again!)
            };

            new_enemies.push(split_enemy);
        }

        // Console feedback
        println!(
            "✓ Splitter split into {} enemies at ({:.0}, {:.0})!",
            split_count, enemy.pos.x, enemy.pos.y
        );
    }

    new_enemies
}

/// Spawn visual split effect particles (shows splitting animation)
pub fn spawn_split_particles(
    state: &mut GameState,
    pos: Position,
    split_count: usize,
    is_ghost: bool,
) {
    let color = if is_ghost { GREEN } else { ORANGE };

    // Create directional burst showing split directions
    for i in 0..split_count {
        let spacing = 40.0;
        let x_offset = if split_count == 1 {
            0.0
        } else {
            (i as f32 - (split_count as f32 - 1.0) / 2.0) * spacing
        };

        // Spawn particles moving from center to split position
        for j in 0..8 {
            let progress = j as f32 / 8.0;
            let target_x = pos.x + x_offset;

            let particle = Particle {
                pos,
                velocity: Position {
                    x: (target_x - pos.x) * 3.0, // Move horizontally towards split position
                    y: rand::gen_range(-20.0, 20.0), // Some vertical spread
                },
                lifetime: 0.5 + progress * 0.3, // Staggered lifetime
                max_lifetime: 0.8,
                color,
                size: rand::gen_range(3.0, 6.0),
                size_decay: 0.95,
            };

            state.particles.push(particle);
        }
    }

    // Central burst showing the split happening
    for _ in 0..15 {
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let speed = rand::gen_range(50.0, 120.0);

        let particle = Particle {
            pos,
            velocity: Position {
                x: angle.cos() * speed,
                y: angle.sin() * speed,
            },
            lifetime: rand::gen_range(0.3, 0.6),
            max_lifetime: 0.6,
            color,
            size: rand::gen_range(4.0, 8.0),
            size_decay: 0.92,
        };

        state.particles.push(particle);
    }
}

/// Handle ghost splitter behavior - creates temporary clones when damaged
/// Returns optional clone ghost to spawn
/// NOTE: This is a future feature, not yet integrated into the damage system
#[allow(dead_code)]
pub fn handle_ghost_splitter_damage(
    ghost: &Ghost,
    _config: &crate::config::GameConfig, // Reserved for future config options
) -> Option<Ghost> {
    // Only splitter ghosts create clones
    if ghost.entity_type != EntityType::Splitter {
        return None;
    }

    // 20% chance to spawn clone when taking damage
    if rand::gen_range(0.0, 1.0) > 0.2 {
        return None;
    }

    // Create temporary clone with reduced stats
    let clone = Ghost {
        pos: Position {
            x: ghost.pos.x + rand::gen_range(-40.0, 40.0),
            y: ghost.pos.y + rand::gen_range(-40.0, 40.0),
        },
        stats: Stats {
            health: ghost.stats.health * 0.5, // Clone has 50% HP
            max_health: ghost.stats.max_health * 0.5,
            damage: ghost.stats.damage,
        },
        weapon_type: ghost.weapon_type.clone(),
        entity_type: EntityType::Splitter,
        energy_drain_per_sec: 0.0, // Clones don't drain energy!
        anim: EntityAnimState::new_spawning(0.3), // Quick spawn animation for clones
    };

    println!("✓ Ghost splitter created a clone!");
    Some(clone)
}

/// Handle ghost splitter splitting when killed
/// Returns a vector of new split ghosts to spawn
pub fn handle_ghost_splits(
    dead_ghosts: &[Ghost],
    config: &crate::config::GameConfig,
) -> Vec<Ghost> {
    let mut new_ghosts = Vec::new();

    for ghost in dead_ghosts {
        // Only splitter ghosts split when killed
        if ghost.entity_type != EntityType::Splitter {
            continue;
        }

        let split_count = config.entities.splitter.split_count;
        let split_health_ratio = config.entities.splitter.split_health_ratio;
        let original_max_hp = ghost.stats.max_health;
        let split_hp = original_max_hp * split_health_ratio;

        // Spawn splits in a horizontal line (side-by-side)
        for i in 0..split_count {
            let spacing = 40.0; // Horizontal distance between splits
            let x_offset = if split_count == 1 {
                0.0
            } else {
                // Center the line around death position
                (i as f32 - (split_count as f32 - 1.0) / 2.0) * spacing
            };

            let split_ghost = Ghost {
                pos: Position {
                    x: ghost.pos.x + x_offset,
                    y: ghost.pos.y, // Same Y position
                },
                stats: Stats {
                    health: split_hp,
                    max_health: split_hp,
                    damage: ghost.stats.damage, // Same damage as parent
                },
                weapon_type: ghost.weapon_type.clone(), // Inherit weapons
                entity_type: EntityType::BasicFighter,  // Splits become basic fighters
                energy_drain_per_sec: ghost.energy_drain_per_sec, // Same energy drain
                anim: EntityAnimState::new_spawning(0.4), // Spawn animation for splits
            };

            new_ghosts.push(split_ghost);
        }
    }

    new_ghosts
}
