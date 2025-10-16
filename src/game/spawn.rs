use crate::config::*;
use crate::game::utils::biased_random_x;
use crate::models::*;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

/// Create a Bezier entry path based on enemy type
pub fn create_wave_enemy_path(entity_type: EntityType, spawn_x: f32) -> EnemyMovementState {
    let screen_w = screen_width();
    let screen_h = screen_height();

    match entity_type {
        EntityType::BasicFighter => {
            // Random variant: 3 different entry styles
            let variant = gen_range(0, 3);
            let path = match variant {
                0 => {
                    // Gentle Curve
                    BezierPath {
                        p0: Position {
                            x: spawn_x,
                            y: -30.0,
                        },
                        p1: Position {
                            x: spawn_x + gen_range(-80.0, 80.0),
                            y: 40.0,
                        },
                        p2: Position {
                            x: spawn_x + gen_range(-40.0, 40.0),
                            y: 100.0,
                        },
                        p3: Position {
                            x: spawn_x,
                            y: 140.0,
                        },
                        duration: gen_range(1.2, 1.8),
                        use_cubic: true,
                    }
                }
                1 => {
                    // Sharp Swoop
                    let swoop_dir = if gen_range(0, 2) == 0 { 1.0 } else { -1.0 };
                    BezierPath {
                        p0: Position {
                            x: spawn_x + swoop_dir * 150.0,
                            y: -30.0,
                        },
                        p1: Position {
                            x: spawn_x + swoop_dir * 120.0,
                            y: 30.0,
                        },
                        p2: Position {
                            x: spawn_x - swoop_dir * 30.0,
                            y: 90.0,
                        },
                        p3: Position {
                            x: spawn_x,
                            y: 130.0,
                        },
                        duration: gen_range(1.0, 1.5),
                        use_cubic: true,
                    }
                }
                _ => {
                    // Loop Entry
                    BezierPath {
                        p0: Position {
                            x: spawn_x,
                            y: -30.0,
                        },
                        p1: Position {
                            x: spawn_x + 60.0,
                            y: 20.0,
                        },
                        p2: Position {
                            x: spawn_x - 60.0,
                            y: 80.0,
                        },
                        p3: Position {
                            x: spawn_x,
                            y: 120.0,
                        },
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
                        p0: Position {
                            x: -50.0,
                            y: gen_range(60.0, 120.0),
                        },
                        p1: Position {
                            x: 80.0,
                            y: gen_range(40.0, 80.0),
                        },
                        p2: Position {
                            x: 140.0,
                            y: gen_range(100.0, 140.0),
                        },
                        p3: Position {
                            x: 110.0,
                            y: gen_range(130.0, 160.0),
                        },
                        duration: gen_range(1.5, 2.0),
                        use_cubic: true,
                    }
                } else {
                    // Curved approach
                    BezierPath {
                        p0: Position {
                            x: -50.0,
                            y: gen_range(40.0, 80.0),
                        },
                        p1: Position {
                            x: 120.0,
                            y: gen_range(20.0, 60.0),
                        },
                        p2: Position {
                            x: 100.0,
                            y: gen_range(120.0, 160.0),
                        },
                        p3: Position {
                            x: 100.0,
                            y: gen_range(140.0, 170.0),
                        },
                        duration: gen_range(1.8, 2.3),
                        use_cubic: true,
                    }
                }
            } else {
                // From RIGHT
                if entry_style == 0 {
                    // Fast sweep
                    BezierPath {
                        p0: Position {
                            x: screen_w + 50.0,
                            y: gen_range(60.0, 120.0),
                        },
                        p1: Position {
                            x: screen_w - 80.0,
                            y: gen_range(40.0, 80.0),
                        },
                        p2: Position {
                            x: screen_w - 140.0,
                            y: gen_range(100.0, 140.0),
                        },
                        p3: Position {
                            x: screen_w - 110.0,
                            y: gen_range(130.0, 160.0),
                        },
                        duration: gen_range(1.5, 2.0),
                        use_cubic: true,
                    }
                } else {
                    // Curved approach
                    BezierPath {
                        p0: Position {
                            x: screen_w + 50.0,
                            y: gen_range(40.0, 80.0),
                        },
                        p1: Position {
                            x: screen_w - 120.0,
                            y: gen_range(20.0, 60.0),
                        },
                        p2: Position {
                            x: screen_w - 100.0,
                            y: gen_range(120.0, 160.0),
                        },
                        p3: Position {
                            x: screen_w - 100.0,
                            y: gen_range(140.0, 170.0),
                        },
                        duration: gen_range(1.8, 2.3),
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
                        p0: Position {
                            x: screen_w + 50.0,
                            y: -50.0,
                        },
                        p1: Position {
                            x: screen_w * 0.7,
                            y: 60.0,
                        },
                        p2: Position {
                            x: screen_w * 0.3,
                            y: 140.0,
                        },
                        p3: Position {
                            x: gen_range(100.0, 200.0),
                            y: gen_range(160.0, 200.0),
                        },
                        duration: gen_range(2.2, 2.8),
                        use_cubic: true,
                    }
                } else {
                    BezierPath {
                        p0: Position { x: -50.0, y: -50.0 },
                        p1: Position {
                            x: screen_w * 0.3,
                            y: 60.0,
                        },
                        p2: Position {
                            x: screen_w * 0.7,
                            y: 140.0,
                        },
                        p3: Position {
                            x: gen_range(screen_w - 200.0, screen_w - 100.0),
                            y: gen_range(160.0, 200.0),
                        },
                        duration: gen_range(2.2, 2.8),
                        use_cubic: true,
                    }
                }
            } else {
                // Overhead Arc
                BezierPath {
                    p0: Position {
                        x: gen_range(0.0, screen_w),
                        y: -50.0,
                    },
                    p1: Position {
                        x: screen_w / 2.0 + gen_range(-100.0, 100.0),
                        y: 30.0,
                    },
                    p2: Position {
                        x: spawn_x + gen_range(-80.0, 80.0),
                        y: 120.0,
                    },
                    p3: Position {
                        x: spawn_x,
                        y: gen_range(150.0, 180.0),
                    },
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
                        p0: Position {
                            x: screen_w / 2.0,
                            y: -50.0,
                        },
                        p1: Position {
                            x: screen_w / 2.0 + 120.0,
                            y: 40.0,
                        },
                        p2: Position {
                            x: screen_w / 2.0 - 120.0,
                            y: 120.0,
                        },
                        p3: Position {
                            x: screen_w / 2.0,
                            y: 100.0,
                        },
                        duration: 2.5,
                        use_cubic: true,
                    }
                }
                1 => {
                    // Spiral Entry
                    BezierPath {
                        p0: Position {
                            x: screen_w / 2.0 + 150.0,
                            y: -30.0,
                        },
                        p1: Position {
                            x: screen_w / 2.0 + 100.0,
                            y: 60.0,
                        },
                        p2: Position {
                            x: screen_w / 2.0 - 80.0,
                            y: 100.0,
                        },
                        p3: Position {
                            x: screen_w / 2.0,
                            y: 120.0,
                        },
                        duration: 2.2,
                        use_cubic: true,
                    }
                }
                _ => {
                    // Overhead Dive
                    BezierPath {
                        p0: Position {
                            x: screen_w / 2.0,
                            y: -80.0,
                        },
                        p1: Position {
                            x: screen_w / 2.0 - 100.0,
                            y: 20.0,
                        },
                        p2: Position {
                            x: screen_w / 2.0 + 100.0,
                            y: 80.0,
                        },
                        p3: Position {
                            x: screen_w / 2.0,
                            y: 110.0,
                        },
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
                p0: Position {
                    x: spawn_x,
                    y: -30.0,
                },
                p1: Position {
                    x: spawn_x + curve_dir * curve_strength,
                    y: gen_range(50.0, 90.0),
                },
                p2: Position {
                    x: spawn_x - curve_dir * (curve_strength * 0.5),
                    y: gen_range(100.0, 130.0),
                },
                p3: Position {
                    x: spawn_x,
                    y: gen_range(120.0, 150.0),
                },
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
            EnemyMovementState::FreeMovement => Position {
                x: spawn_x,
                y: -30.0,
            },
        };

        // Create enemy with Bezier entry path
        let enemy = Enemy {
            pos: start_pos,
            stats: entity_stats,
            entity_type,
            weapon: final_weapons,
            anim: EntityAnimState::default(),
            movement_state,
        };

        state.enemies.push(enemy);
    }
}
