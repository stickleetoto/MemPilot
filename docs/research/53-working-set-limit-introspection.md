# 53 — Working-Set Limit Introspection

## Mechanism

`GetProcessWorkingSetSizeEx` reports a process's minimum and maximum working-set sizes and whether hard min/max enforcement flags are enabled.

## Why MemPilot cares

Before interpreting unusual residency or testing trims, MemPilot should know whether the process already has explicit working-set policy. Existing hard limits can radically change normal paging behavior.

## Integration idea

Add read-only fields to deep diagnostics:

- minimum working set;
- maximum working set;
- hard-min enabled;
- hard-max enabled.

Do not override those limits automatically.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getprocessworkingsetsizeex

**Priority: medium-high diagnostic context.**