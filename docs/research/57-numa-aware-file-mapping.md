# 57 — NUMA-Aware File Mapping

## Mechanism

`CreateFileMapping2` and `MapViewOfFile3` can take extended parameters, including a preferred NUMA node for physical memory placement.

## Why MemPilot cares

A cooperative application could expose large shared caches or arenas whose placement matters on NUMA machines. MemPilot's SDK could help such applications place memory near the workload rather than merely reducing total bytes.

## Integration idea

Keep this as an opt-in cooperative feature. First expose NUMA topology and benchmark local-vs-remote access before adding any allocation advice.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-createfilemapping2
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile3

**Priority: later cooperative/server feature.**