# 55 — Memory Error Handling Capabilities

## Mechanism

`GetMemoryErrorHandlingCapabilities` reports whether the platform supports Windows memory-error handling features.

## Why MemPilot cares

Hardware reliability behavior varies by platform. Recording capability metadata gives diagnostic context for bad-page events and pressure-related crashes on workstation/server systems.

## Integration idea

Expose the capability in `mempilot diagnose system` and include it in benchmark/environment manifests. This is observation-only and should never affect normal candidate scoring unless an actual hardware-error signal occurs.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getmemoryerrorhandlingcapabilities

**Priority: low-medium platform diagnostics.**