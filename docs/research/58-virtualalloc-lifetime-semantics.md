# 58 — Reserve / Commit / Decommit Lifetime Semantics

## Mechanism

`VirtualAlloc` separates reserving virtual address space from committing backing. `VirtualFree(MEM_DECOMMIT)` removes commitment while preserving the reservation; `MEM_RELEASE` releases the reserved range itself.

## Why MemPilot cares

For cooperative software, the strongest memory optimization may be explicit decommit of genuinely idle arenas rather than trimming their current residency. That reduces commit demand, not merely working-set bytes.

## Integration idea

A future MemPilot SDK can expose allocator guidance for caches/arenas that can safely decommit cold chunks and recommit them later. Never attempt this on arbitrary third-party allocations.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualfree

**Priority: high for cooperative memory management.**