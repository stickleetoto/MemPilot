# 56 — NUMA Memory Performance Topology

## Mechanism

Modern Windows memory APIs include NUMA topology/performance queries such as `GetNumaNodeMemoryReadBandwidth`, `GetNumaNodeMemoryReadLatency`, write bandwidth/latency variants, and `GetNumaNodeMemoryClosestInitiatorNode`.

## Why MemPilot cares

On multi-socket/high-end systems, equal free-memory amounts on two NUMA nodes do not imply equal access cost. A future governor or benchmark harness may need to distinguish capacity pressure from locality/bandwidth pressure.

## Integration idea

Collect these values only when supported and only on NUMA-capable systems. Store them as environment metadata first; do not use them in default policy until real multi-node benchmarks exist.

## Source

- Microsoft Learn memoryapi reference: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/

**Priority: medium for workstation/server expansion.**