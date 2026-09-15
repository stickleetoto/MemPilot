# 21 — OfferVirtualMemory / ReclaimVirtualMemory

## Mechanism

Windows lets an application **offer** page-aligned memory that it can regenerate. `OfferVirtualMemory` removes those pages from the process working set and makes them inaccessible without writing them to the paging file. `ReclaimVirtualMemory` later asks for them back; if Windows discarded the data, the application must rebuild it.

## Why MemPilot cares

This is not a tool for stealing memory from arbitrary processes. It is more interesting as a future **MemPilot cooperative SDK**: applications that opt in could mark caches or regenerable buffers as disposable when system pressure rises.

## Integration idea

- expose pressure state over a tiny local API;
- cooperating apps offer cold cache regions under HIGH/CRITICAL pressure;
- reclaim on demand;
- measure intact-reclaim rate and regeneration cost;
- use `OFFER_PRIORITY` to distinguish cheap versus expensive-to-regenerate data.

## Safety

Never apply to memory the target cannot regenerate. Offered pages may come back with undefined contents.

## Experiment

Build a test process with a regenerable 512 MiB cache. Compare normal paging, Offer/Reclaim, and complete free/reallocate behavior under controlled pressure.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-offervirtualmemory
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-reclaimvirtualmemory

**Priority: medium-high, cooperative-only.**