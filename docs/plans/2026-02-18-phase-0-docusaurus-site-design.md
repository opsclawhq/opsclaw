# Phase 0 Docusaurus Docs Site Foundation (00-04/00-05) Design

## Scope
Establish a first-class Docusaurus docs site so OpsClaw docs can be consumed as a navigable website while preserving the existing markdown source of truth.

1. scaffold a Docusaurus site under `packages/docs-site`
2. include core docs surfaces required by PUB-01 (Getting Started, Concepts, Skills, Architecture, API Reference)
3. include dedicated User Guide and Developer Guide tracks required by roadmap 00-05
4. ensure agent-readable navigation and indexing remain available via existing docs structure and explicit nav metadata

## Requirement Coverage
- PUB-01
- PUB-04

## Options Considered

1. Keep markdown-only docs and defer Docusaurus.
2. Build Docusaurus with docs plugin over existing `docs/` tree (recommended).
3. Move all docs into a new directory and migrate links in one large refactor.

## Selected Approach
Option 2. It minimizes disruption, keeps current docs as canonical content, and adds a deployable site layer with immediate value.

## Architecture

- Add `packages/docs-site` with:
  - `package.json` scripts for `start`, `build`, `serve`
  - `docusaurus.config.ts` with sidebar and navbar routes
  - `sidebars.ts` grouping docs into core, user guide, developer guide, and blog
  - lightweight CSS and static assets
- Wire docs source to repository `docs/` directory so existing markdown remains the single content source.
- Add a generated entry page in `docs/` for Docusaurus (`docs-site-index.md`) that links user and developer tracks explicitly.
- Update root docs readme and changelog with build/run instructions.

## Failure Modes

- site scaffolds but fails build due unresolved markdown links
- user/developer tracks exist in filesystem but are not discoverable via navigation
- docs become human-readable only and lose structured discoverability for agents

## Verification Strategy

- RED: Docusaurus config absent before implementation
- GREEN: `npm --prefix packages/docs-site run build` succeeds
- UAT: run static serve command and verify top-level nav includes User Guide + Developer Guide
- full: `cargo test --workspace` and `cargo clippy --workspace --all-targets` remain green
