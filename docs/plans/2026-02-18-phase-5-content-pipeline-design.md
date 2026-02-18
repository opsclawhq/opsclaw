# Phase 5 Build-in-Public Content Pipeline (05-05) Design

## Scope
Implement deterministic content pipeline tooling and agent-readable docs indexing for phase-by-phase publishing outputs.

1. per-slice content generation script (LinkedIn/X + blog drafts)
2. machine-readable docs index for LLM/agent navigation
3. docs/blog discoverability updates tied to pipeline outputs

## Requirement Coverage
- PUB-04
- PUB-07

## Options Considered

1. Keep manual content drafting only in local notes.
2. Add deterministic script + structured docs index (recommended).
3. Defer content pipeline until final release.

## Selected Approach
Option 2. A deterministic script enforces repeatable outputs after each merged slice, while a structured docs index keeps documentation both human-readable and agent-readable.

## Architecture

- Add `scripts/content/phase-delivery-pipeline.sh`:
  - inputs: phase, slice, date, requirements, PR URL, summary
  - outputs: phase social drafts in `.content/phase-<n>/`
  - outputs: tracked blog drafts in `docs/blog/`
  - outputs: machine-readable delivery manifest in `docs/blog/manifests/`
- Add `docs/agent-index.yaml` as a structured docs navigation map for LLM consumption.
- Update docs indexes to link pipeline outputs and structured index artifacts.

## Failure Modes

- pipeline outputs are inconsistent across slices due to manual naming drift
- required content artifacts are missing at phase gate time
- docs remain human-readable only and lack machine-discoverable structure

## Verification Strategy

- RED: pipeline script missing before implementation
- GREEN: script generates expected files and manifest for a sample slice
- full: workspace tests and clippy remain green
