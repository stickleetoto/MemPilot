# 43 — AWE / User Physical Pages

## Mechanism

Address Windowing Extensions (AWE) let a process allocate physical pages with `AllocateUserPhysicalPages` and map them into an AWE region with `MapUserPhysicalPages`. These pages are physically resident and require `SeLockMemoryPrivilege` for allocation.

## Why MemPilot cares

AWE memory is not ordinary pageable working-set memory. Aggressive heuristics that assume every large resident footprint is reclaimable would misclassify AWE-heavy workloads.

## Integration idea

- classify AWE/locked-page-heavy processes as special workloads;
- lower reclaim confidence sharply;
- surface privilege-backed physical memory usage as a diagnostic warning rather than a trim opportunity.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-allocateuserphysicalpages
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapuserphysicalpages

**Priority: medium, mostly workload protection.**