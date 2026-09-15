# 03 — PROCESS_MEMORY_COUNTERS_EX2

## Mechanism

`PROCESS_MEMORY_COUNTERS_EX2` extends process memory statistics with `PrivateWorkingSetSize` and `SharedCommitUsage`, in addition to working set, private commit, peaks, pools, and page-fault count.

## Why MemPilot cares

Total working set can exaggerate reclaim value because it includes shared pages. Private working set is a better signal for memory uniquely resident for a process, while shared commit helps explain shared-resource cost.

## Compatibility

Microsoft documents EX2 support on Windows 10 22H2 / Windows 11 22H2 with the September 2023 cumulative update or later. MemPilot therefore needs runtime capability detection and fallback to `PROCESS_MEMORY_COUNTERS_EX`.

## Integration idea

Add telemetry fields:

- `private_working_set_bytes: Option<u64>`;
- `shared_commit_bytes: Option<u64>`;
- `telemetry_capability: ex | ex2`.

Candidate scoring should prefer private working set when available, but remain deterministic when it is not.

## Experiment

For browsers, Electron apps, games, IDEs, and services, compare `WorkingSetSize` versus `PrivateWorkingSetSize`. Quantify how often total working set materially overestimates candidate reclaim.

## Status

**Priority: high.** Likely v0.3 telemetry upgrade.
