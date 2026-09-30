---
name: github-projects
description: Add spacerobo issues and pull requests to the "Spacerobo project" GitHub Project (#13) and set its Priority, Size, Estimate and Status fields with gh project. Use after creating an issue or PR, or when asked to update project fields.
---

# GitHub Projects Workflow (`gh project`)

The repository's items belong to **"Spacerobo project"**, project number `13`, owner `haruki7049`
(`https://github.com/users/haruki7049/projects/13`).

## 1. Inspect Before Editing

Never guess field names or option values. Read the schema first:

```bash
gh project field-list 13 --owner haruki7049 --format json \
  --jq '.fields[] | {name, type, options: [.options[]?.name]}'
```

At the time of writing the custom fields are:

| Field | Type | Values |
| :--- | :--- | :--- |
| `Status` | single select | `Backlog`, `Ready`, `In progress`, `In review`, `Done` |
| `Priority` | single select | `P0`, `P1`, `P2` |
| `Size` | single select | `XS`, `S`, `M`, `L`, `XL` |
| `Estimate` | number | e.g. `1`, `2`, `3`, `5`, `8` |

If the live schema differs, follow the live schema and tell the user this table is outdated.

Guidance for values:

- `Priority`: `P0` build breakage or a crash that blocks playing; `P1` gameplay features and important refactoring;
  `P2` documentation, minor fixes and maintenance.
- `Size` and `Estimate`: relative effort. Keep them consistent with each other.

## 2. Add the Item

Check whether the item is already in the project (it may be added automatically):

```bash
gh pr view <N> --json projectItems      # or: gh issue view <N> --json projectItems
```

If not, add it:

```bash
gh project item-add 13 --owner haruki7049 --url <ISSUE_OR_PR_URL>
```

## 3. Set Fields: One Field per Command

`gh project item-edit` updates **only one field per invocation**. Never pass several `--field`/`--value` pairs in one
command: only the last one is applied and the others are dropped without an error. Run one command per field:

```bash
URL="https://github.com/haruki7049/spacerobo/pull/<N>"   # or .../issues/<N>

gh project item-edit 13 --owner haruki7049 --url "$URL" --field "Priority" --value "P1"
gh project item-edit 13 --owner haruki7049 --url "$URL" --field "Size" --value "M"
gh project item-edit 13 --owner haruki7049 --url "$URL" --field "Estimate" --value "3"
```

Selecting the item and field by name (`--url`, `--field`, `--value`) needs a recent `gh` (2.101.0 has it). If the
installed version rejects these flags, read `gh project item-edit --help` and use the ID-based flags it documents.

Leave `Status` alone unless the user asks; the project's workflows may manage it.

## 4. Confirm

```bash
gh project item-list 13 --owner haruki7049 --limit 500 --format json \
  --jq ".items[] | select(.content.url == \"$URL\")"
```

Report the values that were set. Never set a milestone unless asked.
