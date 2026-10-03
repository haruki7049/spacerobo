---
name: update-dependencies
description: Bump spacerobo dependencies - Bevy, avian3d, other crates in Cargo.toml/Cargo.lock, the Rust toolchain, or GitHub Actions versions - and keep every pinned copy of a version in sync. Use for any version bump or Cargo.lock change.
---

# Dependency Update Workflow

Only update dependencies when the user asks. Keep each PR to one logical upgrade (e.g. "Bevy 0.18 → 0.19 with the
matching avian3d"), and use the `chore:` type (`build:` fails the PR title check).

## 1. Where Versions Are Pinned

| What | Where |
| :--- | :--- |
| Crate versions and features | `[workspace.dependencies]` in the root `Cargo.toml`; resolved versions in `Cargo.lock` |
| Rust toolchain | `rust-toolchain.toml` (`channel`) |
| Nix inputs | `flake.lock`, updated by weekly Dependabot pull requests; do not touch it unless asked |
| GitHub Actions | `uses:` lines in `.github/workflows/*.yml` |
| sccache | nixpkgs in `flake.lock` for the devShell and the Nix-based CI jobs; `version:` of `mozilla-actions/sccache-action` in the Windows job of `.github/workflows/heavy-ci.yml`. Keep them on the same version |

The Nix build and every CI job read the toolchain from `rust-toolchain.toml`: the Windows job in
`.github/workflows/heavy-ci.yml` installs it with `rustup toolchain install`. State a Rust bump in the PR.

## 2. Bevy and avian3d

avian3d releases target one Bevy minor version. Bump them together.

1. Read the current versions: `grep -A1 -E '^name = "(bevy|avian3d)"$' Cargo.lock`.
1. Find the avian3d release that supports the target Bevy version from its changelog or crates.io dependency list.
   Cite the source; do not guess.
1. Edit the versions in `[workspace.dependencies]`. Keep the Bevy feature list (`debug`, `serialize`,
   `track_location`, `wayland`) unless a feature was renamed or removed; check that in the release notes.
1. Resolve: `cargo update -p bevy -p avian3d` (narrow updates; avoid a bare `cargo update` that bumps everything).
1. Make sure there is exactly one Bevy version in the graph: `cargo tree -d -e normal | grep -E '^bevy '`.
1. Follow the Bevy migration guide (`https://bevy.org/learn/migration-guides/`) for every minor version crossed and
   fix the code. See [`bevy-development`](../bevy-development/SKILL.md) for checking APIs at the new version.
1. Update the pinned versions stated in `AGENTS.md` (Project Overview) and in the examples of the
   `bevy-development` skill.

## 3. Other Crates

- Bump with `cargo update -p <crate>` for a compatible update, or change the version requirement in
  `[workspace.dependencies]` for a new major version.
- Keep the dependency declared at the workspace level and referenced with `<dep>.workspace = true` in crates.

## 4. Verification

Run all of these; a dependency bump can break any platform:

- `cargo xtask --debug --release` (what CI runs)
- `cargo clippy --workspace --all-targets -- --deny warnings`
- `nix build .#default` (crane vendors dependencies from `Cargo.lock`)
- `nix build .#checks.x86_64-linux.treefmt`

The game itself still needs a play-test by the user after a Bevy or avian3d bump: rendering, input, audio and physics
behaviour can change without a compile error. Report that as unverified. macOS and Windows are covered only by CI.
