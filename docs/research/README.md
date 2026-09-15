# MemPilot Research Pack

This directory is a technology stockpile for future MemPilot releases. The rule is simple: **research first, benchmark second, mutate memory last**.

Each note records the mechanism, why it matters, integration idea, risks, and a concrete experiment. Items are intentionally allowed to remain research-only if evidence says they should not ship.

## 60 technology notes

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
21. [OfferVirtualMemory / ReclaimVirtualMemory](21-offer-reclaim-virtual-memory.md)
22. [DiscardVirtualMemory](22-discard-virtual-memory.md)
23. [MEM_RESET / MEM_RESET_UNDO](23-mem-reset-undo.md)
24. [VirtualQueryEx VA Region Classifier](24-virtualqueryex-va-map.md)
25. [QueryVirtualMemoryInformation / MemoryRegionInfo](25-query-virtual-memory-information.md)
26. [Mapped-File Attribution](26-mapped-file-attribution.md)
27. [Process Snapshotting (PSS)](27-process-snapshotting.md)
28. [Process CPU Activity Deltas](28-process-cpu-activity-deltas.md)
29. [QueryProcessCycleTime](29-process-cycle-time.md)
30. [GetProcessIoCounters Activity Signal](30-process-io-activity.md)
31. [Working-Set Min/Max Bounds](31-working-set-bounds.md)
32. [System File Cache Limits](32-system-file-cache-limits.md)
33. [HeapOptimizeResources](33-heap-optimize-resources.md)
34. [NUMA-Aware Pressure Analysis](34-numa-awareness.md)
35. [Large-Page Awareness](35-large-page-awareness.md)
36. [Installed RAM vs OS-Usable RAM](36-installed-vs-usable-ram.md)
37. [APP_MEMORY_INFORMATION](37-app-memory-information.md)
38. [EcoQoS / Background Resource Scheduling](38-ecoqos-background-mode.md)
39. [MEM_WRITE_WATCH / GetWriteWatch](39-write-watch.md)
40. [Memory Exhaustion Hardening](40-memory-exhaustion-hardening.md)
41. [VirtualLock / Locked-Page Awareness](41-virtual-lock-awareness.md)
42. [Working-Set Page Flags](42-working-set-page-flags.md)
43. [AWE / User Physical Pages](43-awe-user-physical-pages.md)
44. [VirtualAlloc2 Placeholders & Address Requirements](44-virtualalloc2-placeholders.md)
45. [MapViewOfFile3, Placeholders & Transient Boost](45-mapview-placeholders-transient-boost.md)
46. [File Mapping: SEC_RESERVE vs SEC_COMMIT](46-file-mapping-reserve-commit.md)
47. [Copy-on-Write Commit Amplification](47-copy-on-write-commit-amplification.md)
48. [Background Processing Mode](48-background-processing-mode.md)
49. [Standby-List Tier Telemetry](49-standby-list-tier-telemetry.md)
50. [Modified-Page Writeback Pressure](50-modified-page-writeback-pressure.md)
51. [Transition Pages RePurposed/sec](51-transition-pages-repurposed.md)
52. [System Cache Residency vs Cached Memory](52-system-cache-residency.md)
53. [Working-Set Limit Introspection](53-working-set-limit-introspection.md)
54. [Bad Memory Notifications](54-bad-memory-notifications.md)
55. [Memory Error Handling Capabilities](55-memory-error-capabilities.md)
56. [NUMA Memory Performance Topology](56-numa-memory-performance-topology.md)
57. [NUMA-Aware File Mapping](57-numa-aware-file-mapping.md)
58. [Reserve / Commit / Decommit Lifetime Semantics](58-virtualalloc-lifetime-semantics.md)
59. [Pagefile-Backed Shared Sections](59-pagefile-backed-shared-sections.md)
60. [Mapped-File Writeback & Flush Semantics](60-mapped-file-writeback.md)

## Suggested order

- **Near-term v0.3 telemetry:** 01, 02, 03, 08, 11, 13, 14, 28, 30, 36, 49, 50, 51, 52, 53.
- **Active-governor gate:** 06, 07, 12, 19, 20, 31, 41, 42, 47.
- **Deep diagnostics:** 04, 05, 09, 10, 18, 24, 25, 26, 27, 43, 46, 54, 55, 60.
- **Cooperative SDK ideas:** 21, 22, 23, 33, 39, 44, 45, 57, 58, 59.
- **Platform/workload awareness:** 15, 16, 17, 34, 35, 37, 38, 40, 48, 56.

## Research acceptance rule

A technology graduates into default behavior only if:

1. it uses documented Windows behavior or has an explicit experimental flag;
2. it has a low-privilege implementation path;
3. it can be measured independently;
4. it improves a workload metric without unacceptable refault, I/O, latency, CPU, or stability cost;
5. it has a rollback/backoff path where mutation is involved.

## Current architectural insight

The research packs now expose three complementary layers:

- **external governor:** observe and conservatively influence ordinary Windows processes;
- **cooperative memory API:** software that explicitly integrates with MemPilot can expose disposable/regenerable memory and make safer, stronger reclamation decisions;
- **memory topology/diagnostics layer:** explain where memory lives, what backs it, whether it is actually reclaimable, and whether pressure is RAM-, commit-, writeback-, mapping-, NUMA-, or hardware-related.

The long-term advantage is therefore not a stronger “RAM cleaner.” It is better classification plus safer coordination: reclaim only memory whose cost and recovery behavior are understood.