# 47 — Copy-on-Write Commit Amplification

## Mechanism

A file view mapped with copy-on-write can become private page by page when written. Windows reserves commit for the possibility that every page in the view could become private.

## Why MemPilot cares

A process may appear to use a shared file mapping while simultaneously carrying significant commit liability. Working-set size alone does not capture that risk.

## Integration idea

Add a `cow_mapping_risk` diagnostic category. Correlate mapped-file regions, protection flags, private commit, and write-copy activity before deciding that mapped memory is cheap.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile

**Priority: high for explaining commit pressure.**