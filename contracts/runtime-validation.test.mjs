import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

import { validate } from "../generated/typescript/validate.mjs";

const manifest = JSON.parse(readFileSync("conformance/manifest.json", "utf8"));

for (const fixture of manifest.cases) {
  test(`TypeScript runtime: ${fixture.file}`, () => {
    const value = JSON.parse(
      readFileSync(`conformance/cases/${fixture.file}`, "utf8"),
    );
    const result = validate(fixture.model, value);
    assert.equal(result.ok, fixture.valid, JSON.stringify(result));
  });
}

test("unknown model names fail closed", () => {
  assert.deepEqual(validate("InternalGrant", {}), {
    ok: false,
    errors: ["unknown model InternalGrant"],
  });
});

