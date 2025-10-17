# Ghost Engine

A fast-paced 2D top-down shooter where you pilot a ship alongside ghostly allies. Battle through waves of enemies—who
can become your allies—while managing your energy and ghost formations.

![demo](demo.gif)

## Features

- Unique mechanic: Summon ghost allies to fight for you.
- Multiple ghost types: Basic, Sniper, Tank, Elite, Healer, Splitter.
- Formation control: Switch between Line, Circle, and V-shape for tactical advantage.
- Variety of weapons: Bullet, laser, homing missile, plasma, bombs.
- Pure Maths Procedural Animation: Smooth, dynamic ship and ghost movements.
- Predictive aiming: Enemy lead shots based on player velocity, so watch out.
- Energy system: Summons, parries, and dashes consume energy.
- Dynamic waves: Lua scripts define enemy waves and behaviors.
- Parry, dash, and cancel summoning to adapt on the fly.

## Controls

- Move: Arrow keys or WASD
- Shoot: H/J/K/L for primary–quaternary weapons
- Dash(I-frames): Shift (costs energy)
- Summon Ghosts: Space-bar (costs energy)
- Parry Missiles: X (quick window, costs energy)
- Cancel Summon: C (ghosts return to player queue)
- Change Formation: 1–3 (Line, Circle, V-shape)
- Single Summon: F1–F6 for ghost types

## Play the game

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Build and run:
   ```bash
   rustup update
   cargo run --release
   ```

## Customization

- Enemy waves and behaviors are scripted in Lua under `scripts/waves/`.
- Dash, formation, and weapon configs can be tweaked in `src/default.rs` and `config.toml`.

---

Your enemies don’t die—they obey.