# MemPilot Architecture

## Core idea

MemPilot is a feedback-controlled Windows memory policy engine. It should intervene only when measured system pressure makes intervention worthwhile.

```text
Windows metrics
     |
     v
+-----------+      +-----------------+
| Telemetry |----->| Pressure Engine |
+-----------+      +-----------------+
     |                     |
     |                     v
     |              +---------------+
     +------------->| Policy Engine |
                    +---------------+
                           |
                           v
                    +---------------+
                    |   Optimizer   |
                    +---------------+
                           |
                           v
                    Windows actions
                           |
                           v
                    +---------------+
                    | Feedback Loop |
                    +---------------+
                           |
                    backoff / retain
```

## Components

### Telemetry

Responsibilities:

- system physical-memory totals and available memory;
- system commit charge and commit limit;
- hard-fault activity;
- per-process working set, private working set and commit usage;
- process activity/foreground state;
- later: working-set change observation and intervention outcome metrics.

The first live Windows backend is expected to use documented Win32 APIs. Platform-specific code should remain behind the telemetry boundary so scoring and policy code can be unit-tested independently.

### Pressure Engine

Consumes snapshots and produces:

- a normalized `0..100` score;
- a pressure class: `Normal`, `Watch`, `Pressure`, `High`, or `Critical`.

The initial formula is a provisional heuristic, not a performance claim. All coefficients and thresholds must be benchmarked against real workloads before a stable release.

### Process Profiler

Planned responsibility:

- rank processes by likely reclaim value;
- distinguish foreground/active workloads from cold background processes;
- exclude protected/system-critical processes;
- maintain cooldown/history after an intervention;
- penalize candidates that immediately fault their memory back in.

### Policy Engine

Escalation order:

1. observe;
2. lower memory priority of safe background candidates;
3. selectively trim cold candidates;
4. in critical pressure, prioritize keeping the active workload responsive.

The policy engine chooses *what should happen*. It must not call Win32 mutation APIs directly.

### Optimizer

Owns the action boundary. Future mutating actions belong here, including:

- process memory-priority changes;
- selective working-set trimming;
- cooldown/backoff tracking;
- optional advanced policies added after validation.

## Safety invariants

These rules are architectural, not optional UI preferences.

1. **Foreground protection** — the active workload is protected by default.
2. **Dry-run first** — every new mutation path should be observable without applying it.
3. **No blind cache purge** — standby/file cache is not treated as wasted RAM.
4. **No page-file disabling** — commit capacity must not be reduced as a generic optimization.
5. **No kill policy in v0.1** — MemPilot does not terminate applications to manufacture free memory.
6. **Cooldown after mutation** — a process cannot be repeatedly trimmed in a tight loop.
7. **Feedback required** — intervention effectiveness must be measured using reclaim, refault and latency signals.
8. **Back off on regression** — a candidate that rapidly reloads memory becomes less eligible for future intervention.
9. **Documented APIs first** — avoid undocumented kernel manipulation in the baseline product.
10. **Fail passive** — telemetry/API failures should normally result in no mutation.

## v0.1 execution model

The initial implementation should be a user-run CLI rather than an always-on privileged service.

```text
mempilot status
mempilot watch
mempilot analyze
mempilot optimize --dry-run
```

A background service/tray component should only be considered after the policy engine is measurable and stable.

## Validation metrics

A successful optimization is not defined by "RAM used went down". Useful measurements include:

- physical memory made available;
- commit pressure before/after;
- hard faults after intervention;
- memory reloaded by the target process;
- foreground frame-time/latency regression;
- intervention frequency;
- amount reclaimed per unit of observed penalty.

The long-term target is a policy that improves behavior under memory pressure without degrading normal workloads.
