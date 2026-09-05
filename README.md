# hhm-pub-lib-core

Public, client-safe contracts and runtime adapters for Hacker House Medellin applications.

This repository owns only public additions: client installation metadata, idempotency keys,
the public location catalog, and a minimal account context that distinguishes individual and
organization use. Reservation, stay, room, access-control, camera, billing, and organization
membership authorities remain in `hacker-house-medellin/hhm-interfaces`.

## Independent authorities

`contracts/typespec/main.tsp` and `contracts/json-schema/contract.schema.json` are separately
human-authored peers. Neither file is generated from the other. `ores-contracts` parses both,
compares their normalized models, emits each lane independently, and refuses generation unless
the lanes and all generated artifacts agree.

The generator is pinned to immutable commit
`b7c639c980fe35c681fab3c33b2b8f3ee9ae5f1c`. CI invokes the CLI implementation directly and
asserts its receipt so a broken package bin cannot create a false-positive green build.

```sh
npm ci --ignore-scripts
npm run contracts:test
npm run generated:verify
cargo test --locked --manifest-path runtime/rust/Cargo.toml
dart pub get --directory runtime/dart --enforce-lockfile
dart analyze runtime/dart
dart test runtime/dart
```

Generated shapes are necessary but do not replace semantic validation. The TypeScript, Rust,
and Dart adapters add client-safe length and unknown-field checks and share the cases under
`conformance/cases/`.

Public client-safe contracts and SDK core for Hacker House Medellín
