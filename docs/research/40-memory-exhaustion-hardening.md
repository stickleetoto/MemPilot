# 40 — Memory Exhaustion Hardening

## Mechanism

`PROCESS_MEMORY_EXHAUSTION_INFO` can configure a process to fail fast if an allocation fails to commit memory. The documented mode is `PMETypeFailFastOnCommitFailure`.

## Why MemPilot cares

MemPilot itself may run precisely when the system is near commit exhaustion. Its own behavior should remain predictable under extreme pressure. This API is therefore relevant as a **self-hardening research topic**, not as a policy for arbitrary user processes.

## Integration idea

Keep MemPilot allocations bounded, preallocate small critical buffers where reasonable, degrade diagnostics under CRITICAL pressure, and investigate whether fail-fast-on-commit-failure improves correctness versus handling allocation failures normally.

## Strong warning

Automatically enabling this on third-party processes would be reckless: it intentionally terminates the process on commit-allocation failure. Even for MemPilot itself, use only after explicit fault-injection testing.

## Experiment

Run MemPilot in a disposable VM with synthetic commit exhaustion and compare normal allocation-failure handling versus fail-fast mode. Verify logs, cleanup, restore-state behavior, and crash artifacts.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_memory_exhaustion_info

**Priority: medium for reliability research; never an external optimization action.**