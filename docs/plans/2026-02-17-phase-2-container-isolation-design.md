# Phase 2 Container Isolation Design

## Scope
Define a runtime container isolation contract with explicit policy guarantees:
- one container spec per agent
- no host networking (`network_mode=none`)
- scoped workspace mount only
- read-only root filesystem by default

## Options Considered
1. Implement full Docker orchestration now.
2. Implement policy-only typed contract and conversion layer (recommended).
3. Delay isolation work until chat adapters are added.

## Selected Approach
Implement option 2: typed isolation contract + validation + `bollard` config conversion. This provides immediate testable safety guarantees and keeps orchestration concerns separate.

## Data Flow
1. Build `AgentContainerSpec` from agent id, image, workspace root, and scoped mount settings.
2. Validate isolation constraints (network mode, mount scope).
3. Convert to `bollard::container::Config<String>` for runtime container creation path.

## Failure Modes
- host network mode or invalid mount target should fail validation
- empty agent id/image/workspace path should fail build

## Test Strategy
- spec defaults enforce `network_mode=none` and read-only root
- validation rejects host network and invalid mount targets
- conversion includes expected bind mount and host-config flags
