# Phase 5 Docs Release Hardening + Blog Automation (05-06) Design

## Scope
Finalize Phase 5 by hardening user/developer docs release checks and adding recurring engineering blog automation workflow.

1. docs release validation script for required user/developer/blog index coverage
2. recurring engineering blog scaffolding script with deterministic naming
3. editorial workflow documentation for review, publish, and traceability

## Requirement Coverage
- PUB-05
- PUB-06
- PUB-07

## Options Considered

1. Continue manual doc checks and manual blog drafting.
2. Add deterministic scripts + explicit editorial workflow docs (recommended).
3. Defer release hardening until post-Phase 5.

## Selected Approach
Option 2. Explicit release checks reduce missing-doc regressions, and deterministic blog scaffolding operationalizes recurring cadence requirements.

## Architecture

- Add `scripts/docs/validate-release-docs.sh` to verify key docs/index/agent-map paths.
- Add `scripts/content/generate-engineering-blog.sh` to scaffold recap + design + process posts per phase.
- Add workflow docs:
  - `docs/developer-guide/docs-release-hardening.md`
  - `docs/blog/editorial-workflow.md`
- Wire guide indexes to reference new scripts and workflow docs.

## Failure Modes

- phase closes with missing or stale docs links
- blog cadence drifts due ad-hoc file naming
- contributors lack a clear review/publish workflow

## Verification Strategy

- RED: release-doc validation script absent before implementation
- GREEN: scripts execute and generate expected artifacts in temp root
- full: workspace tests and clippy remain green
