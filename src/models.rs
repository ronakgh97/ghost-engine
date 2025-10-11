use crate::config::GameConfig;
use macroquad::logging::warn;

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
    pub damage: f32,
    pub fire_rate: f32, // Shots per second
    pub ammo: f32,
    pub cooldown: f32,  // Not Used currently
    pub projectile_speed: f32
}

pub struct Projectile {
    pub pos: Position,
    pub velocity: Position,
    pub damage: f32,
    pub weapon_type: WeaponType, // Not used currently
    pub owner: ProjectileOwner,  // To differentiate between player, ghost and enemy projectiles
}

#[derive(Clone, Copy, Debug)]
pub enum ProjectileOwner {
    Player,
    Enemy,
    Ghost,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum WeaponType {
    Bullet,
    Laser,
    Missile, // Not implemented yet
    Plasma,  // Not implemented yet
    Bombs,   // Not implemented yet
}

impl WeaponType {
    /// Get weapon stats from config (no more hardcoded values!)
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
            cooldown: weapon_cfg.cooldown,
            ammo: weapon_cfg.ammo,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityType {
    BasicFighter,
    Sniper,
    Tank,
    Boss,
}

impl EntityType {
    /// Get stats from config (no more hardcoded values!)
    pub fn get_stats(&self, config: &crate::config::EntitiesConfig) -> Stats {
        let entity_stats = match self {
            EntityType::BasicFighter => &config.basic_fighter,
            EntityType::Sniper => &config.sniper,
            EntityType::Tank => &config.tank,
            EntityType::Boss => &config.boss,
        };

        Stats {
            health: entity_stats.health,
            max_health: entity_stats.health,
            damage: entity_stats.damage,
        }
    }

    pub fn get_energy_cost(&self, config: &crate::config::EntitiesConfig) -> f32 {
        match self {
            EntityType::BasicFighter => config.basic_fighter.energy_cost,
            EntityType::Sniper => config.sniper.energy_cost,
            EntityType::Tank => config.tank.energy_cost,
            EntityType::Boss => config.boss.energy_cost,
        }
    }

    pub fn get_fire_interval(&self, config: &crate::config::EntitiesConfig) -> f32 {
        match self {
            EntityType::BasicFighter => config.basic_fighter.fire_interval,
            EntityType::Sniper => config.sniper.fire_interval,
            EntityType::Tank => config.tank.fire_interval,
            EntityType::Boss => config.boss.fire_interval,
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
    pub available_ghosts: Vec<EntityType>,
}

// Enemy
pub struct Enemy {
    pub pos: Position,
    pub stats: Stats,
    pub weapon: Vec<WeaponType>,
    pub entity_type: EntityType,
}

// Ghost
pub struct Ghost {
    pub pos: Position,
    pub stats: Stats,
    pub weapon_type: Vec<WeaponType>,
    pub entity_type: EntityType,
    pub energy_drain_per_sec: f32,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GhostFormation {
    VShape,    // Classic V formation (good for spread coverage)
    Line,      // Horizontal line (maximum firepower forward)
    Circle,    // Circle around player (defensive)
    Scattered, // Random positions (chaotic but cool)
}

impl GhostFormation {
    /// Get minimum ghost count required for this formation
    pub fn min_ghost_count(&self) -> usize {
        match self {
            GhostFormation::VShape => 2,    // At least 2 to make a V
            GhostFormation::Line => 3,      // At least 3 for a line
            GhostFormation::Circle => 4,    // At least 4 for circular shape
            GhostFormation::Scattered => 3, // Can scatter even 1 ghost
        }
    }

    /// Get optimal ghost count for this formation
    pub fn optimal_ghost_count(&self) -> usize {
        match self {
            GhostFormation::VShape => 6,    // 3 on each side
            GhostFormation::Line => 5,      // Nice spread
            GhostFormation::Circle => 8,    // Perfect circle
            GhostFormation::Scattered => 6, // Not too chaotic
        }
    }

    /// Check if this formation can be used with given ghost count
    pub fn is_valid_for_count(&self, ghost_count: usize) -> bool {
        ghost_count >= self.min_ghost_count()
    }
}

impl Ghost {
    pub fn from_enemy(enemy: &Enemy, entities_config: &crate::config::EntitiesConfig) -> Self {
        Ghost {
            pos: enemy.pos,
            stats: enemy.stats,
            weapon_type: enemy.weapon.clone(),
            entity_type: enemy.entity_type,
            energy_drain_per_sec: enemy.entity_type.get_energy_cost(entities_config) * 0.1,
        }
    }
}

// GameState
pub struct GameState {
    pub config: crate::config::GameConfig,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub ghosts: Vec<Ghost>,
    pub projectiles: Vec<Projectile>,
    pub player_fire_timer: f32,
    pub enemy_fire_timers: Vec<f32>,
    pub ghost_fire_timers: Vec<f32>,
    pub spawn_timer: f32,
    pub ghost_formation: GhostFormation,
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
                weapon: vec![WeaponType::Bullet, WeaponType::Laser],
                energy: config.player.starting_energy,
                max_energy: config.player.max_energy,
                available_ghosts: Vec::new(),
            },
            enemies: Vec::new(),
            ghosts: Vec::new(),
            projectiles: Vec::new(),
            player_fire_timer: 0.0,
            enemy_fire_timers: Vec::new(),
            ghost_fire_timers: Vec::new(),
            spawn_timer: 0.0,
            ghost_formation: GhostFormation::Scattered,
        }
    }
    pub fn apply_config(&mut self, config: &GameConfig) {
        self.config = config.clone();
        self.player.stats.max_health = config.player.max_health;
        self.player.max_energy = config.player.max_energy;
    }
}
