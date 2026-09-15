# 22 — DiscardVirtualMemory

## Mechanism

`DiscardVirtualMemory` discards the contents of a committed `PAGE_READWRITE` region without decommitting the virtual address range. Physical RAM backing those pages can be returned to the system; the application must rewrite the data before relying on it again.

## Why MemPilot cares

This is another cooperative optimization primitive for caches and scratch buffers. Compared with `VirtualFree`, the address range stays committed, which can simplify allocators that want to preserve virtual-address layout while giving physical pages back.

## Integration idea

Future MemPilot SDK clients could register disposable buffers. Under sustained pressure, the app can discard cold scratch/cache regions and rebuild lazily.

## Risks

- only safe for data that can be regenerated;
- not an external-process optimization tool;
- discarded contents are undefined;
- wrong use becomes data corruption.

## Experiment

Measure RSS reduction, rehydration latency, commit behavior, and page-fault pattern for a synthetic cache using discard versus free/reallocate.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-discardvirtualmemory

**Priority: medium, cooperative-only.**