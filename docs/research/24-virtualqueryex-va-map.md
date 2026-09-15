# 24 — VirtualQueryEx VA Region Classifier

## Mechanism

`VirtualQueryEx` walks another process's virtual address space and reports contiguous regions that share state, type, allocation origin, and protection. Important classes include `MEM_PRIVATE`, `MEM_MAPPED`, and `MEM_IMAGE` plus commit/reserve/free state.

## Why MemPilot cares

A process with a large working set is not one uniform block. Region classification can estimate whether resident memory is private heap-like data, mapped files, executable images, reserved space, or guarded/no-access regions.

## Integration idea

Use only in explicit deep diagnostics, not the 1–2 second runtime loop. Build a per-process VA summary such as private committed, mapped committed, image committed, reserved, and protection distribution.

## Caveat

Copy-on-write pages may still be reported as mapped/image after becoming private. Pair sampled regions with `QueryWorkingSetEx` Shared-bit data when precision matters.

## Experiment

Profile browsers, Electron apps, games, IDEs, databases, and memory-mapped workloads; compare VA composition against private working set and candidate rankings.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualqueryex

**Priority: high for diagnose mode.**