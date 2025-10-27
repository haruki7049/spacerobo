# spacerobo

spacerobo: Work-in-progress space robotics

[![CI](https://img.shields.io/github/actions/workflow/status/haruki7049/spacerobo/nix-checker.yml?branch=main)](https://github.com/haruki7049/spacerobo/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Overview
--------
spacerobo is a Work-in-progress Rust project to create a simulation game of space robotics. It aims to be fast, reliable, and easy to develop with reproducible builds using Nix flakes.

This README provides:
- Quickstart for developers and users
- Build & run instructions (Nix and Cargo)
- Developer workflow (formatting, linting, testing)
- Contribution and release guidance

Repository layout
-----------------
- Cargo workspace root (Cargo.toml)
- crates/ — workspace crates (libraries and binaries)
- src/ — primary source for the main binary (if present)
- flake.nix / shell.nix — Nix devshell & reproducible development
- docs/ — documentation and examples
- assets/ — static assets used by the project

Quickstart — using Nix (recommended)
-----------------------------------
If you use Nix (flakes enabled), this repo provides a reproducible development environment.

1. Enter the dev shell (Nix Flake):
    - With flakes:
      ```sh
      nix develop
      ```
    - Or with shell.nix:
      ```sh
      nix-shell
      ```
    - You also can use nix-direnv with Nix flakes. This process uses `nix develop`:
      ```sh
      direnv allow
      ```

2. Build and run (inside the shell):
      ```sh
      cargo build --release
      ```

Quickstart — using rustup / cargo
---------------------------------
If you prefer not to use Nix:

1. Install Rust:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   whereis rustup # Checks whether rustup is correctly installed or not.
   cargo # You should be able to use cargo, Rust package manager.
   ```

2. Build and test:
   ```sh
   cargo build --workspace
   cargo test --workspace
   ```

3. Run a binary:
   ```sh
   cargo run
   ```

Development workflow
--------------------
Follow these steps to keep the repository consistent and CI-compatible.

Formatting:
- Keep code formatted with rustfmt:
  ```sh
  cargo fmt --all
  ```

  You also can use `nix fmt` command. This includes rustfmt, nixfmt-rfc-style, actionlint, mdformat, and etc.
  ```sh
  nix fmt
  ```

Linting:
- Use clippy for lints and to catch common issues:
  ```sh
  cargo clippy
  ```

Testing:
- Run unit and integration tests:
  ```sh
  cargo test --workspace --verbose
  ```

  You also can use `nix flake check` command. Please check .#check attribute on [flake.nix](./flake.nix).
  ```sh
  nix flake check
  ```

Contributing
------------
Contributions are welcome!

- Open an issue to talk with [@haruki7049](https://github.com/haruki7049) about spacerobo's issue or features you want.
- Create a Pull request if you create a patch or feature for spacerobo.
- Use GitHub Discussions when you want to talk with any spacerobo player (I think there are no players lol) about spacerobo.

Issues / bug reports
--------------------
When reporting a bug, include:
- A concise description of the problem
- Steps to reproduce
- Expected vs actual behavior
- `cargo test` output or relevant logs and backtraces
- Your OS and Rust toolchain

Examples & docs
---------------
- Add short example programs under `examples/` or `docs/` to show common usage patterns.
- Consider documenting API usage for any library crates and publishing to docs.rs if crates are published.

Release & versioning
--------------------
- I use semantic versioning for spacerobo publishing.
- If you want to discuss about spacerobo versioning, please create an issue.

Acknowledgements
----------------
- Thank contributors and list dependent libraries if appropriate.

License
-------
This project is distributed under the terms in the [LICENSE file](./LICENSE) (MIT compatible). See LICENSE for details.

Maintainers / Contact
---------------------
- @haruki7049 (maintainer)
