# 52 — System Cache Residency vs Cached Memory

## Mechanism

Windows exposes several related but non-equivalent cache metrics. `PERFORMANCE_INFORMATION.SystemCache`, `\Memory\Cache Bytes`, `System Cache Resident Bytes`, standby-cache counters, and modified-list bytes describe different slices of cache/residency.

## Why MemPilot cares

A single `cache MB` number can be misleading. The governor should avoid double-counting or treating all cached memory as pinned/wasted.

## Integration idea

Document each metric's definition and produce a diagnostic breakdown instead of one synthetic cache figure. Keep the pressure engine primarily anchored to available memory, commit, and paging behavior.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/memory/memory-performance-information

**Priority: medium-high for telemetry correctness.**