# 42 — Working-Set Page Flags

## Mechanism

`PSAPI_WORKING_SET_EX_BLOCK`, returned through `QueryWorkingSetEx`, exposes page-level metadata including share count, shared/private status, protection, NUMA node, locked state, large-page state, and bad-page status.

## Why MemPilot cares

This lets deep diagnostics distinguish memory that merely appears in a process working set from memory that is actually private, locked, shared, large-page backed, or otherwise a poor trim target.

## Integration idea

Sample only a bounded subset of addresses from selected candidates and derive a `reclaimability profile` rather than scanning every page continuously.

## Safety rule

Treat this metadata as diagnostic evidence, not a precise per-page eviction oracle. Snapshots age immediately.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/psapi/ns-psapi-psapi_working_set_ex_block

**Priority: high for diagnose mode.**