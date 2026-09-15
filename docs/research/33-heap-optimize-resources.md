# 33 — HeapOptimizeResources

## Mechanism

`HeapSetInformation(..., HeapOptimizeResources, ...)` can optimize caches for low-fragmentation heaps and decommit memory when possible. Calling it with a null heap handle can target all LFH heaps in the current process.

## Why MemPilot cares

This is not an external-process optimizer. It is useful in two places: keeping **MemPilot itself** lean after bursty diagnostic work, and as an optional cooperative API for software that integrates with MemPilot.

## Integration idea

After heavy one-shot operations such as ETW parsing or PSS diagnostics, MemPilot can experimentally request heap resource optimization for itself and record whether private commit/residency drops without harming latency.

## Risks

Calling it too often may trade allocator cache efficiency for lower memory. It should be event-driven, not placed in the sampling loop.

## Experiment

Run a bursty allocator benchmark, enter an idle phase, invoke HeapOptimizeResources, then measure commit/resident reduction and the latency of the next allocation burst.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapsetinformation
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-heap_optimize_resources_information

**Priority: medium for MemPilot self-optimization.**