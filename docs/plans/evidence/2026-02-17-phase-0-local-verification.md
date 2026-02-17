# Phase 0 Local Verification Evidence

Date: 2026-02-17  
Plan: `docs/plans/2026-02-17-phase-0-workspace-foundations.md`

## RED -> GREEN Proofs

1. Workspace check (RED): `cargo check --workspace` failed before `Cargo.toml` existed.
2. Workspace check (GREEN): `cargo check --workspace` passed after workspace scaffold creation.
3. Workspace tests: `cargo test --workspace` passed.
4. Types pipeline (RED): `bash scripts/generate-types.sh` failed before script creation.
5. Types pipeline (GREEN): `bash scripts/generate-types.sh` passed and generated `packages/sdk/src/generated/types.ts`.
6. Types sync: `bash scripts/generate-types.sh && git diff --exit-code -- packages/sdk/src/generated/types.ts` passed.
7. Docker compose (RED): `docker compose -f docker/docker-compose.yaml config` failed before file creation.
8. Docker compose (GREEN): `docker compose -f docker/docker-compose.yaml config` passed after compose setup.
9. Dev stack health: `docker compose -f docker/docker-compose.yaml up -d && docker compose -f docker/docker-compose.yaml ps` showed PostgreSQL, Redis, and NATS healthy.
10. Binary version check: `cargo run -p opsclaw -- --version` printed `opsclaw 0.1.0`.
11. Docs skeleton (RED): required docs file checks failed before creation.
12. Docs skeleton (GREEN): required docs file checks passed after creation.

## Verification Commands Re-run

```bash
cargo check --workspace
cargo test --workspace
bash scripts/generate-types.sh && git diff --exit-code -- packages/sdk/src/generated/types.ts
docker compose -f docker/docker-compose.yaml config
docker compose -f docker/docker-compose.yaml ps
cargo run -p opsclaw -- --version
```

All commands passed in local verification.
