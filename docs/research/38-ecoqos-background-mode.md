# 38 — EcoQoS / Background Resource Scheduling

## Mechanism

Windows can lower resource-scheduling impact for background work. `SetProcessInformation(ProcessPowerThrottling, ...)` can opt a process into EcoQoS-style execution-speed throttling, while process/thread background modes lower resource scheduling priority for work that should not disturb foreground activity.

## Why MemPilot cares

This is not a memory reclamation primitive, but it can reduce **optimizer interference**. MemPilot's own diagnostic/background workers should avoid stealing CPU/I/O responsiveness from the workload they are trying to protect.

## Integration idea

- keep the main CLI responsive;
- run optional long diagnostics/export/indexing workers under low-impact QoS;
- never apply EcoQoS to foreground or latency-critical user processes automatically;
- treat third-party process mutation here as experimental and explicit only.

## Experiment

Run MemPilot ETW/PSS analysis in normal versus low-impact QoS while a foreground benchmark runs. Compare analysis time, foreground latency, CPU package power, and fan/thermal behavior.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass

**Priority: high for MemPilot self-behavior, low for external mutation.**