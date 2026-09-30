# Agent Guidelines for `spacerobo`

This document defines the project context, invariants and non-negotiable rules for AI agents working on the
`spacerobo` repository. Procedures live in workspace skills (see [Workspace Skills](#5-workspace-skills)).

______________________________________________________________________

## 1. Project Overview

`spacerobo` is a 3D game that simulates a robo moving in space, built with [Bevy](https://bevy.org) and the
[avian3d](https://github.com/avianphysics/avian) physics engine. The game is not stable yet.

- **Pinned versions**: Rust `1.94.1` (`rust-toolchain.toml`), edition `2024`, `bevy 0.18.1`, `avian3d 0.6.1` (see
  `Cargo.lock`). Bevy changes its API every minor release, so check APIs against these exact versions, never against
  memory of another version. For example, this code base uses `Message`/`MessageWriter` for buffered events and
  `Event` with observers (`On<T>`), not the older `EventWriter` API. See
  [`bevy-development`](.agents/skills/bevy-development/SKILL.md).
- **Development environment**: Nix flake (`flake.nix`) with `direnv`/`nix-direnv` (`.envrc`). The dev shell provides
  the toolchain and the native libraries Bevy links against (ALSA, udev, Vulkan, X11, Wayland). Formatting is
  `treefmt` through `nix fmt`; `treefmt` itself is not on the dev shell `PATH`.
- **Binary**: `spr` (package `spacerobo_client`, the workspace default member). `cargo run` starts the game.

### Workspace Layout

Crates form a strict dependency layering. A crate may depend only on crates in lower layers; never add a dependency
that points upward or sideways.

| Layer | Crate (path) | Contents | Workspace deps |
| :--- | :--- | :--- | :--- |
| 0 | `spacerobo_commons` (`crates/commons`) | `GameMode` state, `Hp`, `KillCounter`, `DeathMessage`, `Damage`, the `Player`/`Target`/`Bullet` traits, `GameConfigs`, `ControllablePlugin` (keyboard/mouse control) | none |
| 1 | `spacerobo_target` (`crates/target`) | Target entity | commons |
| 2 | `spacerobo_gun` (`crates/gun`) | `Gun`, bullets, select fire, `GunPlugin` | commons, target |
| 3 | `spacerobo_player` (`crates/player`) | Player entity, HUD, respawn, `PlayerCommonPlugin` | commons, gun |
| 4 | `spacerobo_title_plugin` (`crates/plugins/title_plugin`) | Title screen (`GameMode::Title`) | commons |
| 4 | `spacerobo_shooting_range_plugin` (`crates/plugins/shooting_range_plugin`) | Shooting range scene (`GameMode::InGame`): targets, boundary, damage and death | commons, target, player |
| 5 | `spacerobo_client` (`crates/client`) | `spr` binary: CLI, config loading, `App` assembly | commons, both plugins |
| — | `spacerobo_xtask` (`crates/xtask`) | `cargo xtask` build runner; excluded from its own runs | none |

Other paths:

- `crates/client/assets/`: game assets (`SE/*.ogg`, `branding/icon.png`). Every third-party asset must be credited in
  `crates/client/assets/THIRDPARTY_LICENSE.md`.
- `docs/README.md`: the player manual, including the configuration file reference.
- `.github/workflows/`: CI (see [Verification](#3-verification)).
- `scripts/push-artifacts-to-cachix.nu`: pushes build outputs to Cachix; run by CI only.

______________________________________________________________________

## 2. Strict Safety & Operational Rules (Always Enforced)

- **NEVER MERGE PULL REQUESTS**: Never merge PRs, enable auto-merge (`gh pr merge --auto`), or `git merge` into
  `main`. Merging rests strictly with the human maintainer.
- **Topic branches only**: Never commit or push to `main`. Work on a topic branch. Ordinary pushes of new commits to
  your own topic branch need no confirmation; rewriting pushed history (amend or rebase, then force-push) does.
- **Never propose commits or pushes unprompted**: Do not ask "Should I commit?" and do not append proposed commit
  messages. When instructed, or when creating or updating a PR, run `git commit` and `git push` directly.
- **Conventional Commits restricted to CI types**: The PR title check (`pr-conventional-commits-validation.yml`) accepts
  only `feat`, `fix`, `docs`, `test`, `ci`, `refactor`, `perf`, `chore` and `revert`, with an optional scope. Use only
  these types for PR titles and commits. `build:` and `style:` fail the check; use `chore:` for dependency and Nix
  changes.
- **No issue numbers in commit messages**: Neither in the summary nor the body. Squash merges copy every commit message
  into `main`, so a `Closes #N` in a commit body can close the wrong issue. Link issues only from the PR description.
- **No session links**: Do not put AI session URLs or identifiers (e.g. a `Claude-Session:` trailer) in commits, PRs,
  issues or comments. They are not accessible from this public repository. A `Co-Authored-By:` trailer is fine.
- **English-only repository text**: Code comments, doc comments, documentation, skills, commit messages, PR and issue
  text are written in English. (`THIRDPARTY_LICENSE.md` keeps its existing Japanese headings; do not translate it.)
- **Evidence first**: Base every claim on file contents, command output or documentation you actually read. State
  explicitly what you could not verify.
- **Targeted edits**: Make the minimal change the request needs. Do not reformat or refactor unrelated code.
- **No milestones, releases or tags**: Never set milestones, create tags or publish GitHub Releases unless the user
  explicitly asks. Tags such as `0.2.1` (no `v` prefix) trigger the Cachix push workflow.
- **Do not bump `flake.lock` in PRs**: Dependabot opens weekly pull requests for the flake inputs
  (`.github/dependabot.yml`). Touch `flake.lock` only when the user asks.

______________________________________________________________________

## 3. Verification

Run the checks that match the change before reporting it as done (details and the report format:
[`verify`](.agents/skills/verify/SKILL.md)):

| Purpose | Command | Notes |
| :--- | :--- | :--- |
| Build, check, clippy, test and doc for the workspace | `cargo xtask` | Debug profile. CI runs `cargo xtask --debug --release` on Linux, macOS and Windows |
| Single action | `cargo xtask <build\|check\|clippy\|test\|doc>` | `cargo x` is an alias |
| Tests of one crate | `cargo test -p <crate>` | e.g. `cargo test -p spacerobo_commons` |
| Lints that fail on warnings | `cargo clippy --workspace --all-targets -- --deny warnings` | `cargo xtask clippy` only reports warnings, but the clippy check in `nix flake check` denies them |
| Formatting check | `nix build .#checks.x86_64-linux.treefmt` | Checks Rust, Nix, TOML, Markdown, shell and workflows without writing files |
| Format files | `nix fmt` | Rewrites files in place |
| Flake checks (clippy + treefmt) | `nix flake check` | Slow: builds the workspace in the Nix sandbox |
| Nix package | `nix build .#default` | Slow; run when `flake.nix`, assets or packaging change |

- **The game needs a display and a GPU.** Tests and CI run headless, so an agent cannot confirm gameplay, rendering,
  input or audio by running the game. Report such behaviour as unverified and ask the user to play-test.
- Never delete or weaken assertions to make a check pass. Fix the root cause.

______________________________________________________________________

## 4. Status Assessment

When asked to check status:

1. **Local Git state**: `git status -s -b`, `git log -n 5 --oneline`, and how far the branch is behind `origin/main`
   (`git fetch` first).
1. **GitHub PRs (always)**: `gh pr list` and `gh pr status`.
1. **GitHub issues (always)**: `gh issue list`.
1. **Environment health**: `cargo xtask` and `nix build .#checks.x86_64-linux.treefmt`.
1. **Synthesis**: Report local state, the open PR and issue lists (number, title, state) or that there are none, and
   environment health.

______________________________________________________________________

## 5. Workspace Skills

Procedures are maintained as skills under `.agents/skills/`. Read the matching `SKILL.md` before starting the task.

| Trigger / Context | Skill | Purpose |
| :--- | :--- | :--- |
| Writing or changing game code (systems, plugins, components, assets) | [`bevy-development`](.agents/skills/bevy-development/SKILL.md) | Repository ECS patterns and how to check Bevy/avian3d APIs at the pinned versions |
| Changing `GameConfigs` or the config file format | [`game-configs`](.agents/skills/game-configs/SKILL.md) | Keeping user config files loadable and `docs/README.md` in sync |
| Verifying a change | [`verify`](.agents/skills/verify/SKILL.md) | Choosing the narrowest check and the report format |
| Commits, PRs, labels, project fields | [`pr-workflow`](.agents/skills/pr-workflow/SKILL.md) | Branch, commit and PR procedure for this repository |
| Setting GitHub Project fields | [`github-projects`](.agents/skills/github-projects/SKILL.md) | "Spacerobo project" (#13) schema and single-field updates |
| Bumping Bevy, avian3d, Rust or other dependencies | [`update-dependencies`](.agents/skills/update-dependencies/SKILL.md) | Lock-step upgrades and every place a version is pinned |
| Deleting, overwriting, resetting, force-pushing | [`irreversible`](.agents/skills/irreversible/SKILL.md) | Pre-checks and the confirmation format |
