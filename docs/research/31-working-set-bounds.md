# 31 — Working-Set Min/Max Bounds

## Mechanism

`GetProcessWorkingSetSizeEx` reads a process's minimum/maximum working-set bounds and enforcement flags. `SetProcessWorkingSetSizeEx` can change them, including hard minimum/maximum flags, or use `-1/-1` to remove as many pages as possible.

## Why MemPilot cares

These APIs expose another layer of Windows paging policy beyond a one-shot trim. They could theoretically cap residency for MemPilot-launched workloads or controlled experiments.

## Strong warning

Hard working-set caps can easily create continuous refaulting and latency. Microsoft explicitly warns that changing working-set sizes affects whole-system performance. This should not become a generic optimizer feature without very strong evidence.

## Integration idea

- read/report current bounds in diagnostics;
- detect unusual hard limits;
- reserve mutation for lab/explicit managed-workload modes;
- always record original values and restore them.

## Experiment

On synthetic workloads, compare soft limits, hard maximums, one-shot trim, and memory-priority demotion. Measure sustained hard faults and task latency, not just resident MB.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getprocessworkingsetsizeex
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-setprocessworkingsetsizeex

**Priority: low for product, high for research.**