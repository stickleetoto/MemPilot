# 41 — VirtualLock / Locked-Page Awareness

## Mechanism

`VirtualLock` pins pages from the calling process into physical memory so later access does not incur page faults. `VirtualUnlock` releases that guarantee. Locked pages therefore behave differently from ordinary reclaimable working-set pages.

## Why MemPilot cares

Any page-level diagnostics or trim heuristics should treat locked memory as effectively non-reclaimable. A process with a large working set may still expose little safe reclaim if much of it is locked.

## Integration idea

- detect locked pages through `QueryWorkingSetEx` sampling;
- subtract locked-page estimates from reclaim confidence;
- never attempt to “optimize” another process by blindly calling `VirtualUnlock`;
- report unusually large locked regions in diagnostics.

## Experiment

Create a helper process that locks known ranges and compare total working set, sampled locked ratio, trim behavior, and post-trim residency.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtuallock
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualunlock

**Priority: medium-high diagnostic safety.**