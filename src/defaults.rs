//! Default game values - compiled into binary

use crate::config::*;

/// Get default configuration (always available)
pub fn default_config() -> GameConfig {
    GameConfig {
        window: default_window(),
        player: default_player(),
        energy: default_energy(),
        entities: default_entities(),
        weapons: default_weapons(),
        spawning: default_spawning(),
        formations: default_formations(),
        debug: default_debug(),
        collision: default_collision(),
        ghost_behavior: default_ghost_behavior(),
        enemy_behavior: default_enemy_behavior(),
        formation_spacing: default_formation_spacing(),
        projectile_bounds: default_projectile_bounds(),
        screen_shake: default_screen_shake(),
        particles: default_particles(),
        background: default_background(),
    }
}

fn default_window() -> WindowConfig {
    WindowConfig {
        title: "Ghost Ally Shooter".to_string(),
        width: 800,
        height: 600,
        target_fps: 60,
    }
}

fn default_player() -> PlayerConfig {
    PlayerConfig {
        starting_health: 100.0,
        max_health: 100.0,
        starting_energy: 200.0,
        max_energy: 500.0,
        movement_speed: 250.0,
        starting_weapons: vec![
            "Bullet".to_string(),
            "Laser".to_string(),
            "Missile".to_string(),
            "Plasma".to_string(),
        ],

        // Parry system
        parry_cooldown: 1.5,
        parry_window: 0.3,
        parry_energy_cost: 5.0,
    }
}

fn default_energy() -> EnergyConfig {
    EnergyConfig {
        regen_rate_idle: 30.0,
        regen_rate_active: 1.0,
    }
}

fn default_entities() -> EntitiesConfig {
    EntitiesConfig {
        basic_fighter: EntityStats {
            health: 50.0,
            damage: 10.0,
            energy_cost: 15.0,
            fire_interval: 2.0,
            weapons: vec!["Bullet".to_string()], // Simple straight shots
        },
        sniper: EntityStats {
            health: 30.0,
            damage: 25.0,
            energy_cost: 25.0,
            fire_interval: 4.0,
            weapons: vec!["Laser".to_string()], // Piercing beam
        },
        tank: EntityStats {
            health: 150.0,
            damage: 15.0,
            energy_cost: 40.0,
            fire_interval: 1.5,
            weapons: vec!["Missile".to_string(), "Plasma".to_string()], // Homing + Spread
        },
        boss: EntityStats {
            health: 500.0,
            damage: 50.0,
            energy_cost: 80.0,
            fire_interval: 0.8,
            weapons: vec![
                "Laser".to_string(),
                "Missile".to_string(),
                "Plasma".to_string(),
            ],
        },
        healer: HealerStats {
            health: 60.0,
            damage: 5.0,
            energy_cost: 20.0,
            fire_interval: 3.0,
            weapons: vec!["Bullet".to_string()],
            // Healing stats
            heal_rate: 15.0,    // Heals 15 HP/sec to allies in range
            heal_radius: 150.0, // 150 pixel radius healing field
        },
        splitter: SplitterStats {
            health: 80.0,
            damage: 12.0,
            energy_cost: 25.0,
            fire_interval: 2.0,
            weapons: vec!["Bullet".to_string()],
            // Splitting stats
            split_count: 3,
            split_health_ratio: 0.3,
            split_speed_multiplier: 2.0,
        },
    }
}

// Weapon config with balanced stats
fn default_weapons() -> WeaponsConfig {
    WeaponsConfig {
        bullet: WeaponStats {
            damage: 10.0,
            fire_rate: 0.1,
            projectile_speed: 500.0,
        },
        laser: WeaponStats {
            damage: 60.0,
            fire_rate: 1.5,
            projectile_speed: 800.0,
        },
        missile: WeaponStats {
            damage: 30.0,
            fire_rate: 0.5,
            projectile_speed: 250.0,
        },
        plasma: WeaponStats {
            damage: 25.0,
            fire_rate: 0.4,
            projectile_speed: 500.0,
        },
        bombs: WeaponStats {
            damage: 80.0,
            fire_rate: 2.0,
            projectile_speed: 200.0,
        },
    }
}

fn default_spawning() -> SpawningConfig {
    SpawningConfig {
        wave_mode: false,          // Enable Lua wave system by default
        wave_count: 5,             // 5 waves total
        enemy_spawn_interval: 2.2, // Random spawn timer
        initial_delay: 3.0,        // Initial delay before spawning
    }
}

fn default_formations() -> FormationsConfig {
    FormationsConfig {
        v_shape_min: 2,
        v_shape_optimal: 6,
        line_min: 3,
        line_optimal: 5,
        circle_min: 4,
        circle_optimal: 8,
    }
}

fn default_debug() -> DebugConfig {
    DebugConfig {
        show_hitboxes: false,
        show_fps: true,
    }
}

fn default_collision() -> CollisionConfig {
    CollisionConfig {
        projectile_radius: 5.0,
        enemy_radius: 15.0,
        player_radius: 15.0,
        ghost_radius: 12.0,
    }
}

fn default_ghost_behavior() -> GhostBehaviorConfig {
    GhostBehaviorConfig {
        fire_interval: 1.0,
        movement_threshold_y: 200.0,
        fast_ascent_speed: 50.0,
        slow_hover_speed: 100.0,
        projectile_speed: 350.0,
        screen_boundary_top: -50.0,
    }
}

fn default_enemy_behavior() -> EnemyBehaviorConfig {
    EnemyBehaviorConfig {
        movement_threshold_y: 200.0,
        fast_descent_speed: 100.0,
        slow_hover_speed: 50.0,
        fire_threshold_y: 50.0,
        screen_boundary_bottom: 650.0,
        basic_projectile_speed_y: 250.0,
    }
}

fn default_formation_spacing() -> FormationSpacingConfig {
    FormationSpacingConfig {
        v_shape_spacing: 40.0,
        v_shape_vertical_factor: 0.8,
        line_spacing: 50.0,
        line_height_offset: 80.0,
        circle_radius: 70.0,
        screen_edge_padding: 30.0,
    }
}

fn default_projectile_bounds() -> ProjectileBoundsConfig {
    ProjectileBoundsConfig {
        off_screen_padding: 50.0,
        player_projectile_speed_y: -400.0,
    }
}

fn default_screen_shake() -> ScreenShakeConfig {
    ScreenShakeConfig {
        bullet_hit_intensity: 0.8,
        laser_hit_intensity: 4.0,
        missile_hit_intensity: 2.5,
        plasma_hit_intensity: 1.5,
        bomb_hit_intensity: 5.0,
        weapon_hit_duration: 0.5,

        // Event-specific shake
        enemy_death_duration: 1.0,
        enemy_death_intensity: 1.5,
        parry_duration: 0.25,
        parry_intensity: 5.0,
        player_hit_duration: 0.3,
        player_hit_intensity: 5.0,
    }
}

fn default_particles() -> ParticleConfig {
    ParticleConfig {
        // Explosion particles (radial burst)
        explosion_count_min: 10,
        explosion_count_max: 20,
        explosion_lifetime_min: 0.3,
        explosion_lifetime_max: 0.6,
        explosion_size_min: 3.0,
        explosion_size_max: 6.0,
        explosion_speed_min: 50.0,
        explosion_speed_max: 150.0,

        // Hit sparks (directional)
        spark_count: 3,
        spark_lifetime_min: 0.15,
        spark_lifetime_max: 0.3,
        spark_size_min: 2.0,
        spark_size_max: 4.0,
        spark_speed_min: 80.0,
        spark_speed_max: 200.0,

        // Weapon-specific particle counts
        bullet_particle_count: 3,       // Small sparks
        laser_particle_count: 15,       // Energy burst
        missile_particle_count: 12,     // Explosion
        plasma_particle_count: 6,       // Energy
        bomb_red_particle_count: 20,    // HUGE explosion
        bomb_orange_particle_count: 15, // Secondary burst

        // Death explosion counts
        death_red_count: 15,
        death_orange_count: 10,
        death_yellow_count: 5,

        // Parry effect counts
        parry_blue_count: 12,
        parry_white_count: 8,

        // Physics
        friction: 0.95,  // 5% slowdown per frame
        size_decay: 8.0, // Shrinks 8 pixels/sec

        // Projectile trails
        trails_enabled: true,        // Trails enabled by default
        trail_spawn_interval: 0.005, // Spawn rate
        trail_lifetime: 0.3,         // Trail fades in 0.5 seconds
        trail_size: 3.5,             // Small trail particles
    }
}

fn default_background() -> BackgroundConfig {
    BackgroundConfig {
        scroll_speed: 5.0, // Pixels per second
    }
}
