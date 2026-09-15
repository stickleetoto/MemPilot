# 30 — GetProcessIoCounters Activity Signal

## Mechanism

`GetProcessIoCounters` exposes cumulative read, write and other I/O operation/byte counts for a process. Sampling deltas provides a cheap approximation of storage/pipe/device activity without ETW.

## Why MemPilot cares

A background process doing heavy I/O may be warming data, streaming assets, compiling, indexing, syncing, or paging-related work. It should not automatically be classified as cold merely because CPU and UI activity are low.

## Integration idea

Track read/write/other byte deltas per stable process identity. Use high I/O as a protection/risk signal, not as proof of memory pressure. Keep ETW as the later path for separating paging I/O from ordinary application I/O.

## Risks

These counters aggregate many I/O types. They cannot tell MemPilot whether bytes came from page faults, file reads, network-backed handles, pipes, or explicit app I/O.

## Experiment

Compare process I/O deltas against ETW and system paging counters during browser cache, game asset streaming, compiler, file copy, and synthetic refault scenarios.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessiocounters

**Priority: high as a low-cost activity signal.**