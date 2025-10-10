use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub window: WindowConfig,
    pub player: PlayerConfig,
    pub energy: EnergyConfig,
    pub entities: EntitiesConfig,
    pub weapons: WeaponsConfig,
    pub spawning: SpawningConfig,
    pub formations: FormationsConfig,
    pub debug: DebugConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub target_fps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerConfig {
    pub starting_health: f32,
    pub max_health: f32,
    pub starting_energy: f32,
    pub max_energy: f32,
    pub movement_speed: f32,
    pub starting_weapons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyConfig {
    pub regen_rate_idle: f32,
    pub regen_rate_active: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitiesConfig {
    pub basic_fighter: EntityStats,
    pub sniper: EntityStats,
    pub tank: EntityStats,
    pub boss: EntityStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityStats {
    pub health: f32,
    pub damage: f32,
    pub energy_cost: f32,
    pub fire_interval: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponsConfig {
    pub bullet: WeaponStats,
    pub laser: WeaponStats,
    pub missile: WeaponStats,
    pub plasma: WeaponStats,
    pub bombs: WeaponStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponStats {
    pub damage: f32,
    pub fire_rate: f32,
    pub projectile_speed: f32,
    pub cooldown: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawningConfig {
    pub enemy_spawn_interval: f32,
    pub initial_delay: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationsConfig {
    pub v_shape_min: usize,
    pub v_shape_optimal: usize,
    pub line_min: usize,
    pub line_optimal: usize,
    pub circle_min: usize,
    pub circle_optimal: usize,
    pub diamond_min: usize,
    pub diamond_optimal: usize,
    pub scattered_min: usize,
    pub scattered_optimal: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    pub god_mode: bool,
    pub infinite_energy: bool,
    pub instant_ghost_spawn: bool,
    pub show_hitboxes: bool,
    pub show_fps: bool,
}

impl GameConfig {
    /// Load config from file, or create default if missing
    pub fn load() -> Self {
        match fs::read_to_string("config.toml") {
            Ok(contents) => toml::from_str(&contents).unwrap_or_else(|e| {
                eprintln!("Failed to parse config.toml: {}", e);
                eprintln!("Using default configuration");
                Self::default()
            }),
            Err(_) => {
                eprintln!("config.toml not found, creating default...");
                let default = Self::default();
                let toml_string =
                    toml::to_string_pretty(&default).expect("Failed to serialize default config");
                fs::write("config.toml", toml_string).expect("Failed to write default config");
                default
            }
        }
    }

    /// Reload config (useful for hot-reloading during development)
    pub fn reload(&mut self) {
        *self = Self::load();
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            window: WindowConfig {
                title: "Ghost Ally Shooter".to_string(),
                width: 800,
                height: 600,
                target_fps: 60,
            },
            player: PlayerConfig {
                starting_health: 100.0,
                max_health: 100.0,
                starting_energy: 100.0,
                max_energy: 100.0,
                movement_speed: 200.0,
                starting_weapons: vec!["Bullet".to_string(), "Laser".to_string()],
            },
            energy: EnergyConfig {
                regen_rate_idle: 30.0,
                regen_rate_active: 10.0,
            },
            entities: EntitiesConfig {
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
            },
            weapons: WeaponsConfig {
                bullet: WeaponStats {
                    damage: 10.0,
                    fire_rate: 0.3,
                    projectile_speed: 400.0,
                    cooldown: 3.0,
                },
                laser: WeaponStats {
                    damage: 60.0,
                    fire_rate: 2.5,
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
            },
            spawning: SpawningConfig {
                enemy_spawn_interval: 2.0,
                initial_delay: 3.0,
            },
            formations: FormationsConfig {
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
            },
            debug: DebugConfig {
                god_mode: false,
                infinite_energy: false,
                instant_ghost_spawn: false,
                show_hitboxes: false,
                show_fps: true,
            },
        }
    }
}
