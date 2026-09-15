# 06 — Memory Priority Governor

## Mechanism

Windows exposes process/thread memory priority through `GetProcessInformation` / `SetProcessInformation` and `MEMORY_PRIORITY_INFORMATION`. Documented priorities range from very low to normal. Lower-priority pages are preferred for trimming when other factors are equal.

## Why MemPilot cares

This is the most attractive first mutation because it is a **hint to the Windows Memory Manager**, not a forced page eviction.

## Proposed policy

- foreground/recently-active: never demote;
- warm background: keep normal;
- cold background under sustained pressure: temporary below-normal/low;
- restore original value after TTL, foreground transition, pressure recovery, or MemPilot shutdown.

## Safety requirements

- record original priority before changing it;
- maintain an idempotent restore map keyed by stable process identity;
- refuse to chase access-denied processes;
- rate-limit mutations;
- never silently make permanent priority changes.

## Experiment

A/B test Windows default versus temporary background demotion during memory pressure. Measure foreground latency, hard faults, reclaimed residency, and background recovery cost.

## Status

**Priority: critical.** Candidate for first active-governor feature after benchmark acceptance.
