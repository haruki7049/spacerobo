# The Spacerobo CONTRIBUTING.md

Thank you for your interest in Spacerobo. This document describes how to set up the development environment and how to
submit a change. [AGENTS.md](AGENTS.md) describes the same rules in more detail.

## Development environment

The development environment is a [Nix](https://nixos.org) flake. With [direnv](https://direnv.net) and
[nix-direnv](https://github.com/nix-community/nix-direnv), run `direnv allow` once in the repository. Otherwise, enter
the shell with `nix develop`. The shell provides the pinned Rust toolchain and the native libraries that Bevy links
against.

The shell also sets `RUSTC_WRAPPER=sccache`, so cargo stores compiler outputs in [sccache](https://github.com/mozilla/sccache)'s
local disk cache (`~/.cache/sccache` on Linux by default). To build without it, enter the shell with `RUSTC_WRAPPER` set
to an empty value: `RUSTC_WRAPPER= nix develop`.

## Build, check and test

`cargo xtask` runs `build`, `check`, `clippy`, `test` and `doc` for the whole workspace. `cargo x` is an alias.
`doc` documents only the workspace crates (`--no-deps`) and runs only in the debug profile.

```bash
cargo xtask                 # every action, debug profile
cargo xtask test            # a single action
cargo xtask --debug --release
```

CI runs `cargo xtask --debug` on Linux (`fast-ci.yml`, quick feedback) and `cargo xtask --debug --release` on Linux,
macOS and Windows (`heavy-ci.yml`, full check).

Run the game with `cargo run`. Tests and CI are headless, so rendering, input and audio need a manual play-test.

## Formatting

Format Rust, Nix, TOML, Markdown, shell scripts and workflows with:

```bash
nix fmt
```

## Commits and pull requests

- Work on a topic branch; do not commit to `main`.
- Commit messages and pull request titles follow [Conventional Commits](https://www.conventionalcommits.org). The
  allowed types are `feat`, `fix`, `docs`, `test`, `ci`, `refactor`, `perf`, `chore` and `revert`, with an optional
  scope, for example `fix(gun): ...`.
- Write commit messages, code comments and documentation in English.
- Do not put issue numbers in commit messages. Link an issue from the pull request description with `Closes #N`.
- Describe the purposes, changes and verification in the pull request, and run `cargo xtask` and the formatter before you push.
