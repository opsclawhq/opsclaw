# KPI Telemetry Hardening Design

## Scope
Add operational tooling to validate planning metadata contracts and audit KPI snapshot quality in GitHub Project gate tracking.

## Requirement Coverage
- Roadmap quality policy: no gate closure without KPI snapshot evidence.
- Roadmap phase gate checklist item: KPI snapshot is recorded for gate decision.

## Selected Approach
1. add local metadata validator script for `.planning/contracts/*.metadata.json`
2. add GitHub Project KPI audit script for passed/ready items
3. add contributor docs showing strict vs placeholder modes and recommended gate workflow

## Failure Modes Addressed
- malformed metadata JSON silently accepted
- gate marked passed while KPI snapshot is missing/invalid/placeholder
- inconsistent manual gate checks across sessions

## Verification Strategy
- RED: metadata validator script missing
- GREEN: validators run in permissive mode on current repo/project state
- STRICT mode: fails when placeholder KPI values remain
- Full: release-doc validation + workspace test/clippy remain green
