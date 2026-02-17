# Phase 1 Alert Ingestion Slice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Phase:** 1  
**Goal:** Add alert-ingestion parsing primitives that accept PagerDuty and Prometheus payloads for upcoming webhook handling.  
**Architecture:** Implement a runtime alert parser that converts provider-specific webhook JSON into a normalized enum, with robust defaults for optional fields and clear errors for unsupported shapes. Keep the API small (`parse_alert_payload`) and validated by focused unit tests.  
**Tech Stack:** Rust (`serde_json`)  
**Requirement IDs:** INFRA-05

---

## Implemented Tasks

1. Added `oax-runtime::alert` module and exported it via `oax-runtime/src/lib.rs`.
2. Added `AlertPayload` enum with normalized variants:
- `PagerDuty { incident_id, severity, summary }`
- `Prometheus { alertname, severity, summary }`
3. Implemented `parse_alert_payload(&str) -> Result<AlertPayload, String>` supporting:
- PagerDuty payload shape under `data.incident`
- Prometheus payload shape under `alerts[0]`
- explicit unsupported-shape error handling
4. Added/extended tests for:
- PagerDuty parsing
- Prometheus parsing
- unknown payload rejection
- Prometheus payload without `annotations`
- Prometheus severity fallback to top-level `status`

## Verification

- `cargo test -p oax-runtime alert::tests::falls_back_to_prometheus_status_for_severity -- --exact`
- `cargo test -p oax-runtime`
- `cargo test --workspace`

## Definition of Done

`Code merged + Tests green + Review pass + Docs updated + Social drafts created + Traceability updated`
