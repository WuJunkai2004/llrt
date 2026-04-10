# WebAssembly Bridge Completion Requirements

## Scope

- Keep current registration behavior unchanged.
- Complete the concrete behavior paths for the 5 exported bridge symbols:
  - `llrt_wasm_validate`
  - `llrt_wasm_compile`
  - `llrt_wasm_instantiate`
  - `llrt_wasm_compile_streaming`
  - `llrt_wasm_instantiate_streaming`
- Add tests that verify functional behavior (not just symbol existence).

## Style and Quality Requirements

- Follow existing LLRT Rust and TypeScript code style.
- Keep changes minimal and focused on WebAssembly bridge/runtime behavior.
- Avoid introducing unrelated refactors.
- Ensure all added unsafe code is documented with `SAFETY` comments.

## Validation Requirements

- Validate runtime bridge behavior with Rust tests in `llrt_webassembly_runtime`.
- Validate JS-facing behavior through LLRT unit tests in `tests/unit/webassembly.test.ts`.
- Run repository checks required by LLRT contribution flow:
  - `make check`
  - Relevant tests for changed areas

## Phased Execution Plan

1. Add this requirements document and stage the implementation plan.
2. Implement the 5-symbol behavior paths in `libs/llrt_webassembly_runtime`.
3. Add Rust tests for bridge symbol behavior (valid and invalid wasm bytes).
4. Add/extend LLRT JS unit tests to verify the 5 WebAssembly APIs functionally.
5. Run checks/tests and make final fixes.
