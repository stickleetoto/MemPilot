# 29 — QueryProcessCycleTime

## Mechanism

`QueryProcessCycleTime` returns the total CPU cycle count consumed by all threads in a process. It gives a very cheap cumulative activity counter and can be sampled as a delta.

## Why MemPilot cares

It can complement `GetProcessTimes` when ranking background activity. A process whose cycle count is rising rapidly should be treated as active even when its window is hidden.

## Caveat

Cycle counts are not wall-clock time. Microsoft warns against converting them directly to elapsed time because CPU timer behavior can differ by hardware/frequency behavior. Use only as a relative activity signal.

## Integration idea

Store cycle deltas per stable process identity, normalize within the observation window, and combine with CPU-time, I/O and fault deltas rather than using a fixed global threshold.

## Experiment

Compare cycle deltas against `GetProcessTimes` and ETW CPU attribution across mixed foreground/background workloads. Determine whether it adds predictive value before keeping both signals.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-queryprocesscycletime

**Priority: medium-high.**