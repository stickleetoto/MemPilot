# 04 — QueryWorkingSetEx

## Mechanism

`QueryWorkingSetEx` returns extended information for selected virtual addresses in a process address space. Unlike a coarse process counter, it can reveal page-level properties and can query relevant addresses outside the current working set in some cases.

## Why MemPilot cares

This enables targeted deep diagnostics: shared/private composition, large-page/AWE awareness, and page residency characteristics can explain why a process that looks large is actually a poor reclaim target.

## Integration idea

Do **not** place page-level scans in the normal sampling loop. Use them only in an explicit diagnostic mode or on a small sample of candidate address ranges.

Possible command:

```text
mempilot diagnose ws --pid <PID> --sample <N>
```

## Risks

- page-level enumeration can be expensive;
- process access can fail legitimately;
- snapshots become stale immediately;
- interpreting page metadata incorrectly can lead to fake precision.

## Experiment

Sample a handful of known workloads and measure diagnostic overhead, runtime, and whether page-level data changes candidate ranking compared with EX2 counters alone.

## Status

**Priority: medium.** Deep-diagnostics feature, not core-loop telemetry.
