# 39 — MEM_WRITE_WATCH / GetWriteWatch

## Mechanism

Regions allocated with `MEM_WRITE_WATCH` let Windows track which pages have been written since allocation or the last reset. `GetWriteWatch` returns page addresses and the tracking granularity.

## Why MemPilot cares

This is a powerful **cooperative heat signal** for allocators and caches owned by software that integrates with MemPilot. A region with few writes over a long period is a better candidate for discard/offer/reset than a write-hot region.

## Integration idea

Future SDK clients may allocate selected large caches with `MEM_WRITE_WATCH`, then expose a compact dirty-page ratio to MemPilot. Keep page addresses inside the application; MemPilot only needs aggregate heat metrics.

## Risks

Only works for regions allocated with the flag. Reset races can miss writes if synchronization is wrong. It is not a general external-process profiler.

## Experiment

Create synthetic read-mostly, write-hot, and bursty caches. Compare dirty-page ratio against future reuse and determine whether it predicts safe disposal.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getwritewatch
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc

**Priority: medium-high for cooperative SDK research.**