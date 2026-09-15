# 15 — Job Object Memory Budgets

## Mechanism

Windows Job Objects can group processes and expose process/job committed-memory limits and peak memory tracking. Hard limits can cause future commits to fail; notification limits can be used for observation without immediately enforcing failure.

## Why MemPilot cares

For processes **launched by MemPilot**, a job can provide clean group accounting for parent + child workloads and controlled memory-budget experiments.

## Safe direction

Prefer observation/notification before hard limits:

- record `PeakProcessMemoryUsed` / `PeakJobMemoryUsed`;
- experiment with notification limits;
- reserve commit hard limits for explicit sandbox/profile modes.

## Risks

A hard memory limit changes application correctness: allocations can fail. It must never be silently applied to arbitrary third-party processes.

## Possible command

```text
mempilot run --memory-watch 4G app.exe
```

Later, an explicit `--hard-limit` could exist only with strong warnings.

## Status

**Priority: later/experimental.** Useful for controlled workloads, not general RAM optimization.
