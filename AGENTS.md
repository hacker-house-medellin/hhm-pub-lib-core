# AGENTS.md

## Scope

This file governs the entire `hhm-pub-lib-core` repository.

## Ownership

- Own public/client/external SDK additions only.
- Import shared domain authorities from `hhm-interfaces`; never duplicate reservations, stays,
  rooms, access decisions, cameras, billing, grants, or organization membership here.
- Never include secrets, credentials, private provider URLs, raw camera data, biometric data,
  internal database entities, or admin-only operations.
- Keep `hhm-lib-core` as the server/domain implementation boundary and `hhm-orm-core` as the
  separate opaque persistence-operation boundary.

## Contract invariants

- TypeSpec and JSON Schema are independent, human-authored peers. Neither is generated from the
  other and neither wins a discrepancy.
- Pin the generator and compilers to immutable versions or commits.
- A contract check is green only when the standalone TypeSpec compiler succeeds, both authority
  digests exist, the receipt says `passed`, every artifact has byte parity, committed generated
  files are unchanged, and all runtime conformance tests pass.
- Generated files are never hand-edited. Run `npm run generated:update` after an approved change
  to both authorities.
- Runtime adapters must reject unknown fields and enforce constraints the generated shape alone
  cannot enforce.

## Branch and review policy

- `main` is production and `dev` is integration. Feature branches merge to `dev`; reviewed `dev`
  promotions merge to `main`.
- Never force-push, rebase shared work, bypass checks, or commit secrets.
- Stage explicit paths and preserve concurrent work.

## Repository-local Git worktrees

- Create or use a Git worktree only when the human operator explicitly authorizes it for the current task. Concurrency or a dirty checkout is not permission by itself.
- Put every authorized worktree at `<repository-root>/tmp/worktrees/<name>`; from the repository root, use `./tmp/worktrees/<name>`. Never place worktrees beside repositories or organization directories.
- Keep `tmp`, `temp`, `tmp/worktrees`, and `temp/worktrees` ignored in the repository-root `.gitignore`. Do not commit files from those directories.
- Relocate or remove a worktree only when the operator explicitly requests it. Before removal, preserve and publish intended changes, verify its commit is represented on the target branch, and confirm there are no tracked, untracked, ignored-sensitive, or in-use files that must survive. Remove it with `git worktree remove <path>` without `--force`; never delete a worktree directory with `rm`.
