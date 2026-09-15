# 16 — PrefetchVirtualMemory Recovery

## Mechanism

`PrefetchVirtualMemory` is a performance hint that can pull likely-needed virtual address ranges into physical cache using larger/concurrent I/O. Prefetched pages are not directly inserted into the target working set until touched.

## Why MemPilot cares

If MemPilot ever evicts memory and can predict that an application is about to become active again, controlled prefetch may reduce page-fault latency.

## Major limitation

MemPilot generally **does not know the exact useful address ranges** of arbitrary applications. Blindly prefetching large regions can create the very memory pressure MemPilot is trying to reduce.

## Research direction

Only investigate in explicit experiments where address ranges are known or learned safely. Potential use is a cooperative SDK or MemPilot-managed workload, not generic third-party process magic.

## Experiment

Create a lab process with known mapped ranges, evict/cool the ranges, then compare cold access latency with and without targeted prefetch while measuring additional pressure.

## Status

**Priority: experimental.** Do not promise generic app acceleration from this API.
