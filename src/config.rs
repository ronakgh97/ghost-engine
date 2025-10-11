use crate::defaults::default_config;
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
    pub show_hitboxes: bool,
    pub show_fps: bool,
}

impl GameConfig {
    /// Load config with smart fallback strategy
    pub fn load() -> Self {
        // Try to load from file
        match Self::load_from_file() {
            Ok(config) => {
                println!("✅ Loaded config from config.toml");
                config
            }
            Err(e) => {
                println!("⚠️  Could not load config.toml: {}", e);
                println!("✅ Using compiled-in defaults");
                default_config()
            }
        }
    }

    /// Try to load from file (returns error if missing/invalid)
    fn load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string("config.toml")?;
        let config: GameConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Create default config file for modders/testers
    pub fn create_template() -> std::io::Result<()> {
        let default = default_config();
        let toml_string = toml::to_string_pretty(&default).expect("Failed to serialize config");

        fs::write("config.toml", toml_string)?;
        println!("✅ Created config.toml template");
        Ok(())
    }

    /// Reload config (hot-reload for development)
    pub fn reload(&mut self) {
        *self = Self::load();
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        default_config()
    }
}
