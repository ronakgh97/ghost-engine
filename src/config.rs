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
    pub collision: CollisionConfig,
    pub ghost_behavior: GhostBehaviorConfig,
    pub enemy_behavior: EnemyBehaviorConfig,
    pub formation_spacing: FormationSpacingConfig,
    pub projectile_bounds: ProjectileBoundsConfig,
    pub screen_shake: ScreenShakeConfig,
    pub particles: ParticleConfig,
    pub background: BackgroundConfig,
    pub animations: AnimationConfig, // Animation system config
    pub dash: DashConfig,            // Dash mechanic config
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

    // Parry system
    pub parry_cooldown: f32,
    pub parry_window: f32,
    pub parry_energy_cost: f32,
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
    pub elite: EntityStats,
    pub healer: HealerStats,     // Special stats for healer enemy
    pub splitter: SplitterStats, // Special stats for splitter enemy
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityStats {
    pub health: f32,
    pub damage: f32,
    pub energy_cost: f32,
    pub fire_interval: f32,
    pub weapons: Vec<String>, // Weapon types this entity can use
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealerStats {
    pub health: f32,
    pub damage: f32,
    pub energy_cost: f32,
    pub fire_interval: f32,
    pub weapons: Vec<String>,
    // Healer-specific fields
    pub heal_rate: f32,   // HP healed per second
    pub heal_radius: f32, // Range of healing field (pixels)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitterStats {
    pub health: f32,
    pub damage: f32,
    pub energy_cost: f32,
    pub fire_interval: f32,
    pub weapons: Vec<String>,
    // Splitter-specific fields
    pub split_count: usize,          // How many splits to spawn (2-3)
    pub split_health_ratio: f32,     // HP ratio for each split (0.3 = 30%)
    pub split_speed_multiplier: f32, // Speed boost for splits (1.5 = 50% faster)
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
    pub damage: f32,           // Base damage per projectile
    pub fire_rate: f32,        // Cooldown between shots (in seconds)
    pub projectile_speed: f32, // How fast projectiles travel (pixels/sec)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawningConfig {
    pub wave_mode: bool,   // true = Lua wave system, false = classic random spawning
    pub wave_count: usize, // Number of waves (if wave_mode = true)
    pub enemy_spawn_interval: f32, // Random spawn timer (if wave_mode = false)
    pub initial_delay: f32, // Delay before first spawn (random mode)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationsConfig {
    pub v_shape_min: usize,
    pub v_shape_optimal: usize,
    pub line_min: usize,
    pub line_optimal: usize,
    pub circle_min: usize,
    pub circle_optimal: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    pub show_hitboxes: bool,
    pub show_fps: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollisionConfig {
    pub projectile_radius: f32,
    pub enemy_radius: f32,
    pub player_radius: f32,
    pub ghost_radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostBehaviorConfig {
    pub fire_interval: f32,
    pub movement_threshold_y: f32,
    pub fast_ascent_speed: f32,
    pub slow_hover_speed: f32,
    pub projectile_speed: f32,
    pub screen_boundary_top: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyBehaviorConfig {
    pub movement_threshold_y: f32,
    pub fast_descent_speed: f32,
    pub slow_hover_speed: f32,
    pub fire_threshold_y: f32,
    pub screen_boundary_bottom: f32,
    pub basic_projectile_speed_y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationSpacingConfig {
    pub v_shape_spacing: f32,
    pub v_shape_vertical_factor: f32,
    pub line_spacing: f32,
    pub line_height_offset: f32,
    pub circle_radius: f32,
    pub screen_edge_padding: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectileBoundsConfig {
    pub off_screen_padding: f32,
    pub player_projectile_speed_y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenShakeConfig {
    // Weapon-specific hit shake (when player/ghost hits enemy)
    pub bullet_hit_intensity: f32,
    pub laser_hit_intensity: f32,
    pub missile_hit_intensity: f32,
    pub plasma_hit_intensity: f32,
    pub bomb_hit_intensity: f32,
    pub weapon_hit_duration: f32, // Duration for all weapon hits

    // Event-specific shake
    pub enemy_death_duration: f32,
    pub enemy_death_intensity: f32,
    pub parry_duration: f32,
    pub parry_intensity: f32,
    pub player_hit_duration: f32,
    pub player_hit_intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleConfig {
    // Explosion particles (radial burst)
    pub explosion_count_min: usize,
    pub explosion_count_max: usize,
    pub explosion_lifetime_min: f32,
    pub explosion_lifetime_max: f32,
    pub explosion_size_min: f32,
    pub explosion_size_max: f32,
    pub explosion_speed_min: f32,
    pub explosion_speed_max: f32,

    // Hit sparks (directional)
    pub spark_count: usize,
    pub spark_lifetime_min: f32,
    pub spark_lifetime_max: f32,
    pub spark_size_min: f32,
    pub spark_size_max: f32,
    pub spark_speed_min: f32,
    pub spark_speed_max: f32,

    // Weapon-specific particle counts
    pub bullet_particle_count: usize,
    pub laser_particle_count: usize,
    pub missile_particle_count: usize,
    pub plasma_particle_count: usize,
    pub bomb_red_particle_count: usize,
    pub bomb_orange_particle_count: usize,

    // Death explosion counts
    pub death_red_count: usize,
    pub death_orange_count: usize,
    pub death_yellow_count: usize,

    // Parry effect counts
    pub parry_blue_count: usize,
    pub parry_white_count: usize,

    // Physics
    pub friction: f32,   // Velocity multiplier per frame (0.95 = 5% slowdown)
    pub size_decay: f32, // Pixels per second

    // Projectile trails
    pub trails_enabled: bool,      // Toggle trails on/off
    pub trail_spawn_interval: f32, // Time between trail particle spawns (seconds)
    pub trail_lifetime: f32,       // How long trail particles last
    pub trail_size: f32,           // Size of trail particles
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundConfig {
    pub scroll_speed: f32, // Pixels per second
}

impl GameConfig {
    /// Load config with smart fallback strategy (Hot-reload on R key)
    #[allow(dead_code)] // Reserved for future config loading strategies
    pub fn load() -> Self {
        // Try to load from file
        match Self::try_load_from_file() {
            Ok(config) => {
                println!("✓ Loaded config from config.toml");
                config
            }
            Err(e) => {
                println!("✗  Could not load config.toml: {e}");
                println!("✓ Using compiled-in defaults");
                default_config()
            }
        }
    }

    /// Try to load from file (returns error if missing/invalid)
    /// Made public for hot-reload error handling in main loop
    pub fn try_load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string("config.toml")?;
        let config: GameConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Create default config file for mods/testers
    #[allow(dead_code)] // Utility for generating config.toml template
    pub fn create_template() -> std::io::Result<()> {
        let default = default_config();
        let toml_string = toml::to_string_pretty(&default).expect("Failed to serialize config");

        fs::write("config.toml", toml_string)?;
        println!("✓ Created config.toml template");
        Ok(())
    }

    /// Reload config (hot-reload for development)
    #[allow(dead_code)] // Alternative reload method (main.rs uses try_load_from_file)
    pub fn reload(&mut self) {
        *self = Self::load();
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        default_config()
    }
}

// Animation system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    // Ghost spawn animation
    pub ghost_spawn_duration: f32,      // Total spawn animation time
    pub ghost_spawn_scale_start: f32,   // Starting scale (0.3 = 30% size)
    pub ghost_spawn_rotation_speed: f32,// Rotation while spawning (rad/s)
    
    // Ghost despawn animation
    pub ghost_despawn_duration: f32,    // Total despawn animation time
    pub ghost_despawn_rotation_speed: f32, // Spin speed while despawning (rad/s)
    
    // Hit flash effect (when taking damage)
    pub hit_flash_duration: f32,        // How long the flash lasts (seconds)
    pub hit_flash_intensity: f32,       // How much white to blend (0.0-1.0)
    
    // Parry animations
    pub parry_stance_glow_intensity: f32, // Glow brightness while parry active (0.0-1.0)
    pub parry_stance_pulse_speed: f32,    // How fast stance glow pulses (Hz)
    pub parry_stance_glow_duration: f32,  // How long stance glow lasts (seconds)
    pub parry_success_glow_burst: f32,    // Glow intensity multiplier on success (e.g., 2.0 = 200%)
    pub parry_success_duration: f32,      // Elastic bounce duration on success
    pub parry_success_scale_max: f32,     // Max scale during success bounce (1.3 = 130%)
    pub parry_failed_duration: f32,       // Shrink duration on missed parry
    pub parry_failed_scale_min: f32,      // Min scale during failed shrink (0.85 = 85%)
}

// Dash mechanic configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashConfig {
    pub enabled: bool,               // Master toggle for dash mechanic
    pub distance: f32,               // How far the dash travels (pixels)
    pub duration: f32,               // How long the dash movement lasts (seconds)
    pub i_frame_duration: f32,       // Invincibility window (seconds)
    pub energy_cost: f32,            // Energy consumed per dash
    pub cooldown: f32,               // Minimum time between dashes (seconds)
    
    // Visual effects
    pub trail_particle_count: i32,   // Particles spawned during dash trail
    pub trail_spawn_rate: f32,       // Trails per second during dash
    pub glow_intensity: f32,         // Blue glow intensity during dash (0.0-1.0)
    pub cooldown_ring_thickness: f32,// Thickness of cooldown indicator ring
}
