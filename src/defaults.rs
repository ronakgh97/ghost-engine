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
        starting_energy: 150.0,
        max_energy: 100.0,
        movement_speed: 200.0,
        starting_weapons: vec![
            "Bullet".to_string(),
            "Laser".to_string(),
            "Missile".to_string(),
            "Plasma".to_string(),
            "Bombs".to_string(),
        ],

        // Parry system
        parry_cooldown: 1.5,     // 2 second cooldown
        parry_window: 0.2,       // 0.2 second window for success
        parry_energy_cost: 10.0, // 10 energy per parry attempt
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
            weapons: vec!["Missile".to_string(), "Plasma".to_string()], // Tank: Homing + Spread
        },
        boss: EntityStats {
            health: 500.0,
            damage: 50.0,
            energy_cost: 80.0,
            fire_interval: 0.8,
            weapons: vec![
                "Laser".to_string(),
                "Missile".to_string(),
                "Bombs".to_string(),
                "Plasma".to_string(),
            ], // Boss arsenal: ALL advanced weapons!
        },
    }
}

fn default_weapons() -> WeaponsConfig {
    WeaponsConfig {
        // BULLET: Rapid fire, low damage, general purpose
        bullet: WeaponStats {
            damage: 10.0,
            fire_rate: 0.1, // Fast fire rate
            projectile_speed: 500.0,
            ammo: 100.0,
        },
        // LASER: Piercing beam, high damage, slow fire rate
        laser: WeaponStats {
            damage: 60.0,
            fire_rate: 1.5,          // Slow fire rate
            projectile_speed: 800.0, // Very fast
            ammo: 100.0,
        },
        // MISSILE: Homing, medium damage/fire rate
        missile: WeaponStats {
            damage: 30.0,
            fire_rate: 0.5,
            projectile_speed: 250.0, // Slow but tracks
            ammo: 100.0,
        },
        // PLASMA: Spread shot (TODO: implement 3-projectile cone)
        plasma: WeaponStats {
            damage: 25.0,
            fire_rate: 0.4,
            projectile_speed: 500.0,
            ammo: 100.0,
        },
        // BOMBS: AOE explosion (TODO: implement explosion radius)
        bombs: WeaponStats {
            damage: 80.0,
            fire_rate: 2.0,          // Very slow
            projectile_speed: 200.0, // Slow drop
            ammo: 100.0,
        },
    }
}

fn default_spawning() -> SpawningConfig {
    SpawningConfig {
        enemy_spawn_interval: 1.5,
        initial_delay: 3.0,
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
        // Weapon-specific shake (intensity scales with weapon power)
        bullet_hit_intensity: 0.8,  // Light tap - rapid fire feel
        laser_hit_intensity: 4.0,   // Medium - piercing beam power
        missile_hit_intensity: 2.5, // Strong - explosive impact
        plasma_hit_intensity: 1.5,  // Medium - energy burst (3 projectiles!)
        bomb_hit_intensity: 5.0,    // HUGE - massive explosion!
        weapon_hit_duration: 0.5,   // Quick shake for hits (not too long)

        // Event-specific shake
        enemy_death_duration: 1.0,  // Quick shake on enemy death
        enemy_death_intensity: 1.5, // Subtle shake
        parry_duration: 0.25,       // Longer shake for parry success
        parry_intensity: 5.0,       // Strong shake for skill reward
        player_hit_duration: 0.3,   // Longest shake when hit
        player_hit_intensity: 5.0,  // Strongest shake for feedback
    }
}
