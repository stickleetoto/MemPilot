# 09 — ETW System Memory Provider

## Mechanism

The Windows System Memory Provider exposes ETW keywords for general memory activity, hard faults, all faults, pool, memory info, working sets, VirtualAlloc, footprint, reference sets, VA maps, and related memory-manager events.

## Why MemPilot cares

ETW is the path from coarse counters to **why pressure happened**. It can attribute memory events to processes and time windows, enabling postmortem validation and richer `diagnose` mode.

## Architecture

Keep ETW out of the always-on fast path initially.

```text
normal loop: snapshot APIs + PDH
explicit diagnose: ETW session -> event aggregation -> report
benchmark: WPR/WPA + optional MemPilot ETW collector
```

## Risks

- schema/version complexity;
- trace volume;
- privilege/session constraints;
- easy to over-collect and increase overhead;
- event correlation requires careful timestamps/process identity.

## Experiment

Build a minimal collector for hard-fault and memory-info keywords. Compare event counts against WPA on the same ETL trace.

## Status

**Priority: high for v0.3 diagnostics**, not yet default runtime telemetry.
