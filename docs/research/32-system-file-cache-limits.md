# 32 — System File Cache Limits

## Mechanism

`GetSystemFileCacheSize` reports configured minimum/maximum working-set limits for the system file cache. `SetSystemFileCacheSize` can set hard limits or flush the cache by using sentinel values.

## Why MemPilot cares

The read side is useful diagnostic context: unusual cache limits can explain machine behavior that looks like memory pressure. The write side is much riskier and should not be treated as a normal RAM-cleaning mechanism.

## Product stance

MemPilot should be **observe-first** here. Report active file-cache limits and whether hard min/max flags are set. Do not auto-flush or auto-cap the system cache in normal operation.

## Risks

A file-cache cap can reduce useful caching and shift cost into disk I/O. A flush can make Task Manager look better while making actual workload latency worse.

## Experiment

In a lab VM, compare Windows default cache behavior against explicit limits during file-copy, compile, database and game-asset workloads. Track disk latency, page faults, cache residency and foreground responsiveness.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getsystemfilecachesize
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-setsystemfilecachesize

**Priority: diagnostic only by default.**