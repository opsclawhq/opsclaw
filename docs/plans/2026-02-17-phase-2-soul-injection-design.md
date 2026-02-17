# Phase 2 SOUL Injection Design

## Scope
Implement runtime prompt assembly that injects SOUL identity metadata and SOUL system instructions into the final system prompt used by agents.

## Options Considered
1. Compose prompt ad-hoc at each caller site.
2. Central runtime composer function (recommended).
3. Store prompt templates as separate files and render dynamically.

## Selected Approach
Use a central runtime composer module (`oax-runtime::prompt`) with deterministic formatting. This keeps identity injection consistent and testable while remaining lightweight.

## Data Flow
1. Load SOUL profile from markdown file (`oax_core::soul::load_soul_file`) or receive typed `SoulProfile` directly.
2. Build a system prompt containing:
- identity fields (name, role, personality, communication style, avatar)
- base runtime instructions
- SOUL-specific system prompt body
3. Return final prompt string for model invocation.

## Failure Modes
- Missing or malformed SOUL markdown: return explicit error.
- Empty base instructions: still compose prompt with identity + SOUL section.

## Test Strategy
- Verify composed prompt includes identity and base instructions.
- Verify file-based composition path loads SOUL and injects content.
- Verify two distinct presets generate non-identical prompt outputs.
