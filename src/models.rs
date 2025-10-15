use crate::config::GameConfig;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy)]
pub struct Stats {
    pub health: f32,
    pub max_health: f32,
    pub damage: f32,
}

pub struct WeaponStats {
    pub damage: f32,           // Base damage per projectile
    pub fire_rate: f32,        // Cooldown between shots (in seconds)
    pub projectile_speed: f32, // How fast projectiles travel (pixels/sec)
}

pub struct Projectile {
    pub pos: Position,
    pub velocity: Position,
    pub damage: f32,
    pub weapon_type: WeaponType, // Determines behavior (piercing, homing, etc)
    pub owner: ProjectileOwner,  // To differentiate between player, ghost and enemy projectiles

    // Weapon-specific behavior flags
    pub piercing: bool,        // Laser: doesn't despawn on hit
    pub homing: bool,          // Missile: tracks nearest enemy
    pub explosion_radius: f32, // Bombs: AOE damage on impact (0.0 = no explosion)

    // Homing missile data
    pub locked_target_index: Option<usize>, // Which enemy index is locked (None = find new target)
    pub lifetime: f32,                      // How long projectile has existed (for cleanup)
    pub trail_timer: f32,                   // Timer for spawning trail particles
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: Position,
    pub velocity: Position,
    pub lifetime: f32,     // Time until particle disappears
    pub max_lifetime: f32, // For fade-out calculation
    pub color: macroquad::prelude::Color,
    pub size: f32,       // Initial size
    pub size_decay: f32, // How much size shrinks per second
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProjectileOwner {
    Player,
    Enemy,
    Ghost,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WeaponType {
    Bullet,
    Laser,
    Missile,
    Plasma, // Not implemented yet
    Bombs,  // Not implemented yet
}

impl WeaponType {
    /// Parse weapon type from string
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "Bullet" => Some(WeaponType::Bullet),
            "Laser" => Some(WeaponType::Laser),
            "Missile" => Some(WeaponType::Missile),
            "Plasma" => Some(WeaponType::Plasma),
            "Bombs" => Some(WeaponType::Bombs),
            _ => None,
        }
    }

    /// Get weapon stats from config
    pub fn get_weapon_stats(&self, config: &crate::config::WeaponsConfig) -> WeaponStats {
        let weapon_cfg = match self {
            WeaponType::Bullet => &config.bullet,
            WeaponType::Laser => &config.laser,
            WeaponType::Missile => &config.missile,
            WeaponType::Plasma => &config.plasma,
            WeaponType::Bombs => &config.bombs,
        };

        WeaponStats {
            damage: weapon_cfg.damage,
            fire_rate: weapon_cfg.fire_rate,
            projectile_speed: weapon_cfg.projectile_speed,
        }
    }

    pub fn _get_damage_value(&self, config: &crate::config::WeaponsConfig) -> f32 {
        self.get_weapon_stats(config).damage
    }

    pub fn _get_fire_rate(&self, config: &crate::config::WeaponsConfig) -> f32 {
        self.get_weapon_stats(config).fire_rate
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityType {
    BasicFighter,
    Sniper,
    Tank,
    Elite,
    Healer,
    Splitter, // Splits into 2-3 smaller versions when killed
}

impl EntityType {
    /// Get stats from config
    pub fn get_stats(&self, config: &crate::config::EntitiesConfig) -> Stats {
        match self {
            EntityType::BasicFighter => Stats {
                health: config.basic_fighter.health,
                max_health: config.basic_fighter.health,
                damage: config.basic_fighter.damage,
            },
            EntityType::Sniper => Stats {
                health: config.sniper.health,
                max_health: config.sniper.health,
                damage: config.sniper.damage,
            },
            EntityType::Tank => Stats {
                health: config.tank.health,
                max_health: config.tank.health,
                damage: config.tank.damage,
            },
            EntityType::Elite => Stats {
                health: config.elite.health,
                max_health: config.elite.health,
                damage: config.elite.damage,
            },
            EntityType::Healer => Stats {
                health: config.healer.health,
                max_health: config.healer.health,
                damage: config.healer.damage,
            },
            EntityType::Splitter => Stats {
                health: config.splitter.health,
                max_health: config.splitter.health,
                damage: config.splitter.damage,
            },
        }
    }

    pub fn get_energy_cost(&self, config: &crate::config::EntitiesConfig) -> f32 {
        match self {
            EntityType::BasicFighter => config.basic_fighter.energy_cost,
            EntityType::Sniper => config.sniper.energy_cost,
            EntityType::Tank => config.tank.energy_cost,
            EntityType::Elite => config.elite.energy_cost,
            EntityType::Healer => config.healer.energy_cost,
            EntityType::Splitter => config.splitter.energy_cost,
        }
    }

    pub fn get_fire_interval(&self, config: &crate::config::EntitiesConfig) -> f32 {
        match self {
            EntityType::BasicFighter => config.basic_fighter.fire_interval,
            EntityType::Sniper => config.sniper.fire_interval,
            EntityType::Tank => config.tank.fire_interval,
            EntityType::Elite => config.elite.fire_interval,
            EntityType::Healer => config.healer.fire_interval,
            EntityType::Splitter => config.splitter.fire_interval,
        }
    }
}

// Player
pub struct Player {
    pub pos: Position,
    pub stats: Stats,
    pub energy: f32,
    pub max_energy: f32,
    pub weapon: Vec<WeaponType>,
    pub available_ghosts: Vec<EntityType>, // Queue of defeated enemies (can summon as ghosts)

    // Parry system
    pub parry_cooldown: f32, // Time until parry available again
    pub parry_window: f32,   // How long parry is active (0.2 seconds)
    pub parry_active: bool,  // Currently in parry stance
    
    // Parry animations
    pub parry_success_scale_timer: f32, // Elastic bounce on successful deflection
    pub parry_failed_timer: f32,        // Shrink/desaturation on missed parry
}

// Enemy
#[derive(Clone)] // Needed for splitter system
pub struct Enemy {
    pub pos: Position,
    pub stats: Stats,
    pub weapon: Vec<WeaponType>,
    pub entity_type: EntityType,
    pub anim: EntityAnimState, // Animation state (hit flash, spawn, despawn)
}

// Animation state for entities (ghosts, enemies, etc.)
#[derive(Clone)]
pub struct EntityAnimState {
    // Lifecycle timers
    pub spawn_timer: f32,     // Fade in on spawn
    pub despawn_timer: f32,   // Fade out on despawn
    pub is_spawning: bool,    // Currently playing spawn animation
    pub is_despawning: bool,  // Currently playing despawn animation
    
    // Visual modifiers (applied during rendering)
    pub scale: f32,           // Size multiplier (1.0 = normal)
    pub rotation: f32,        // Rotation in radians
    pub alpha: f32,           // Opacity (0.0 - 1.0)
    
    // Hit feedback
    pub hit_flash_timer: f32, // White flash when taking damage (counts down from duration)
}

impl Default for EntityAnimState {
    fn default() -> Self {
        Self {
            spawn_timer: 0.0,
            despawn_timer: 0.0,
            is_spawning: false,
            is_despawning: false,
            scale: 1.0,
            rotation: 0.0,
            alpha: 1.0,
            hit_flash_timer: 0.0,
        }
    }
}

impl EntityAnimState {
    /// Create new animation state for spawning entity
    pub fn new_spawning(duration: f32) -> Self {
        Self {
            spawn_timer: duration,
            despawn_timer: 0.0,
            is_spawning: true,
            is_despawning: false,
            scale: 0.3,     // Start small
            rotation: 0.0,
            alpha: 0.0,     // Start invisible
            hit_flash_timer: 0.0,
        }
    }
    
    /// Trigger despawn animation
    pub fn start_despawn(&mut self, duration: f32) {
        self.is_despawning = true;
        self.despawn_timer = duration;
    }
}

// Ghost
#[derive(Clone)]
pub struct Ghost {
    pub pos: Position,
    pub stats: Stats,
    pub weapon_type: Vec<WeaponType>,
    pub entity_type: EntityType,
    pub energy_drain_per_sec: f32,
    pub anim: EntityAnimState,  // Animation state
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GhostFormation {
    VShape, // Classic V formation (good for spread coverage)
    Line,   // Horizontal line (maximum firepower forward)
    Circle, // Circle around player (defensive)
}

impl GhostFormation {
    /// Get minimum ghost count required for this formation
    pub fn min_ghost_count(&self) -> usize {
        match self {
            GhostFormation::VShape => 2, // At least 2 to make a V
            GhostFormation::Line => 3,   // At least 3 for a line
            GhostFormation::Circle => 4, // At least 4 for circular shape
        }
    }

    /// Get optimal ghost count for this formation
    pub fn optimal_ghost_count(&self) -> usize {
        match self {
            GhostFormation::VShape => 6, // 3 on each side
            GhostFormation::Line => 5,   // Nice spread
            GhostFormation::Circle => 8, // Perfect circle
        }
    }

    /// Check if this formation can be used with given ghost count
    pub fn is_valid_for_count(&self, ghost_count: usize) -> bool {
        ghost_count >= self.min_ghost_count()
    }
}

impl Ghost {
    /// Create ghost directly from EntityType
    pub fn from_entity_type(
        entity_type: EntityType,
        spawn_pos: Position,
        config: &crate::config::GameConfig,
    ) -> Self {
        // Get entity config (handle healer's special struct)
        let (weapons_vec, base_stats) = match entity_type {
            EntityType::BasicFighter => (
                &config.entities.basic_fighter.weapons,
                Stats {
                    health: config.entities.basic_fighter.health,
                    max_health: config.entities.basic_fighter.health,
                    damage: config.entities.basic_fighter.damage,
                },
            ),
            EntityType::Sniper => (
                &config.entities.sniper.weapons,
                Stats {
                    health: config.entities.sniper.health,
                    max_health: config.entities.sniper.health,
                    damage: config.entities.sniper.damage,
                },
            ),
            EntityType::Tank => (
                &config.entities.tank.weapons,
                Stats {
                    health: config.entities.tank.health,
                    max_health: config.entities.tank.health,
                    damage: config.entities.tank.damage,
                },
            ),
            EntityType::Elite => (
                &config.entities.elite.weapons,
                Stats {
                    health: config.entities.elite.health,
                    max_health: config.entities.elite.health,
                    damage: config.entities.elite.damage,
                },
            ),
            EntityType::Healer => (
                &config.entities.healer.weapons,
                Stats {
                    health: config.entities.healer.health,
                    max_health: config.entities.healer.health,
                    damage: config.entities.healer.damage,
                },
            ),
            EntityType::Splitter => (
                &config.entities.splitter.weapons,
                Stats {
                    health: config.entities.splitter.health,
                    max_health: config.entities.splitter.health,
                    damage: config.entities.splitter.damage,
                },
            ),
        };

        // Parse weapons from config (inherit from entity type!)
        let weapons: Vec<WeaponType> = weapons_vec
            .iter()
            .filter_map(|w| WeaponType::from_string(w))
            .collect();

        Ghost {
            pos: spawn_pos,
            stats: base_stats,
            weapon_type: if weapons.is_empty() {
                vec![WeaponType::Bullet] // Fallback only if config invalid
            } else {
                weapons // Uses entity's configured weapons!
            },
            entity_type,
            energy_drain_per_sec: entity_type.get_energy_cost(&config.entities) * 0.1,
            anim: EntityAnimState::new_spawning(0.5), // 0.5s spawn animation
        }
    }
}

/// Wave state machine
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WaveState {
    Ready,      // Wave loaded, waiting for prep countdown
    Preparing,  // Countdown active
    Active,     // Enemies spawning
    Complete,   // All enemies defeated
    Transition, // Brief pause before next wave
}

/// Individual spawn group within a wave
#[derive(Debug, Clone)]
pub struct WaveSpawn {
    pub enemy_type: EntityType,
    pub count: usize,
    pub interval: f32, // Seconds between each enemy spawn
    #[allow(dead_code)] // Used in from_lua() initialization
    pub delay: f32, // Delay before starting this spawn group (copied to timer)
    pub spawned: usize, // How many have been spawned so far
    pub timer: f32,    // Time until next spawn (initialized from delay)
}

/// Wave definition (converted from Lua)
pub struct WaveDefinition {
    pub wave_number: usize,
    pub name: String,
    pub prep_time: f32,
    pub spawns: Vec<WaveSpawn>,
}

impl WaveDefinition {
    /// Convert from Lua wave definition
    pub fn from_lua(
        lua_wave: crate::scripting::LuaWaveDefinition,
        _config: &crate::config::GameConfig, // Reserved for future difficulty scaling
    ) -> Option<Self> {
        let mut spawns = Vec::new();

        for lua_spawn in lua_wave.spawns {
            // Parse entity type from string
            let entity_type = match lua_spawn.enemy_type.as_str() {
                "BasicFighter" => EntityType::BasicFighter,
                "Sniper" => EntityType::Sniper,
                "Tank" => EntityType::Tank,
                "Elite" => EntityType::Elite,
                "Healer" => EntityType::Healer,
                "Splitter" => EntityType::Splitter,
                unknown => {
                    println!(
                        "✗ Unknown enemy type in wave {}: {}",
                        lua_wave.wave_number, unknown
                    );
                    continue; // Skip invalid enemy types
                }
            };

            spawns.push(WaveSpawn {
                enemy_type: entity_type,
                count: lua_spawn.count,
                interval: lua_spawn.interval,
                delay: lua_spawn.delay,
                spawned: 0,
                timer: lua_spawn.delay, // Start with delay
            });
        }

        if spawns.is_empty() {
            println!("✗ Wave {} has no valid spawns!", lua_wave.wave_number);
            return None;
        }

        Some(WaveDefinition {
            wave_number: lua_wave.wave_number,
            name: lua_wave.name,
            prep_time: lua_wave.prep_time,
            spawns,
        })
    }

    /// Total enemies in this wave (used by wave UI)
    #[allow(dead_code)] // UI Helper
    pub fn total_enemy_count(&self) -> usize {
        self.spawns.iter().map(|s| s.count).sum()
    }
}

// GameState
pub struct GameState {
    pub config: crate::config::GameConfig,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub ghosts: Vec<Ghost>,
    pub projectiles: Vec<Projectile>,
    pub particles: Vec<Particle>,
    pub player_fire_timer: f32,
    pub enemy_fire_timers: Vec<f32>,
    pub ghost_fire_timers: Vec<f32>,
    pub spawn_timer: f32,
    pub ghost_formation: GhostFormation,

    // Screen shake
    pub screen_shake_duration: f32,
    pub screen_shake_intensity: f32,

    // Background scroll
    pub bg_scroll_offset: f32,

    // Wave system
    pub wave_manager: crate::game::wave::WaveManager,
}

impl GameState {
    pub fn new() -> Self {
        use macroquad::prelude::*;

        // ALWAYS start with compiled defaults - never try to load config.toml at startup
        let config = crate::defaults::default_config();

        GameState {
            config: config.clone(),
            player: Player {
                pos: Position {
                    x: screen_width() / 2.0,
                    y: screen_height() - 50.0,
                },
                stats: Stats {
                    health: config.player.starting_health,
                    max_health: config.player.max_health,
                    damage: 20.0,
                },
                weapon: vec![
                    WeaponType::Bullet,
                    WeaponType::Laser,
                    WeaponType::Missile,
                    WeaponType::Plasma,
                ],
                energy: config.player.starting_energy,
                max_energy: config.player.max_energy,
                available_ghosts: Vec::new(),

                // Parry system initialized
                parry_cooldown: 0.0,
                parry_window: 0.0,
                parry_active: false,
                
                // Parry animations initialized
                parry_success_scale_timer: 0.0,
                parry_failed_timer: 0.0,
            },
            enemies: Vec::new(),
            ghosts: Vec::new(),
            projectiles: Vec::new(),
            particles: Vec::new(),
            player_fire_timer: 0.0,
            enemy_fire_timers: Vec::new(),
            ghost_fire_timers: Vec::new(),
            spawn_timer: 0.0,
            ghost_formation: GhostFormation::Line,

            // Screen shake
            screen_shake_duration: 0.0,
            screen_shake_intensity: 0.0,

            // Starts at bottom of texture
            bg_scroll_offset: 0.0,

            // Wave system - initialized based on config
            wave_manager: {
                let cfg = crate::defaults::default_config();
                if cfg.spawning.wave_mode {
                    println!(
                        "✓ Initializing Lua wave system ({} waves)",
                        cfg.spawning.wave_count
                    );
                } else {
                    println!("✓ Wave system ready (classic random mode)");
                }
                crate::game::wave::WaveManager::new(cfg.spawning.wave_count)
            },
        }
    }
    pub fn apply_config(&mut self, config: &GameConfig) {
        self.config = config.clone();
        self.player.stats.max_health = config.player.max_health;
        self.player.max_energy = config.player.max_energy;
    }
}
