# 50 — Modified-Page Writeback Pressure

## Mechanism

`\Memory\Modified Page List Bytes` measures modified physical pages waiting to be written to backing storage. A large modified list combined with low available memory and heavily used page files can signal a very different bottleneck from ordinary clean-cache pressure.

## Why MemPilot cares

Trimming or demoting processes during heavy dirty-page writeback can increase storage contention instead of helping foreground responsiveness.

## Integration idea

Add a `writeback_pressure` signal from modified-list size plus Pages Output/sec/Page Writes/sec and pagefile usage. Under high writeback pressure, prefer observation/backoff over aggressive trim.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/troubleshoot/windows-client/performance/how-to-determine-the-appropriate-page-file-size-for-64-bit-versions-of-windows

**Priority: critical for avoiding I/O-amplifying interventions.**