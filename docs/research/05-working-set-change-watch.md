# 05 — Working-Set Change Watch

## Mechanism

`InitializeProcessForWsWatch` starts working-set monitoring. `GetWsChangesEx` reports pages added to the process working set since the previous query and includes the faulting thread ID. If its internal buffer fills, records can be lost until drained.

## Why MemPilot cares

It can reveal whether a process rapidly re-faults pages after an intervention. That makes it useful for measuring trim harm and identifying hot processes that should receive longer cooldowns.

## Integration idea

Use it only for a **small temporary watch set** around an intervention:

1. begin watch before action;
2. apply experimental action;
3. sample changes for 1–60 seconds;
4. calculate page-add rate and burstiness;
5. tear down watch.

## Risks

- buffer overflow/data loss if not queried frequently enough;
- added overhead;
- not every process allows required access;
- raw working-set additions are not identical to disk hard faults.

## Experiment

Compare `GetWsChangesEx` refault bursts with process page-fault deltas and system hard-fault counters after synthetic trims.

## Status

**Priority: medium-high** for experimental feedback, not permanent monitoring.
