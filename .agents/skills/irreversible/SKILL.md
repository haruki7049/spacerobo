---
name: irreversible
description: Pre-checks and the confirmation prompt required before risky or hard-to-revert operations in spacerobo - deleting or overwriting files, git reset/rebase/force-push, lockfile regeneration, repository-wide formatting, cargo clean, or changes outside the repository. Read before running any of them.
---

# Irreversible Operations

Prevent data loss, unwanted history changes and broad side effects.

## 1. Treat as Risky (Confirm First)

- Deleting or overwriting user-authored files, including untracked files the user placed in the repository.
- Changing pushed history: amend, rebase or reset of pushed commits and the force-push that follows; any push to
  `main`.
- Regenerating or broadly updating lockfiles: a bare `cargo update`, `nix flake update` (`flake.lock` is maintained by
  the cron workflow).
- Running `nix fmt` when it would rewrite files outside the task (check first with
  `nix build .#checks.x86_64-linux.treefmt`).
- `cargo clean` or deleting `target/`: not destructive to sources, but forces a full Bevy rebuild that takes a long
  time.
- Anything outside the repository: the user's game config (`~/.config/spacerobo/` on Linux), global Git or Nix
  settings, the Cachix cache (`scripts/push-artifacts-to-cachix.nu` is for CI only; never run it locally).
- GitHub state other than your own topic branch and PR: repository settings, labels, other people's PRs, tags and
  releases.
- Adding, replacing or deleting binary assets in `crates/client/assets/`.

## 2. Not Risky (No Confirmation Needed)

- Ordinary pushes of new commits to your own topic branch.
- Creating a topic branch, running builds and tests, `nix fmt` on files you changed for the task.

## 3. Pre-Check

Before asking, look at:

- the current state: `git status -s -b`, `git diff`, `git log origin/<branch>..HEAD` for unpushed commits
- whether the target is tracked, generated or user-authored
- whether a dry run, narrower target or backup is available (e.g. `cargo update -p <crate> --dry-run`)

## 4. Confirmation Format

Ask once:

- **Action**:
- **Impact**: including how to recover
- **Command**:

`Proceed?`

## 5. Refuse to Proceed

Do not proceed, and report the current state instead, when:

- the target is unclear
- the impact or the recovery path cannot be explained
- the command would affect files outside the task
- unrelated user changes might be overwritten
