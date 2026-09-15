# 48 — Background Processing Mode

## Mechanism

`SetPriorityClass(PROCESS_MODE_BACKGROUND_BEGIN)` lowers resource scheduling priority for the calling process so background work interferes less with foreground activity. The mode can later be ended with `PROCESS_MODE_BACKGROUND_END`.

## Why MemPilot cares

The optimizer itself must not become a source of latency. Expensive scans, trace parsing, history compaction, and benchmark collection should be eligible for background resource scheduling.

## Integration idea

Use only for MemPilot's own background worker phases, with strict enter/exit scopes. Never force another application's entire process into background mode.

## Experiment

Compare MemPilot diagnostic CPU/I/O work with and without background mode while measuring foreground frame time and UI latency.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass

**Priority: high for self-overhead control.**