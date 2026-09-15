# MemPilot Roadmap

## v0.1 — Measurable baseline

Goal: build a safe Windows memory-pressure observer and dry-run policy engine before enabling meaningful optimization actions.

### Phase 0 — Foundation

- [x] Rust project skeleton
- [x] pressure-level model
- [x] policy/action boundary
- [x] architecture and safety invariants
- [x] Windows CI baseline
- [ ] choose/freeze the Win32 binding surface for v0.1

### Phase 1 — Live system telemetry

- [ ] physical memory totals / available memory
- [ ] commit total / commit limit
- [ ] system cache visibility
- [ ] low-memory notification integration
- [ ] hard-fault sampling
- [ ] periodic snapshot loop
- [ ] `mempilot status` with live values
- [ ] `mempilot watch` streaming mode

Exit condition: MemPilot can observe pressure continuously without mutating the system.

### Phase 2 — Process profiler

- [ ] enumerate eligible user processes
- [ ] working-set usage
- [ ] private working-set usage
- [ ] private/shared commit where available
- [ ] page-fault deltas
- [ ] foreground-process detection
- [ ] protected-process filtering
- [ ] idle/cold candidate scoring
- [ ] `mempilot analyze`

Exit condition: candidate rankings are explainable and testable.

### Phase 3 — Dry-run policy engine

- [ ] candidate allowlist/denylist
- [ ] cooldown model
- [ ] pressure-to-action policy
- [ ] explain every proposed action
- [ ] `mempilot optimize --dry-run`
- [ ] before/after metric schema

Exit condition: MemPilot can say what it *would* do and why, with no system mutation.

### Phase 4 — Low-risk intervention

- [ ] background process memory-priority adjustment
- [ ] automatic foreground protection
- [ ] restore/default-priority path
- [ ] action audit log
- [ ] regression/backoff rules

Exit condition: policy changes can be applied and reversed safely.

### Phase 5 — Selective working-set trim

- [ ] explicit trim eligibility rules
- [ ] minimum pressure threshold
- [ ] minimum cold-time threshold
- [ ] per-process cooldown
- [ ] reclaim measurement
- [ ] refault/reload penalty
- [ ] automatic backoff for bad candidates

Exit condition: trimming demonstrates measurable benefit on constrained-memory test workloads without repeat-thrashing.

## v0.2 — Adaptive feedback

- [ ] working-set change observation
- [ ] per-process intervention history
- [ ] adaptive candidate score
- [ ] game/foreground workload mode
- [ ] persistent local configuration
- [ ] benchmark harness
- [ ] reproducible 8 GB / 16 GB pressure scenarios
- [ ] exportable metrics (JSON/CSV)

## v0.3 — Advanced controls

Candidates only after v0.1/v0.2 evidence:

- [ ] Job Object memory budgets for MemPilot-launched workloads
- [ ] application recovery/prefetch experiments
- [ ] optional tray UI
- [ ] optional background service
- [ ] profile system for gaming/build/VM workloads

## Explicitly deferred

These are not baseline features:

- global standby-list purging;
- arbitrary system-file-cache limits;
- automatic page-file modification;
- undocumented kernel hooks/drivers;
- application termination as a routine optimization method.

They require separate evidence and safety review before consideration.

## Release rule

A feature does not graduate from experimental to default-on merely because it frees memory. It must show that the reclaimed memory is useful and that the resulting refault/latency cost is acceptable.
