# 26 — Mapped-File Attribution

## Mechanism

`GetMappedFileNameW` checks whether a virtual address belongs to a memory-mapped file and returns the backing file path. Process Snapshotting (`PSS_CAPTURE_VA_SPACE_SECTION_INFORMATION`) can also capture section backing information for `MEM_IMAGE` and `MEM_MAPPED` regions.

## Why MemPilot cares

Mapped pages should not be treated like anonymous private memory. A large mapped-file working set may be cheap to evict but expensive to refault from slow storage; image-backed pages have different reuse patterns from private heap pages.

## Integration idea

In diagnose mode, sample mapped/image regions and classify backing files by category: executable image, DLL, database/index, cache file, game asset, browser cache, etc. Keep path collection optional because it may expose sensitive filenames in logs.

## Experiment

Compare refault cost and persistence for private, image-backed, and mapped-file-heavy workloads on SSD versus HDD.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmappedfilenamew
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/processsnapshot/ne-processsnapshot-pss_capture_flags

**Priority: high for explaining reclaim cost.**