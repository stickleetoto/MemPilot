# 28 — Process CPU Activity Deltas

## Mechanism

`GetProcessTimes` exposes cumulative kernel and user CPU time plus process creation time. Sampling it periodically yields a cheap activity delta without requiring ETW.

## Why MemPilot cares

A large process that is actively computing is a poor candidate for memory demotion or trim even if it is not foreground. CPU activity is therefore an important **negative signal** in candidate scoring.

## Integration idea

For each stable process identity, store previous kernel+user time and compute normalized CPU activity over the sample interval. Combine with page-fault and working-set deltas to classify processes as hot/warm/cold.

## Policy rule

CPU inactivity must never be interpreted as proof that memory is cold; GPU-heavy apps, blocked workers, and mapped-file consumers can have low CPU while still latency-sensitive.

## Experiment

Record CPU delta, page-fault delta, I/O delta, and focus changes across games, browsers, compilers, editors, media players, launchers, and idle services. Train thresholds only from workload evidence.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes

**Priority: critical for candidate safety.**