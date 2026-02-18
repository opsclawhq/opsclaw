# Verification Evidence: Phase 4 Economics + Conversation Viewer Slice

## RED (expected failure before view-model implementation)

Command:

```bash
npm --prefix packages/dashboard test
```

Result:

- failed with `ERR_MODULE_NOT_FOUND` for `packages/dashboard/src/lib/economics-conversation.mjs` from `tests/economics-conversation.test.mjs`.

## GREEN (dashboard helper tests)

Command:

```bash
npm --prefix packages/dashboard test
```

Result:

- `12 passed; 0 failed`.
- includes economics row mapping, ROI summary math, transcript sorting, and missing-conversation behavior.

## Full Verification

Commands:

```bash
npm --prefix packages/dashboard test
cargo test --workspace
cargo clippy --workspace --all-targets
```

Result:

- dashboard tests passed (`12 passed; 0 failed`)
- workspace tests passed across all crates
- clippy exited `0`
