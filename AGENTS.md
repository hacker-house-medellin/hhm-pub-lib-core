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

