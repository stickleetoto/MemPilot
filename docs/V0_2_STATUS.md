# MemPilot v0.2 implementation status

MemPilot v0.2 is intentionally a **safe, observation-first milestone**.

It implements enough of the planned architecture to collect real Windows memory data, classify pressure, inspect visible processes, rank reclaim candidates, and run an adaptive dry-run loop. It does **not** yet change another process's memory policy or trim working sets.

## Implemented

### System telemetry

- `GlobalMemoryStatusEx`
- `K32GetPerformanceInfo`
- physical total / available memory
- commit total / limit
- system cache
- paged / nonpaged kernel pool
- Windows low-memory resource notification state
- timestamped snapshots

### Process telemetry

- process enumeration with `K32EnumProcesses`
- process path/name through `QueryFullProcessImageNameW`
- working set / peak working set
- private commit
- cumulative page-fault counter
- PID + creation-time process identity
- foreground-process protection
- inaccessible/protected processes are skipped rather than treated as fatal

### Pressure engine

- 0..100 pressure score
- physical + commit pressure
- low-memory signal override
- smoothing
- hysteresis
- `NORMAL / WATCH / PRESSURE / HIGH / CRITICAL`

### Adaptive dry-run policy

- per-process sample deltas
- page-fault-delta risk signal
- working-set stability signal
- in-run historical fault penalty by executable
- deterministic candidate ranking
- protected Windows-process exclusions
- foreground exclusion
- expected reclaim estimate
- dry-run plan serialization

### CLI

```text
mempilot status [--json]
mempilot analyze [--top N] [--json]
mempilot watch [--interval S] [--count N]
mempilot optimize --dry-run [--top N] [--json]
mempilot version
```

## v0.2 safety boundary

The following are deliberately **not enabled**:

- `SetProcessInformation(ProcessMemoryPriority)`
- `EmptyWorkingSet`
- `SetProcessWorkingSetSizeEx`
- standby-list purge
- system file-cache purge
- page-file mutation
- process termination

`mempilot optimize` refuses to run unless `--dry-run` is supplied.

This keeps the first functional release measurable and reversible: we can validate telemetry and candidate quality before enabling any system mutation.

## Known gaps before active optimization

- system-wide hard-fault / paging-I/O rate via PDH or ETW
- CPU activity deltas
- private working set (distinct from private commit)
- current process memory priority
- persistent on-disk intervention history
- allowlist/denylist configuration
- benchmark evidence for memory-priority demotion
- benchmark evidence for selective trimming

These are the next gates before MemPilot should become an active governor.
