# 37 — APP_MEMORY_INFORMATION

## Mechanism

`GetProcessInformation(ProcessAppMemoryInfo, ...)` can return `APP_MEMORY_INFORMATION` with available commit, private commit usage, peak private commit usage, and total commit usage for an app/process context.

## Why MemPilot cares

This offers another commit-oriented view that can complement `PROCESS_MEMORY_COUNTERS_EX/EX2`, especially when diagnosing processes with app-container style limits or unusual commit behavior.

## Integration idea

Capability-gate it and treat it as supplementary telemetry. Prefer the ordinary system commit limit for global pressure and EX/EX2 for general process accounting; use APP_MEMORY_INFORMATION when supported and meaningful.

## Risks

Available-commit semantics are app-specific and should not be mistaken for the machine-wide commit limit. Different process types may expose different usefulness.

## Experiment

Compare APP_MEMORY_INFORMATION with EX2 private/shared commit across desktop, packaged, sandboxed and browser child processes.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-app_memory_information
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation

**Priority: medium, diagnostic/capability-dependent.**