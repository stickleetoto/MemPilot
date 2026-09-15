# 23 — MEM_RESET / MEM_RESET_UNDO

## Mechanism

`VirtualAlloc` supports `MEM_RESET` to tell Windows that the contents of a committed range are no longer important. The range stays committed, but pages do not need to be preserved. `MEM_RESET_UNDO` later asks Windows whether the original contents survived; failure means at least some data was zeroed and must be rebuilt.

## Why MemPilot cares

This provides a cooperative cache-pressure mechanism with slightly different semantics from Offer/Reclaim. It is useful for applications that own large rebuildable buffers and want Windows to reclaim backing storage opportunistically.

## Integration idea

Treat this as an optional SDK primitive, never an external-process action. A cooperating app may reset cold regenerable regions when MemPilot reports sustained HIGH pressure and undo/reset or rebuild on reuse.

## Experiment

Compare `MEM_RESET`, Offer/Reclaim, DiscardVirtualMemory, and free/reallocate on the same cache workload. Measure resident-memory drop, commit accounting, recovery latency, and data survival rate.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc

**Priority: medium, experimental/cooperative.**