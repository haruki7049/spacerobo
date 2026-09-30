---
name: pr-workflow
description: Branch, commit, push and open or update pull requests for spacerobo, including the PR title check, labels and GitHub Project registration. Use whenever creating commits, pushing, or creating or editing a pull request or issue.
---

# Pull Request & Commit Workflow for `spacerobo`

## 1. Branch

- Never commit to `main`. Create a topic branch from an up-to-date `main`:
  `git fetch origin && git switch -c <branch> origin/main`.
- Branch names are short kebab-case descriptions, as in existing branches: `fix-self-kill`,
  `refactor-target-trait`, `implement-sword`.
- Do not touch branches or PRs you did not create unless the user asks.

## 2. Commit

- Commit only when the user asks, or as part of creating or updating a PR they asked for. Never ask "Should I
  commit?" and never append a proposed commit message.
- Stage only the files of the task: `git add <path>...`, never `git add -A` or `git add .`.
- Message: Conventional Commits, English, imperative mood, under 72 characters, no trailing period.
- Allowed types (enforced on PR titles by `pr-conventional-commits-validation.yml`): `feat`, `fix`, `docs`, `test`,
  `ci`, `refactor`, `perf`, `chore`, `revert`. `build:` and `style:` are rejected; use `chore:` for dependency, Nix
  and formatting-only changes. A scope is optional and is usually a crate or file, e.g. `refactor(flake.nix): ...`,
  `fix(gun): ...`.
- No issue numbers anywhere in the message, and no session links. A `Co-Authored-By:` trailer is fine.
- Never amend or rebase commits that are already pushed, and never force-push, without explicit approval.

## 3. Before Pushing

Run the checks from [`verify`](../verify/SKILL.md) for the change, at least:

- `nix build .#checks.x86_64-linux.treefmt`
- `cargo xtask` (for any Rust or Cargo change)
- `cargo clippy --workspace --all-targets -- --deny warnings` (for any Rust or Cargo change)

## 4. Pull Request

Create it with `gh pr create --base main`. The title follows the same Conventional Commits rules as commits; it
becomes the squash-merge commit on `main`.

Body template:

```markdown
## Purposes

- Why this change is needed.

## Changes

- What changed, per crate or file.

## Verification

- `cargo xtask`: passed
- `nix build .#checks.x86_64-linux.treefmt`: passed
- Not verified: in-game behaviour (needs a play-test)

## Breaking Changes

- Only when the change breaks the config file format, save data or public crate APIs. Omit otherwise.

Closes #N
```

- Link issues only here, with a closing keyword (`Closes #N`, `Fixes #N`), and only issues the PR really resolves.
- Write the body in English.

## 5. After Creating the PR or an Issue

1. **Labels**: The PR title workflow applies type labels. Check with `gh pr view <N> --json labels`. If the type label
   is missing and exists in the repository (`gh label list`), add it: `gh pr edit <N> --add-label <type>`. Do not
   create new labels without asking.
1. **GitHub Project**: Register the item in "Spacerobo project" (#13) and set `Priority`, `Size` and `Estimate`. Follow
   [`github-projects`](../github-projects/SKILL.md).
1. **Milestones**: Never set one unless asked.
1. **CI**: Check `gh pr checks <N>`. Report failures with the log excerpt; do not re-run or bypass them silently.

## 6. Never Merge

Never run `gh pr merge` (including `--auto`) or merge into `main` locally. Leave the PR open for the maintainer.
