---
name: verify
description: Choose and run the right checks after changing spacerobo code, Nix files, workflows or documentation, and report the results honestly. Use after every change and before committing or opening a pull request.
---

# Verify

Confirm a change with the narrowest check that covers it, and report exactly what was and was not checked.

## 1. Choose the Checks

| Change | Minimum checks |
| :--- | :--- |
| Rust code in one crate | `cargo test -p <crate>`, then `cargo xtask` before committing |
| Rust code across crates, `Cargo.toml`, `Cargo.lock` | `cargo xtask` and `cargo clippy --workspace --all-targets -- --deny warnings` |
| `crates/xtask` | `cargo test -p spacerobo_xtask` and `cargo xtask` (xtask excludes itself from its own runs) |
| `flake.nix`, `flake.lock` | `nix flake check` and `nix build .#default` |
| Assets or packaging | `nix build .#default` |
| Markdown, TOML, Nix, shell or workflow formatting | `nix build .#checks.x86_64-linux.treefmt` |
| Workflows (`.github/workflows/`) | The treefmt check above (runs `actionlint`) |
| Documentation only | The treefmt check; confirm that commands, paths and names you mention exist |

Always run the treefmt check before committing. If it fails, run `nix fmt`, then review the diff and keep only the
formatting changes that belong to your task.

## 2. What Agents Cannot Verify

- **Gameplay**: The game needs a window, a GPU and audio output. Rendering, controls, physics feel, sound and UI
  layout cannot be confirmed from a headless session. Report them as unverified and ask the user to play-test with
  `cargo run`.
- **Other platforms**: CI also builds on macOS and Windows (`heavy-ci.yml`, `nix-checker.yml`). Local success on
  Linux does not prove those. Point to the CI run on the PR instead.
- **A clean Nix store**: Derivations already in the local store are not rebuilt. Rely on CI for a clean-store build.

## 3. Rules

- Do not run broad or slow checks (`nix flake check`, `--release` builds) just to look thorough. Run them when the
  change touches what they cover, or before opening a PR.
- Do not claim success from an unrelated check (e.g. `cargo check` for a behaviour change).
- Never delete, skip or weaken tests or assertions to make a check pass. Fix the root cause.
- Do not hide failures. Quote the relevant part of the output.

## 4. Report Format

End with:

- **Changed**: 1–3 lines
- **Verified**: each command run and its result
- **Not verified**: what was not checked and why (e.g. "in-game behaviour: needs a play-test")
- **Risk**: remaining risk, if any
