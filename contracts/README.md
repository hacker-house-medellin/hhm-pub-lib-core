# Contract authority and acceptance gate

The TypeSpec and JSON Schema trees are independent review surfaces. A reviewer must update and
understand both. Generation is allowed only after normalized structure and all emitted artifacts
agree.

The acceptance gate proves:

1. the official TypeSpec compiler ran successfully;
2. both authority digests are present;
3. normalized authority comparison found no differences;
4. all four Rust, TypeScript, and Dart artifacts have byte parity;
5. committed generated files match a fresh generation; and
6. language adapters accept and reject the shared conformance cases.

The generator currently emits structural shapes. Adapter code remains responsible for semantic
constraints such as maximum string lengths and strict unknown-field handling in runtimes whose
generated decoder is permissive.

