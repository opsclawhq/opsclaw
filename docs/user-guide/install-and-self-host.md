# Install and Self-Host

OpsClaw is self-hosted. Runtime services and API keys stay on infrastructure you control.

## 1. Bring up local dependencies

```bash
docker compose -f docker/docker-compose.yaml up -d
```

Services started:
- PostgreSQL
- Redis
- NATS

## 2. Build package and verify local installer

```bash
bash scripts/install/verify-local-install.sh
```

This command:
1. Builds `opsclaw` release binary tarball.
2. Installs it into a temporary directory from local archive.
3. Verifies `opsclaw --version`.

## 3. Install from release artifact (curl one-liner)

Latest release:

```bash
curl -fsSL https://github.com/opsclawhq/opsclaw/raw/main/scripts/install/install-opsclaw.sh | bash
```

Specific version and directory:

```bash
OPSCLAW_VERSION=0.1.0 OPSCLAW_INSTALL_DIR="$HOME/.local/bin" \
  curl -fsSL https://github.com/opsclawhq/opsclaw/raw/main/scripts/install/install-opsclaw.sh | bash
```

After install:

```bash
opsclaw --version
```
