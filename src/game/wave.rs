use crate::game::utils::biased_random_x;
use crate::models::*;
use crate::scripting::LuaScripting;
use macroquad::prelude::*;
use mlua::prelude::*;

/// Manages wave progression and enemy spawning
pub struct WaveManager {
    pub scripting: LuaScripting,
    pub current_wave_number: usize,
    pub current_wave: Option<WaveDefinition>,
    pub state: WaveState,
    pub prep_timer: f32,       // Countdown before wave starts
    pub transition_timer: f32, // Pause between waves
    pub total_waves: usize,    // How many waves in total

    // Lua callbacks (stored for execution)
    on_start_callback: Option<LuaFunction>,
    on_complete_callback: Option<LuaFunction>,
}

impl WaveManager {
    /// Create new WaveManager and initialize Lua
    pub fn new(total_waves: usize) -> Self {
        let scripting = LuaScripting::new().expect("Failed to initialize Lua scripting");

        Self {
            scripting,
            current_wave_number: 0,
            current_wave: None,
            state: WaveState::Ready,
            prep_timer: 0.0,
            transition_timer: 0.0,
            total_waves,
            on_start_callback: None,
            on_complete_callback: None,
        }
    }

    /// Create a dummy WaveManager for mem::replace (doesn't initialize Lua)
    pub fn new_dummy() -> Self {
        // Use Lua::new() instead of LuaScripting::new() to avoid loading init.lua
        let lua = mlua::Lua::new();
        let scripting = LuaScripting { lua };

        Self {
            scripting,
            current_wave_number: 0,
            current_wave: None,
            state: WaveState::Ready,
            prep_timer: 0.0,
            transition_timer: 0.0,
            total_waves: 0,
            on_start_callback: None,
            on_complete_callback: None,
        }
    }

    /// Start the next wave
    pub fn start_next_wave(&mut self, config: &crate::config::GameConfig) -> bool {
        self.current_wave_number += 1;

        if self.current_wave_number > self.total_waves {
            println!("✓ All waves complete! Victory!");
            return false; // No more waves
        }

        // Load wave from Lua script
        match self.scripting.load_wave(self.current_wave_number) {
            Ok(lua_wave) => {
                // Store callbacks BEFORE converting (to avoid partial move)
                let on_start = lua_wave.on_start.clone();
                let on_complete = lua_wave.on_complete.clone();

                // Convert to WaveDefinition
                match WaveDefinition::from_lua(lua_wave, config) {
                    Some(wave_def) => {
                        self.on_start_callback = on_start;
                        self.on_complete_callback = on_complete;
                        self.prep_timer = wave_def.prep_time;
                        self.current_wave = Some(wave_def);
                        self.state = WaveState::Preparing;
                        println!("✓ Wave {} loaded", self.current_wave_number);
                        true
                    }
                    None => {
                        println!("✘ Failed to parse wave {}", self.current_wave_number);
                        false
                    }
                }
            }
            Err(e) => {
                println!("✘ Failed to load wave {}: {}", self.current_wave_number, e);
                false
            }
        }
    }

    /// Update wave state machine (call this, then call spawn_for_wave separately)
    pub fn update_state(
        &mut self,
        enemies_alive: usize,
        _player_energy: &mut f32, // TODO: Max-Energy increase based on enemies slayed
        _player_max_energy: f32,
        delta: f32,
    ) {
        match self.state {
            WaveState::Ready => {
                // Waiting to start first wave (or after transition)
                // Will be started externally by calling start_next_wave()
            }

            WaveState::Preparing => {
                // Countdown timer before wave starts
                self.prep_timer -= delta;

                if self.prep_timer <= 0.0 {
                    // Execute wave start callback
                    if let Some(callback) = self.on_start_callback.take() {
                        if let Err(e) = self.scripting.execute_wave_start(Some(callback)) {
                            println!("✘ Wave start callback error: {e}");
                        }
                    }

                    self.state = WaveState::Active;
                    println!(" Wave {} Active!", self.current_wave_number);
                }
            }

            WaveState::Active => {
                // Check if wave complete (spawning handled separately)
                let wave_complete = if let Some(wave) = &self.current_wave {
                    let all_spawned = wave.spawns.iter().all(|s| s.spawned >= s.count);
                    all_spawned && enemies_alive == 0
                } else {
                    false
                };

                if wave_complete {
                    self.state = WaveState::Complete;

                    if let Some(wave) = &self.current_wave {
                        println!("✓ Wave {} Complete: {}", wave.wave_number, wave.name);
                    }
                }
            }

            WaveState::Complete => {
                // Wave just completed, execute callback
                if let Some(callback) = self.on_complete_callback.take() {
                    if let Err(e) = self.scripting.execute_wave_complete(Some(callback)) {
                        println!("✘ Wave complete callback error: {e}");
                    }
                }

                // Transition to next wave
                self.transition_timer = 3.0; // 3 second pause
                self.state = WaveState::Transition;
            }

            WaveState::Transition => {
                // Brief pause before next wave
                self.transition_timer -= delta;

                if self.transition_timer <= 0.0 {
                    self.state = WaveState::Ready;
                    // Next wave will be started externally
                }
            }
        }
    }

    /// Spawn enemies for the current wave (call separately to avoid borrow issues)
    pub fn spawn_for_wave(&mut self, game_state: &mut GameState, delta: f32) {
        if self.state != WaveState::Active {
            return;
        }

        self.spawn_wave_enemies(game_state, delta);
    }

    /// Spawn enemies according to wave definition
    fn spawn_wave_enemies(&mut self, game_state: &mut GameState, delta: f32) {
        let Some(wave) = &mut self.current_wave else {
            return;
        };

        for spawn in &mut wave.spawns {
            // Skip if all spawned
            if spawn.spawned >= spawn.count {
                continue;
            }

            // Update timer
            spawn.timer -= delta;

            // Spawn enemy when timer reaches 0
            if spawn.timer <= 0.0 {
                // Get entity stats from config
                let entity_stats = spawn.enemy_type.get_stats(&game_state.config.entities);

                // Get weapons list
                let weapons_list = match spawn.enemy_type {
                    EntityType::BasicFighter => &game_state.config.entities.basic_fighter.weapons,
                    EntityType::Sniper => &game_state.config.entities.sniper.weapons,
                    EntityType::Tank => &game_state.config.entities.tank.weapons,
                    EntityType::Elite => &game_state.config.entities.elite.weapons,
                    EntityType::Healer => &game_state.config.entities.healer.weapons,
                    EntityType::Splitter => &game_state.config.entities.splitter.weapons,
                };

                let weapons: Vec<WeaponType> = weapons_list
                    .iter()
                    .filter_map(|w| WeaponType::from_string(w))
                    .collect();

                let final_weapons = if weapons.is_empty() {
                    vec![WeaponType::Bullet]
                } else {
                    weapons
                };

                // Create enemy with center-biased random X position
                let enemy = Enemy {
                    pos: Position {
                        //x: gen_range(50.0, screen_width() - 50.0)
                        x: biased_random_x(50.0, screen_width() - 50.0),
                        y: -20.0,
                    },
                    stats: entity_stats,
                    entity_type: spawn.enemy_type,
                    weapon: final_weapons,
                    anim: EntityAnimState::default(), // Default animation state
                };

                game_state.enemies.push(enemy);
                game_state.enemy_fire_timers.push(0.0);

                spawn.spawned += 1;
                spawn.timer = spawn.interval; // Reset timer

                println!(
                    "  Spawned {:?} ({}/{})",
                    spawn.enemy_type, spawn.spawned, spawn.count
                );
            }
        }
    }

    // UI Helpers

    /// Get current wave info for UI
    #[allow(dead_code)] // Will be used in Task 7: Wave UI
    pub fn get_wave_info(&self) -> Option<WaveInfo> {
        self.current_wave.as_ref().map(|wave| WaveInfo {
            wave_number: wave.wave_number,
            name: wave.name.clone(),
            enemies_spawned: wave.spawns.iter().map(|s| s.spawned).sum(),
            enemies_total: wave.total_enemy_count(),
        })
    }

    /// Get prep countdown (for UI)
    #[allow(dead_code)] // Will be used in Wave UI
    pub fn get_prep_countdown(&self) -> Option<f32> {
        if self.state == WaveState::Preparing {
            Some(self.prep_timer)
        } else {
            None
        }
    }

    /// Check if all waves complete
    #[allow(dead_code)] // Will be used in Wave UI
    pub fn is_game_complete(&self) -> bool {
        self.current_wave_number >= self.total_waves && self.state == WaveState::Transition
    }
}

/// Wave info for UI display
#[allow(dead_code)] // Wave UI Helper
pub struct WaveInfo {
    pub wave_number: usize,
    pub name: String,
    pub enemies_spawned: usize,
    pub enemies_total: usize,
}
