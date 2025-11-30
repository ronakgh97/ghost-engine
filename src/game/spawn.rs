use crate::game::utils::biased_random_x;
use crate::models::*;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

/// Create a Bezier entry path based on enemy type
pub fn create_wave_enemy_path(entity_type: EntityType, spawn_x: f32) -> EnemyMovementState {
    let screen_w = screen_width();
    let _screen_h = screen_height();

    match entity_type {
        EntityType::BasicFighter => {
            // Random variant
            let variant = gen_range(0, 3);
            let path = match variant {
                0 => {
                    // Gentle Curve
                    BezierPath {
                        p0: Vec2::new(spawn_x, gen_range(-30.0, -10.0)),
                        p1: Vec2::new(spawn_x + gen_range(-80.0, 80.0), gen_range(30.0, 45.0)),
                        p2: Vec2::new(spawn_x + gen_range(-40.0, 40.0), gen_range(90.0, 120.0)),
                        p3: Vec2::new(spawn_x, gen_range(100.0, 140.0)),
                        duration: gen_range(1.5, 2.0),
                        use_cubic: true,
                    }
                }
                1 => {
                    // Sharp Swoop
                    let swoop_dir = if gen_range(0, 2) == 0 { 1.0 } else { -1.0 };
                    BezierPath {
                        p0: Vec2::new(spawn_x + swoop_dir * 150.0, -30.0),
                        p1: Vec2::new(spawn_x + swoop_dir * 120.0, 30.0),
                        p2: Vec2::new(spawn_x - swoop_dir * 30.0, 90.0),
                        p3: Vec2::new(spawn_x, 130.0),
                        duration: gen_range(1.0, 1.5),
                        use_cubic: true,
                    }
                }
                _ => {
                    // Loop Entry
                    BezierPath {
                        p0: Vec2::new(spawn_x, -30.0),
                        p1: Vec2::new(spawn_x + 60.0, 20.0),
                        p2: Vec2::new(spawn_x - 60.0, 80.0),
                        p3: Vec2::new(spawn_x, 120.0),
                        duration: 1.5,
                        use_cubic: true,
                    }
                }
            };

            EnemyMovementState::FollowingPath {
                path,
                progress: 0.0,
                elapsed_time: 0.0,
            }
        }
        EntityType::Sniper => {
            // Snipers ALWAYS come from sides
            let side = gen_range(0, 2); // 0 = left, 1 = right
            let entry_style = gen_range(0, 2); // Different curve styles

            let path = if side == 0 {
                // From LEFT
                if entry_style == 0 {
                    // Fast sweep
                    BezierPath {
                        p0: Vec2::new(-50.0, gen_range(60.0, 120.0)),
                        p1: Vec2::new(80.0, gen_range(40.0, 80.0)),
                        p2: Vec2::new(140.0, gen_range(100.0, 140.0)),
                        p3: Vec2::new(110.0, gen_range(130.0, 160.0)),
                        duration: gen_range(1.5, 2.0),
                        use_cubic: true,
                    }
                } else {
                    // Curved approach
                    BezierPath {
                        p0: Vec2::new(-50.0, gen_range(40.0, 80.0)),
                        p1: Vec2::new(120.0, gen_range(20.0, 60.0)),
                        p2: Vec2::new(100.0, gen_range(120.0, 160.0)),
                        p3: Vec2::new(100.0, gen_range(140.0, 170.0)),
                        duration: gen_range(1.5, 2.5),
                        use_cubic: true,
                    }
                }
            } else {
                // From RIGHT
                if entry_style == 0 {
                    // Fast sweep
                    BezierPath {
                        p0: Vec2::new(screen_w + 50.0, gen_range(60.0, 120.0)),
                        p1: Vec2::new(screen_w - 80.0, gen_range(40.0, 80.0)),
                        p2: Vec2::new(screen_w - 140.0, gen_range(100.0, 140.0)),
                        p3: Vec2::new(screen_w - 110.0, gen_range(130.0, 160.0)),
                        duration: gen_range(1.5, 2.0),
                        use_cubic: true,
                    }
                } else {
                    // Curved approach
                    BezierPath {
                        p0: Vec2::new(screen_w + 50.0, gen_range(40.0, 80.0)),
                        p1: Vec2::new(screen_w - 120.0, gen_range(20.0, 60.0)),
                        p2: Vec2::new(screen_w - 100.0, gen_range(120.0, 160.0)),
                        p3: Vec2::new(screen_w - 100.0, gen_range(140.0, 170.0)),
                        duration: gen_range(1.5, 2.5),
                        use_cubic: true,
                    }
                }
            };

            EnemyMovementState::FollowingPath {
                path,
                progress: 0.0,
                elapsed_time: 0.0,
            }
        }
        EntityType::Tank => {
            // Tanks: Heavy dramatic entries
            let variant = gen_range(0, 2);
            let path = if variant == 0 {
                // Diagonal Slam - Top corner to opposite bottom
                let from_right = gen_range(0, 2) == 0;
                if from_right {
                    BezierPath {
                        p0: Vec2::new(screen_w + 50.0, -50.0),
                        p1: Vec2::new(screen_w * 0.7, 60.0),
                        p2: Vec2::new(screen_w * 0.3, 140.0),
                        p3: Vec2::new(gen_range(100.0, 200.0), gen_range(160.0, 200.0)),
                        duration: gen_range(2.0, 3.0),
                        use_cubic: true,
                    }
                } else {
                    BezierPath {
                        p0: Vec2::new(-50.0, -50.0),
                        p1: Vec2::new(screen_w * 0.3, 60.0),
                        p2: Vec2::new(screen_w * 0.7, 140.0),
                        p3: Vec2::new(
                            gen_range(screen_w - 200.0, screen_w - 100.0),
                            gen_range(160.0, 200.0),
                        ),
                        duration: gen_range(2.5, 3.0),
                        use_cubic: true,
                    }
                }
            } else {
                // Overhead Arc
                BezierPath {
                    p0: Vec2::new(gen_range(0.0, screen_w), -50.0),
                    p1: Vec2::new(screen_w / 2.0 + gen_range(-100.0, 100.0), 30.0),
                    p2: Vec2::new(spawn_x + gen_range(-80.0, 80.0), 120.0),
                    p3: Vec2::new(spawn_x, gen_range(150.0, 180.0)),
                    duration: gen_range(2.0, 2.5),
                    use_cubic: true,
                }
            };

            EnemyMovementState::FollowingPath {
                path,
                progress: 0.0,
                elapsed_time: 0.0,
            }
        }
        EntityType::Elite => {
            // Elite: Multiple dramatic entrance styles
            let variant = gen_range(0, 3);
            let path = match variant {
                0 => {
                    // Figure-8 Entry
                    BezierPath {
                        p0: Vec2::new(screen_w / 2.0, -50.0),
                        p1: Vec2::new(screen_w / 2.0 + 120.0, 40.0),
                        p2: Vec2::new(screen_w / 2.0 - 120.0, 120.0),
                        p3: Vec2::new(screen_w / 2.0, 100.0),
                        duration: 2.5,
                        use_cubic: true,
                    }
                }
                1 => {
                    // Spiral Entry
                    BezierPath {
                        p0: Vec2::new(screen_w / 2.0 + 150.0, -30.0),
                        p1: Vec2::new(screen_w / 2.0 + 100.0, 60.0),
                        p2: Vec2::new(screen_w / 2.0 - 80.0, 100.0),
                        p3: Vec2::new(screen_w / 2.0, 120.0),
                        duration: 2.0,
                        use_cubic: true,
                    }
                }
                _ => {
                    // Overhead Dive
                    BezierPath {
                        p0: Vec2::new(screen_w / 2.0, -80.0),
                        p1: Vec2::new(screen_w / 2.0 - 100.0, 20.0),
                        p2: Vec2::new(screen_w / 2.0 + 100.0, 80.0),
                        p3: Vec2::new(screen_w / 2.0, 110.0),
                        duration: 2.0,
                        use_cubic: true,
                    }
                }
            };

            EnemyMovementState::FollowingPath {
                path,
                progress: 0.0,
                elapsed_time: 0.0,
            }
        }
        EntityType::Healer | EntityType::Splitter => {
            // Support units: Gentle randomized entries
            let curve_strength = gen_range(30.0, 80.0);
            let curve_dir = if gen_range(0, 2) == 0 { 1.0 } else { -1.0 };

            let path = BezierPath {
                p0: Vec2::new(spawn_x, -30.0),
                p1: Vec2::new(spawn_x + curve_dir * curve_strength, gen_range(50.0, 90.0)),
                p2: Vec2::new(
                    spawn_x - curve_dir * (curve_strength * 0.5),
                    gen_range(100.0, 130.0),
                ),
                p3: Vec2::new(spawn_x, gen_range(120.0, 150.0)),
                duration: gen_range(1.0, 1.5),
                use_cubic: true,
            };
            EnemyMovementState::FollowingPath {
                path,
                progress: 0.0,
                elapsed_time: 0.0,
            }
        }
    }
}

/// Spawn random enemies
pub fn spawn_enemies(state: &mut GameState, delta: f32) {
    state.spawn_timer -= delta;

    if state.spawn_timer <= 0.0 {
        // Reset timer
        state.spawn_timer = state.config.spawning.enemy_spawn_interval;

        //  Spawn pool:
        let enemy_types = [
            EntityType::BasicFighter,
            EntityType::BasicFighter,
            EntityType::Splitter,
            EntityType::Splitter,
            EntityType::Sniper,
            EntityType::Sniper,
            EntityType::Tank,
            EntityType::Tank,
            EntityType::Elite,
        ];

        let random_idx = rand::gen_range(0, enemy_types.len());
        let entity_type = enemy_types[random_idx];

        // Get entity stats from config
        let entity_stats = entity_type.get_stats(&state.config.entities);

        // Get weapons list
        let weapons_list = match entity_type {
            EntityType::BasicFighter => &state.config.entities.basic_fighter.weapons,
            EntityType::Sniper => &state.config.entities.sniper.weapons,
            EntityType::Tank => &state.config.entities.tank.weapons,
            EntityType::Elite => &state.config.entities.elite.weapons,
            EntityType::Healer => &state.config.entities.healer.weapons,
            EntityType::Splitter => &state.config.entities.splitter.weapons,
        };

        // Parse weapons from config
        let weapons: Vec<WeaponType> = weapons_list
            .iter()
            .filter_map(|w| WeaponType::from_string(w))
            .collect();

        // Fallback to Bullet if no valid weapons configured
        let final_weapons = if weapons.is_empty() {
            vec![WeaponType::Bullet]
        } else {
            weapons
        };

        // Generate spawn position and entry path
        let spawn_x = biased_random_x(50.0, screen_width() - 50.0);
        let movement_state = create_wave_enemy_path(entity_type, spawn_x);

        // Get starting position from path
        let start_pos = match &movement_state {
            EnemyMovementState::FollowingPath { path, .. } => path.p0,
            EnemyMovementState::FreeMovement => Vec2::new(spawn_x, -30.0),
        };

        // Create enemy with Bezier entry path
        let enemy = Enemy {
            pos: start_pos,
            stats: entity_stats,
            entity_type,
            weapon: final_weapons,
            anim: EntityAnimState::default(),
            movement_state,
            fire_timer: rand::gen_range(1.0, 3.0), // Random initial delay
        };

        state.enemies.push(enemy);
    }
}
