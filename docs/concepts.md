# Concepts

OpsClaw uses phase-gated delivery, requirement traceability, and evidence-based quality gates.

## Persistent Memory (Phase 1)

Each agent keeps memory entries as key/value pairs, scoped by agent ID. The Phase 1 runtime baseline persists this memory in a local JSON file so learned context survives process restarts.

Current runtime contract:

- `JsonFileMemoryStore::new(path)` loads existing memory or starts empty.
- `put(agent_id, key, value)` updates memory in-process.
- `save()` flushes to disk.
- Reloading from the same path restores previously saved values.
