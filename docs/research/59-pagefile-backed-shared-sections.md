# 59 — Pagefile-Backed Shared Sections

## Mechanism

A file-mapping object created with an invalid file handle can be backed by the system paging file and mapped into one or more process address spaces. `MapViewOfFile2` can map a file or pagefile-backed section into a specified process.

## Why MemPilot cares

This gives a future cooperative SDK a documented shared-memory transport for tiny telemetry/control structures without a kernel driver. It can also host shared caches whose commit and residency are explicitly understood by cooperating processes.

## Integration idea

Use a very small, versioned shared-memory control block for opt-in clients. Keep bulk data ownership with the application and require explicit disposable-memory declarations rather than inspecting raw contents.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile2

**Priority: high for a driverless cooperative protocol.**