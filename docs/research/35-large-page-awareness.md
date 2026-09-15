# 35 — Large-Page Awareness

## Mechanism

Windows supports large pages, typically 2 MiB or larger, for specially privileged applications. `GetLargePageMinimum` reports the platform minimum, and large-page allocations use `MEM_LARGE_PAGES` with `SeLockMemoryPrivilege`.

## Why MemPilot cares

Large-page workloads are often performance-sensitive databases, runtimes, or compute applications. Treating them like ordinary idle applications can be dangerous because page layout, locking privileges, and TLB-performance assumptions differ.

## Integration idea

- detect whether the platform supports large pages;
- use `QueryWorkingSetEx`/VA diagnostics to identify large-page-related mappings where possible;
- mark known large-page workloads as high-risk candidates;
- never try to force large-page configuration or privilege changes.

## Experiment

Compare candidate classification and intervention cost for normal-page versus large-page synthetic workloads. Validate that MemPilot's protection rules avoid regressions.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getlargepageminimum
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/memory/large-page-support

**Priority: medium as a safety classifier.**