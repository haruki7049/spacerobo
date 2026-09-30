---
name: game-configs
description: Change the spacerobo configuration file format (GameConfigs and the structs under crates/commons/src/configs/). Use when adding, renaming or removing a config field or key binding, changing a default, or editing the configuration section of docs/README.md.
---

# Changing `GameConfigs`

The player configuration is a TOML file loaded at startup. A careless schema change silently resets every existing
user's configuration, so follow this procedure.

## 1. How Loading Works

- `crates/client/src/main.rs` loads the file with `confy::load_path(args.config_file())`.
- The default path comes from `directories::ProjectDirs::from("dev", "haruki7049", "spacerobo")` plus `config.toml`
  (`crates/client/src/cli.rs`); `spr --config-file <path>` overrides it.
- **If loading fails for any reason (e.g. a syntax error or a value of the wrong type), the whole config falls back to
  `GameConfigs::default()`**, with only a warning on stderr. Every config struct has `#[serde(default)]`, so a field or
  table missing from the file takes its value from the struct's `Default` impl instead of failing.
- The schema is `GameConfigs` (`crates/commons/src/configs.rs`) and `player::Config` with its nested structs
  (`crates/commons/src/configs/player.rs`). Key bindings are Bevy `KeyCode` values, written in TOML by their variant
  name (e.g. `"KeyW"`, `"ControlLeft"`).
- Code reads the config through `Res<GameConfigs>` (e.g. `game_configs.player.keyboard.respawn`).

## 2. Procedure

1. **Read the current schema** in `crates/commons/src/configs/` and every use site
   (`grep -rn 'game_configs\|GameConfigs' crates`).
1. **Keep old files loadable.**
   - Adding a field: the struct-level `#[serde(default)]` fills it from the `Default` impl when a file lacks it. Put
     `#[serde(default)]` on every new struct too. Otherwise existing users silently lose all their settings.
   - Renaming or removing a field: this breaks existing files. Ask the user before doing it, and mention it under
     breaking changes in the PR.
1. **Update `Default`**: Every field needs a sensible default in the struct's `Default` impl. Keep the existing
   default key bindings unless the user asks to change them.
1. **Update the manual**: `docs/README.md` documents the file format. Update the example TOML block and add or change
   the `####` section for each field you touched. Do not leave `TODO` placeholders for fields you add. If you notice
   drift in fields you did not touch, report it instead of fixing it unasked.
1. **Test**: Add or update unit tests in the `#[cfg(test)] mod tests` of the touched file (e.g. that `Default`
   produces the expected values).

## 3. Verify

- `cargo test -p spacerobo_commons` and `cargo xtask`.
- `toml` is a dev-dependency of `spacerobo_commons`. The tests in `crates/commons/src/configs/player.rs` deserialize
  old and partial files (e.g. `missing_fields_take_default_values`); extend them for the fields you add.
- The game parses the file with the `toml` version that `confy` depends on, which can differ from the dev-dependency.
  Check `Cargo.lock` when either is bumped.
- The in-game effect of a binding or force value needs a play-test by the user (see [`verify`](../verify/SKILL.md)).
