# Getting Started

This guide covers local setup for the OpsClaw Phase 0 baseline.

## 1. Verify docker-compose stack

```bash
docker compose -f docker/docker-compose.yaml config
docker compose -f docker/docker-compose.yaml up -d
```

## 2. Build and verify single-binary installer locally

```bash
bash scripts/install/verify-local-install.sh
```

This validates packaging and install behavior without any network dependency.

## 3. Install from release script (curl one-liner)

```bash
curl -fsSL https://github.com/opsclawhq/opsclaw/raw/main/scripts/install/install-opsclaw.sh | bash
```

## 4. Phase 2 Preview Command

Install a local skill markdown file into `~/.opsclaw/skills/`:

`opsclaw skill install /absolute/path/to/skill.md`
