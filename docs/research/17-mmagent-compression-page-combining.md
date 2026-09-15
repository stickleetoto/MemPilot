# 17 — MMAgent: Memory Compression & Page Combining

## Mechanism

Windows MMAgent exposes controls/status around memory compression, page combining, application launch prefetching/prelaunch, and related memory-management features.

## Why MemPilot cares

These are **Windows-owned mechanisms** that may already provide the optimization users think MemPilot should reimplement.

## Product stance

MemPilot should begin as a diagnostic surface:

```text
Memory Compression: enabled/disabled
Page Combining: enabled/disabled
Application Launch Prefetching: enabled/disabled
```

Do not silently toggle system-wide features based on folklore.

## Research questions

- What is the CPU/latency tradeoff of compression under 8/16/32 GB pressure?
- When does page combining produce measurable savings?
- Do user workloads already receive sufficient benefit from defaults?
- Are toggles persistent/reboot-sensitive and therefore too invasive for automatic policy?

## Experiment

Benchmark Windows defaults first, then controlled on/off configurations in a disposable test environment.

## Status

**Priority: medium research, low default-mutation priority.**
