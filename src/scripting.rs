use mlua::prelude::*;
use std::path::Path;

/// Lua scripting system for wave definitions and AI behaviors
pub struct LuaScripting {
    pub(crate) lua: Lua, // pub(crate) for WaveManager::new_dummy()
}

impl LuaScripting {
    /// Initialize Lua runtime and expose Rust API
    pub fn new() -> LuaResult<Self> {
        let lua = Lua::new();

        // Load init.lua for helper functions
        if Path::new("scripts/init.lua").exists() {
            let init_script = std::fs::read_to_string("scripts/init.lua")
                .map_err(|e| LuaError::RuntimeError(format!("✘ Failed to read init.lua: {e}")))?;
            lua.load(&init_script).exec()?;
            println!("✓ Loaded scripts/init.lua");
        } else {
            return Err(LuaError::RuntimeError(
                "✘ scripts/init.lua not found! Required for Lua helper functions.".to_string(),
            ));
        }

        Ok(Self { lua })
    }

    /// Load a wave definition from Lua script
    pub fn load_wave(&self, wave_num: usize) -> LuaResult<LuaWaveDefinition> {
        let wave_path = format!("scripts/waves/wave_{wave_num}.lua");

        if !Path::new(&wave_path).exists() {
            return Err(LuaError::RuntimeError(format!(
                "✘ Wave script not found: {wave_path}"
            )));
        }

        let script = std::fs::read_to_string(&wave_path)
            .map_err(|e| LuaError::RuntimeError(format!("✘ Failed to read {wave_path}: {e}")))?;

        // Execute script and get the wave table
        let wave_table: LuaTable = self.lua.load(&script).eval()?;

        // Parse wave definition from Lua table
        let wave_num = wave_table.get::<usize>("wave_number")?;
        let name = wave_table.get::<String>("name")?;
        let prep_time = wave_table.get::<f32>("prep_time").unwrap_or(3.0);

        // Parse spawns array
        let spawns_table: LuaTable = wave_table.get("spawns")?;
        let mut spawns = Vec::new();

        for pair in spawns_table.sequence_values::<LuaTable>() {
            let spawn_table = pair?;
            let enemy_type = spawn_table.get::<String>("type")?;
            let count = spawn_table.get::<usize>("count")?;
            let interval = spawn_table.get::<f32>("interval")?;
            let delay = spawn_table.get::<f32>("delay").unwrap_or(0.0);

            spawns.push(LuaSpawnDefinition {
                enemy_type,
                count,
                interval,
                delay,
            });
        }

        // Get callbacks (optional)
        let on_start = wave_table.get::<Option<LuaFunction>>("on_start")?;
        let on_complete = wave_table.get::<Option<LuaFunction>>("on_complete")?;

        println!(
            "✓ Loaded wave {}: {} ({} spawn groups)",
            wave_num,
            name,
            spawns.len()
        );

        Ok(LuaWaveDefinition {
            wave_number: wave_num,
            name,
            prep_time,
            spawns,
            on_start,
            on_complete,
        })
    }

    /// Execute wave start callback
    pub fn execute_wave_start(&self, callback: Option<LuaFunction>) -> LuaResult<()> {
        if let Some(func) = callback {
            func.call::<()>(())?;
        }
        Ok(())
    }

    /// Execute wave complete callback
    pub fn execute_wave_complete(&self, callback: Option<LuaFunction>) -> LuaResult<()> {
        if let Some(func) = callback {
            func.call::<()>(())?;
        }
        Ok(())
    }

    /// Get reference to Lua runtime (for advanced usage)
    #[allow(dead_code)] // Reserved for future Lua API exposure (AI behaviors, custom scripts)
    pub fn lua(&self) -> &Lua {
        &self.lua
    }
}

/// Wave definition loaded from Lua
pub struct LuaWaveDefinition {
    pub wave_number: usize,
    pub name: String,
    pub prep_time: f32, // Countdown before wave starts
    pub spawns: Vec<LuaSpawnDefinition>,
    pub on_start: Option<LuaFunction>,
    pub on_complete: Option<LuaFunction>,
}

/// Individual spawn group within a wave
#[derive(Debug, Clone)]
pub struct LuaSpawnDefinition {
    pub enemy_type: String, // "BasicFighter", "Sniper", etc.
    pub count: usize,       // How many to spawn
    pub interval: f32,      // Seconds between each spawn
    pub delay: f32,         // Delay before starting this spawn group
}
