# 34 — NUMA-Aware Pressure Analysis

## Mechanism

Windows exposes NUMA topology and per-node available-memory information through APIs such as `GetNumaAvailableMemoryNodeEx`. On multi-node systems, memory pressure can be asymmetric across nodes.

## Why MemPilot cares

A single global available-RAM number can hide locality problems on workstation/server hardware. A process may suffer remote-memory latency or local-node scarcity even when total system memory looks healthy.

## Integration idea

- detect whether the machine has more than one NUMA node;
- expose node-local available memory in diagnose mode;
- never apply NUMA policy on single-node consumer systems;
- future managed-workload mode could consider placement only for processes MemPilot launches or controls cooperatively.

## Caveat

Microsoft notes that per-node available memory semantics differ on multi-node systems and may exclude standby pages. Do not sum node values and assume they equal ordinary `Available Bytes` without understanding the counter definition.

## Experiment

Use a multi-NUMA VM/server to compare global pressure score versus per-node scarcity while pinning allocations to different nodes.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getnumaavailablememorynodeex

**Priority: later/server-oriented.**