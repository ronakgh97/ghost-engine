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
            cooldown: 3.0,
        },
        laser: WeaponStats {
            damage: 80.0,
            fire_rate: 0.05,
            projectile_speed: 600.0,
            cooldown: 3.0,
        },
        missile: WeaponStats {
            damage: 30.0,
            fire_rate: 1.0,
            projectile_speed: 350.0,
            cooldown: 3.0,
        },
        plasma: WeaponStats {
            damage: 40.0,
            fire_rate: 1.5,
            projectile_speed: 450.0,
            cooldown: 3.0,
        },
        bombs: WeaponStats {
            damage: 50.0,
            fire_rate: 2.0,
            projectile_speed: 300.0,
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
        diamond_min: 4,
        diamond_optimal: 5,
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
