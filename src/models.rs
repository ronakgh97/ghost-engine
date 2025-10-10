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
    pub cooldown: f32,
}

pub struct Projectile {
    pub pos: Position,
    pub velocity: Position,
    pub damage: f32,
    pub weapon_type: WeaponType,
    pub owner: ProjectileOwner, // To differentiate between player, ghost and enemy projectiles
}

#[derive(Clone, Copy, Debug)]
pub enum ProjectileOwner {
    Player,
    Enemy,
    Ghost,
}

#[derive(Clone, Copy, Debug)]
pub enum WeaponType {
    Bullet,
    Laser,
    Missile,
    Plasma,
    Bombs,
}

impl WeaponType {
    pub fn get_damage(&self) -> WeaponStats {
        match self {
            WeaponType::Bullet => WeaponStats {
                damage: 10.0,
                fire_rate: 0.3,
                cooldown: 3.0,
            },
            WeaponType::Laser => WeaponStats {
                damage: 60.0,
                fire_rate: 2.5,
                cooldown: 3.0,
            },
            WeaponType::Missile => WeaponStats {
                damage: 30.0,
                fire_rate: 1.0,
                cooldown: 3.0,
            },
            WeaponType::Plasma => WeaponStats {
                damage: 40.0,
                fire_rate: 1.5,
                cooldown: 3.0,
            },
            WeaponType::Bombs => WeaponStats {
                damage: 50.0,
                fire_rate: 2.0,
                cooldown: 3.0,
            },
        }
    }

    pub fn get_fire_rate(&self) -> f32 {
        match self {
            WeaponType::Bullet => 0.3,
            WeaponType::Laser => 2.5,
            WeaponType::Missile => 1.0,
            WeaponType::Plasma => 1.5,
            WeaponType::Bombs => 2.0,
        }
    }

    pub fn get_damage_value(&self) -> f32 {
        match self {
            WeaponType::Bullet => 10.0,
            WeaponType::Laser => 60.0,
            WeaponType::Missile => 30.0,
            WeaponType::Plasma => 40.0,
            WeaponType::Bombs => 50.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum EntityType {
    BasicFighter,
    Sniper,
    Tank,
    Boss,
}

impl EntityType {
    pub fn get_stats(&self) -> Stats {
        match self {
            EntityType::BasicFighter => Stats {
                health: 50.0,
                max_health: 50.0,
                damage: 10.0,
            },
            EntityType::Sniper => Stats {
                health: 30.0,
                max_health: 30.0,
                damage: 25.0,
            },
            EntityType::Tank => Stats {
                health: 150.0,
                max_health: 150.0,
                damage: 15.0,
            },
            EntityType::Boss => Stats {
                health: 500.0,
                max_health: 500.0,
                damage: 30.0,
            },
        }
    }

    pub fn get_energy_cost(&self) -> f32 {
        match self {
            EntityType::BasicFighter => 15.0,
            EntityType::Sniper => 25.0,
            EntityType::Tank => 40.0,
            EntityType::Boss => 80.0,
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

impl Ghost {
    pub fn from_enemy(enemy: &Enemy) -> Self {
        Ghost {
            pos: enemy.pos,
            stats: enemy.stats,
            weapon_type: enemy.weapon.clone(),
            entity_type: enemy.entity_type,
            energy_drain_per_sec: enemy.entity_type.get_energy_cost() * 0.1,
        }
    }
}

// GameState
pub struct GameState {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub ghosts: Vec<Ghost>,
    pub projectiles: Vec<Projectile>,
}

impl GameState {
    pub fn new() -> Self {
        use macroquad::prelude::*;

        GameState {
            player: Player {
                pos: Position {
                    x: screen_width() / 2.0,
                    y: screen_height() - 50.0,
                },
                stats: Stats {
                    health: 100.0,
                    max_health: 100.0,
                    damage: 20.0,
                },
                weapon: vec![WeaponType::Bullet, WeaponType::Laser],
                energy: 100.0,
                max_energy: 100.0,
                available_ghosts: Vec::new(),
            },
            enemies: Vec::new(),
            ghosts: Vec::new(),
            projectiles: Vec::new(),
        }
    }
}
