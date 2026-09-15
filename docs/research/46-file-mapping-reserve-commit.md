# 46 — File Mapping: SEC_RESERVE vs SEC_COMMIT

## Mechanism

Pagefile-backed file mappings can reserve address space with `SEC_RESERVE` or commit the mapping with `SEC_COMMIT`. The two modes have very different commit-accounting behavior.

## Why MemPilot cares

Large mapped regions can look similar in VA maps while imposing very different commit risk. A governor that understands only VA size can badly overestimate or underestimate pressure.

## Integration idea

In deep diagnostics, classify pagefile-backed mappings separately and annotate whether they contribute reserved address space or committed backing. For cooperative SDK allocations, prefer delayed commitment when appropriate.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile

**Priority: high for commit-aware diagnostics.**