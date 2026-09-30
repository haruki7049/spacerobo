---
name: bevy-development
description: Write or change spacerobo game code (Bevy systems, plugins, components, messages, observers, physics, assets). Use before touching any crate under crates/ except xtask, and whenever a Bevy or avian3d API is involved.
---

# Bevy Development in `spacerobo`

Read this before adding or changing game code. It covers where code belongs, the ECS patterns this repository
already uses, and how to check APIs against the pinned dependency versions.

## 1. Check APIs at the Pinned Versions

Bevy breaks its API in every minor release, and most examples online target other versions. Never write Bevy or
avian3d code from memory.

1. Read the pinned versions: `grep -A1 -E '^name = "(bevy|avian3d)"$' Cargo.lock`.
1. Look the API up at exactly that version:
   - Source, once dependencies have been built at least once:
     `ls ~/.cargo/registry/src/*/ | grep -E '^(bevy_[a-z_]+|avian3d)-'`, then `grep -rn` in the matching directory
     (e.g. `bevy_ecs-0.18.1`).
   - docs.rs with an explicit version, e.g. `https://docs.rs/bevy/0.18.1/bevy/` and
     `https://docs.rs/avian3d/0.6.1/avian3d/`. Never use `latest`.
   - Bevy migration guides (`https://bevy.org/learn/migration-guides/`) when code or an example targets an older
     version.
1. Prefer the patterns already in this repository (section 3) over patterns from external examples.
1. Cite what you checked (path or URL) when you explain an API choice.

## 2. Where Code Belongs

Respect the crate layering in `AGENTS.md` (Workspace Layout). A crate may depend only on lower layers.

- Types shared across crates (components, messages, events, states, traits, configs) go in `spacerobo_commons`.
- Entity-specific logic goes in its crate: `target`, `gun`, `player`.
- Scene logic and the systems that tie entities together (damage, death, boundaries) go in the scene plugin crate
  (`crates/plugins/*`).
- `spacerobo_client` only parses the CLI, loads the config and assembles the `App`. Do not put gameplay there.
- A new crate must be added to `[workspace] members` and `[workspace.dependencies]` in the root `Cargo.toml` and
  should inherit the `[workspace.package]` fields (`authors.workspace = true`, etc.), as `crates/gun/Cargo.toml` does.
- Add dependencies at the workspace level (`[workspace.dependencies]`) and reference them with `<dep>.workspace = true`.

## 3. Patterns Used in This Repository

- **Plugins**: Each crate exposes a `Plugin` (`GunPlugin`, `PlayerCommonPlugin`, `ShootingRangePlugin`,
  `TitlePlugin`, `ControllablePlugin`) and registers its systems in `Plugin::build`. A plugin adds the plugins it
  depends on (`PlayerCommonPlugin` adds `GunPlugin`). Only `crates/client/src/main.rs` adds top-level plugins.
- **Game state**: `GameMode` (`Title`, `InGame`) in `spacerobo_commons`.
  - One-time setup runs on `OnEnter(GameMode::X)`.
  - Per-frame gameplay systems are gated with `.run_if(in_state(GameMode::InGame))`.
  - Entities spawned for a state carry `DespawnOnExit(GameMode::X)` (`GameMode` enables `#[states(scoped_entities)]`).
- **Messages and observers**:
  - `DeathMessage` is a buffered `Message`: written with `MessageWriter`, read with `MessageReader`, registered with
    `app.add_message::<DeathMessage>()`.
  - `Damage` is an `Event`: fired with `commands.trigger(Damage { .. })` (`crates/gun/src/gun/bullet.rs`) and handled
    by an observer registered with `app.add_observer(apply_damage_system)` that takes `On<Damage>`
    (`crates/plugins/shooting_range_plugin/src/lib.rs`).
  - Follow the same split: messages for per-frame batched processing, observer events for immediate reactions.
- **Spawning through traits**: `Player`, `Target` and `Bullet` in `spacerobo_commons` define `spawn`-style
  constructors; each concrete type implements them (`spacerobo_target::Common`, `spacerobo_player::Common`).
- **System naming**: Name systems `<verb>_system` (`setup_system`, `update_system`, `death_system`,
  `respawn_system`).
- **Clippy on systems**: Bevy system signatures often trip `clippy::too_many_arguments` and
  `clippy::type_complexity`. Allow them on the specific system or module, as `crates/player/src/lib.rs` and
  `crates/gun/src/gun/select_fire.rs` do. Do not allow lints crate-wide or silence other lints this way.
- **Physics**: avian3d components (`RigidBody`, `Collider`, `Mass`, `CollisionEventsEnabled`, `AngularVelocity`).
  `ShootingRangePlugin` sets `Gravity` to zero. `PhysicsPlugins::default()` is added once, in the client.
- **Logging**: `debug!`/`info!` from `bevy::prelude`.

## 4. Assets

- Assets live in `crates/client/assets/` and are loaded by paths relative to it, e.g.
  `asset_server.load("SE/kill.ogg")`. The Nix package copies this directory next to the `spr` binary.
- Do not add an asset whose origin and license you cannot state. Ask the user for the source.
- Credit every third-party asset in `crates/client/assets/THIRDPARTY_LICENSE.md` with its author and URL.

## 5. Tests

- Put unit tests at the bottom of the file in `#[cfg(test)] mod tests`, grouped in one module per type, with a doc
  comment per test (see `crates/commons/src/lib.rs`).
- Tests must run headless: never add `DefaultPlugins`, windows, rendering or audio output to a test.
- Rendering, input, audio and "feel" cannot be verified by an agent. Say so in the report and ask the user to
  play-test (`cargo run`).

## 6. Before Reporting

Follow [`verify`](../verify/SKILL.md). For game code, at least `cargo xtask` and
`cargo clippy --workspace --all-targets -- --deny warnings`.
