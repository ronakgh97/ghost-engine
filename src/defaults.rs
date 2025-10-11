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
        starting_energy: 100.0,
        max_energy: 100.0,
        movement_speed: 200.0,
        starting_weapons: vec!["Bullet".to_string(), "Laser".to_string()],
    }
}

fn default_energy() -> EnergyConfig {
    EnergyConfig {
        regen_rate_idle: 30.0,
        regen_rate_active: 10.0,
    }
}

fn default_entities() -> EntitiesConfig {
    EntitiesConfig {
        basic_fighter: EntityStats {
            health: 50.0,
            damage: 10.0,
            energy_cost: 15.0,
            fire_interval: 2.0,
        },
        sniper: EntityStats {
            health: 30.0,
            damage: 25.0,
            energy_cost: 25.0,
            fire_interval: 3.0,
        },
        tank: EntityStats {
            health: 150.0,
            damage: 15.0,
            energy_cost: 40.0,
            fire_interval: 1.5,
        },
        boss: EntityStats {
            health: 500.0,
            damage: 50.0,
            energy_cost: 80.0,
            fire_interval: 0.8,
        },
    }
}

fn default_weapons() -> WeaponsConfig {
    WeaponsConfig {
        bullet: WeaponStats {
            damage: 10.0,
            fire_rate: 0.4,
            projectile_speed: 400.0,
            ammo: 100.0,
            cooldown: 3.0,
        },
        laser: WeaponStats {
            damage: 80.0,
            fire_rate: 0.05,
            projectile_speed: 600.0,
            ammo: 100.0,
            cooldown: 3.0,
        },
        missile: WeaponStats {
            damage: 30.0,
            fire_rate: 1.0,
            projectile_speed: 350.0,
            ammo: 100.0,
            cooldown: 3.0,
        },
        plasma: WeaponStats {
            damage: 40.0,
            fire_rate: 1.5,
            projectile_speed: 450.0,
            ammo: 100.0,
            cooldown: 3.0,
        },
        bombs: WeaponStats {
            damage: 50.0,
            fire_rate: 2.0,
            projectile_speed: 300.0,
            ammo: 100.0,
            cooldown: 3.0,
        },
    }
}

fn default_spawning() -> SpawningConfig {
    SpawningConfig {
        enemy_spawn_interval: 2.0,
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
        scattered_min: 1,
        scattered_optimal: 4,
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
        scattered_x_min: -80.0,
        scattered_x_max: 80.0,
        scattered_y_min: -100.0,
        scattered_y_max: -40.0,
        screen_edge_padding: 30.0,
    }
}

fn default_projectile_bounds() -> ProjectileBoundsConfig {
    ProjectileBoundsConfig {
        off_screen_padding: 50.0,
        player_projectile_speed_y: -400.0,
    }
}
