# 45 — MapViewOfFile3, Placeholders & Transient Boost

## Mechanism

`MapViewOfFile3` can map file/pagefile-backed sections with extended parameters and placeholder replacement. `UnmapViewOfFile2` supports `MEM_PRESERVE_PLACEHOLDER` and `MEM_UNMAP_WITH_TRANSIENT_BOOST`, which temporarily boosts the priority of pages expected to be accessed again shortly from another thread.

## Why MemPilot cares

This is a useful cooperative-memory pattern: a producer can hand off mapped data without treating every unmap as cold disposable memory.

## Integration idea

Research a cooperative buffer handoff primitive where mapped views can be replaced while preserving VA layout and optionally signaling short-term reuse expectations.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile3
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-unmapviewoffile2

**Priority: experimental cooperative-runtime feature.**