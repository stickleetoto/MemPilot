# 20 — Evidence-Driven Benchmark Harness

## Goal

MemPilot must prove useful reclaim **without moving the cost somewhere else**.

## Test arms

At minimum compare:

```text
A: Windows default
B: MemPilot observe-only
C: MemPilot memory-priority policy
D: MemPilot priority + selective trim
```

Mutation arms exist only after the relevant safety gate is implemented.

## Workloads

- idle desktop;
- browser-heavy;
- game + browser + chat app;
- IDE/build/compile;
- large file-cache workload;
- synthetic private-commit pressure;
- synthetic physical pressure;
- repeated foreground switching;
- 8 GB, 16 GB, 32 GB+ machines/profiles.

## Metrics

- MemPilot CPU and memory overhead;
- physical available / commit headroom;
- useful reclaim persistence;
- process/system page faults;
- hard-fault I/O time;
- disk read/write amplification;
- foreground latency proxy;
- policy reversals/backoffs;
- crash/error/allocation-failure events.

## Statistics

Use repeated runs, medians and percentiles, not a single screenshot. Version the workload, configuration, Windows build, hardware class, and MemPilot commit.

## External validation

Use WPR/WPA traces to verify MemPilot's own counters and conclusions. A feature should not graduate to default-on solely because MemPilot's internal score says it worked.

## Status

**Priority: critical.** This is the release gate for every active optimization technology.
