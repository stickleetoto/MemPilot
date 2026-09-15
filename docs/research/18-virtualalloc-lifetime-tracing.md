# 18 — VirtualAlloc Lifetime Tracing

## Mechanism

ETW memory events can trace VirtualAlloc/VirtualFree activity with process, base address, region size, and allocation flags. WPA also provides `VirtualAlloc Commit LifeTimes` views.

## Why MemPilot cares

A process with rising commit may be leaking, legitimately loading a level, compiling, caching, or reserving large regions. Allocation lifetime data helps distinguish **persistent growth** from transient bursts.

## Diagnostic features

Potential report:

```text
PID 1234
commit growth: +3.2 GiB / 10 min
largest live allocation cohorts: ...
allocation/free imbalance: ...
```

MemPilot should describe this as diagnosis, not automatically call it a leak.

## Experiment

Trace a synthetic allocator with known lifetimes and a real browser/build workload. Validate allocation totals against process commit deltas and WPA.

## Status

**Priority: medium-high** for `mempilot diagnose`, not the normal governor loop.
