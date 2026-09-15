# 19 — Memory QoS: Process vs Thread Priority

## Mechanism

Windows supports memory priority at both process and thread levels. Process priority becomes the default for pages added by threads; thread-level control can target background worker activity more precisely.

## Why MemPilot cares

A whole application may be interactive while only one background worker performs low-value scanning/indexing. In cooperative workloads, thread-level memory QoS could avoid punishing the whole process.

## Practical boundary

For arbitrary third-party applications, MemPilot often lacks semantic knowledge of threads. Therefore:

- generic governor: process-level only;
- cooperative SDK/managed workload: thread-level experiments;
- never guess critical vs background threads from thread ID alone.

## Restoration model

Any temporary priority change needs original-value capture, TTL, identity validation, and restoration on pressure recovery/exit.

## Experiment

Build a cooperative lab app with foreground and background worker threads. Compare process-wide demotion with worker-only demotion under controlled pressure.

## Status

**Priority: medium.** Strong architectural idea, but mostly for cooperative or well-understood workloads.
