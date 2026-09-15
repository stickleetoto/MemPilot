# MemPilot Research Pack

This directory is a technology stockpile for future MemPilot releases. The rule is simple: **research first, benchmark second, mutate memory last**.

Each note records the mechanism, why it matters, integration idea, risks, and a concrete experiment. Items are intentionally allowed to remain research-only if evidence says they should not ship.

## 20 technology notes

1. [Memory Resource Notifications](01-memory-resource-notifications.md)
2. [Commit & Pagefile Pressure](02-commit-pagefile-pressure.md)
3. [PROCESS_MEMORY_COUNTERS_EX2](03-process-memory-counters-ex2.md)
4. [QueryWorkingSetEx](04-query-working-set-ex.md)
5. [Working-Set Change Watch](05-working-set-change-watch.md)
6. [Memory Priority Governor](06-memory-priority-governor.md)
7. [Selective Working-Set Trim](07-selective-working-set-trim.md)
8. [PDH Memory Counters](08-pdh-memory-counters.md)
9. [ETW System Memory Provider](09-etw-system-memory-provider.md)
10. [WPR/WPA Reference Sets](10-wpr-wpa-reference-sets.md)
11. [Adaptive Pressure Scoring](11-adaptive-pressure-scoring.md)
12. [Refault Feedback & Reclaim Persistence](12-refault-feedback.md)
13. [Stable Process Identity](13-stable-process-identity.md)
14. [Foreground & Activity Protection](14-foreground-activity-protection.md)
15. [Job Object Memory Budgets](15-job-object-memory-budgets.md)
16. [PrefetchVirtualMemory Recovery](16-prefetch-virtual-memory.md)
17. [MMAgent / Compression / Page Combining](17-mmagent-compression-page-combining.md)
18. [VirtualAlloc Lifetime Tracing](18-virtualalloc-lifetime-tracing.md)
19. [Memory QoS: Process vs Thread Priority](19-memory-qos.md)
20. [Evidence-Driven Benchmark Harness](20-benchmark-harness.md)

## Suggested order

- **Near-term v0.3 research:** 01, 02, 03, 08, 09, 11, 12, 13, 14.
- **Active-governor gate:** 06, 07, 19, 20.
- **Deep diagnostics:** 04, 05, 10, 18.
- **Later/experimental:** 15, 16, 17.

## Research acceptance rule

A technology graduates into default behavior only if:

1. it uses documented Windows behavior or has an explicit experimental flag;
2. it has a low-privilege implementation path;
3. it can be measured independently;
4. it improves a workload metric without unacceptable refault, I/O, latency, CPU, or stability cost;
5. it has a rollback/backoff path where mutation is involved.
