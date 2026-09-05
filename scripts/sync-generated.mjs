import { copyFileSync, mkdirSync } from "node:fs";
import { dirname } from "node:path";

const copies = [
  ["generated/rust/types.rs", "runtime/rust/src/generated.rs"],
  ["generated/dart/models.dart", "runtime/dart/lib/src/generated/models.dart"],
];

for (const [source, destination] of copies) {
  mkdirSync(dirname(destination), { recursive: true });
  copyFileSync(source, destination);
}

