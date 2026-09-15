# 08 — PDH / Windows Memory Performance Counters

## Mechanism

Windows performance counters expose rate-oriented memory signals such as `Memory\Pages/sec`, `Page Reads/sec`, `Page Inputs/sec`, `Page Writes/sec`, and `Page Output/sec`.

## Why MemPilot cares

Snapshot APIs tell MemPilot **how much memory exists now**. Rate counters tell it **how violently the system is moving pages**. That distinction helps separate benign high RAM usage from active contention.

## Integration idea

Create a small PDH sampler with counter warm-up and interval-aware rate calculations. Feed normalized rates into the pressure engine but keep raw values in diagnostics/JSON.

## Interpretation rule

Hard page faults may be backed by pagefile, mapped files, DLLs, or executables. Do not label every `Pages Input/sec` event as pagefile thrashing.

## Experiment

Calibrate idle, browser-heavy, compile, game, synthetic commit-pressure, and physical-pressure scenarios. Determine p50/p95 rates per workload and identify false alarms.

## Status

**Priority: critical.** Best next addition to v0.2's snapshot-only telemetry.
