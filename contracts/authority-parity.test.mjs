import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

import { compare } from "../node_modules/@oresoftware/ores-contracts/src/ir.mjs";
import { parseJsonSchema } from "../node_modules/@oresoftware/ores-contracts/src/parse-json-schema.mjs";
import { parseTypeSpec } from "../node_modules/@oresoftware/ores-contracts/src/parse-typespec.mjs";

const typeSpecText = readFileSync("contracts/typespec/main.tsp", "utf8");
const jsonSchemaText = readFileSync("contracts/json-schema/contract.schema.json", "utf8");
const typeSpec = parseTypeSpec(typeSpecText, "contracts/typespec/main.tsp");
const jsonSchemaDocument = JSON.parse(jsonSchemaText);

function differences(mutator) {
  const changed = structuredClone(jsonSchemaDocument);
  mutator(changed);
  return compare(
    typeSpec,
    parseJsonSchema(changed, "contracts/json-schema/contract.schema.json"),
  );
}

test("independent authorities normalize to the same model", () => {
  const jsonSchema = parseJsonSchema(
    jsonSchemaDocument,
    "contracts/json-schema/contract.schema.json",
  );
  assert.deepEqual(compare(typeSpec, jsonSchema), []);
});

for (const [name, mutator] of [
  ["enum wire value", (document) => { document.$defs.ClientPlatform.enum[0] = "web"; }],
  ["required field", (document) => { document.$defs.ClientInfo.required = ["installId", "platform"]; }],
  ["maximum length", (document) => { document.$defs.ClientInfo.properties.appVersion.maxLength = 63; }],
  ["scalar type", (document) => { document.$defs.IdempotencyKey.properties.mintedAtMs.type = "number"; }],
  ["primary key", (document) => { document.$defs.ClientInfo["x-ores-primary-key"] = ["appVersion"]; }],
]) {
  test(`one-sided ${name} drift is rejected`, () => {
    assert.ok(differences(mutator).length > 0);
  });
}

test("receipt proves both lanes, compiler execution, and artifact parity", () => {
  const receipt = JSON.parse(
    readFileSync("target/ores-contracts/receipt.json", "utf8"),
  );
  assert.equal(receipt.status, "passed");
  assert.equal(receipt.authorities.typespec.tspCompile, "ok");
  assert.ok(receipt.authorities.typespec.sha256);
  assert.ok(receipt.authorities["json-schema"].sha256);
  assert.equal(Object.keys(receipt.artifacts).length, 4);
  assert.ok(Object.values(receipt.artifacts).every((artifact) => artifact.byteParity));
});

