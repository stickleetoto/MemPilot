# 07 — Selective Working-Set Trim

## Mechanism

`EmptyWorkingSet` and `SetProcessWorkingSetSizeEx(-1, -1, ...)` can remove as many pages as possible from a process working set. Microsoft describes this primarily as useful for testing/tuning.

## Why MemPilot cares

It can reclaim resident memory immediately, but immediate reclaimed MB is a dangerous metric: hot pages can fault straight back, trading a prettier RAM graph for latency and I/O.

## Proposed eligibility gate

A process must satisfy all of:

- system state is HIGH/CRITICAL for a minimum dwell time;
- not foreground, protected, pinned, or recently active;
- large enough expected useful reclaim;
- low recent fault/activity score;
- no harmful recent intervention history;
- cooldown expired.

## Outcome measurement

Measure at T+1s, T+10s, T+60s:

- working-set delta;
- page-fault delta;
- hard-fault/paging-I/O delta;
- foreground responsiveness proxy.

## Status

**Priority: high-risk.** Keep disabled until the feedback/benchmark stack is mature.
