# MemPilot Technical Research

This document records technologies considered for MemPilot before implementation begins.

The design target is **Windows 10/11 user mode using documented interfaces first**. The project should not depend on undocumented kernel memory-list manipulation for its default path.

---

## 1. System memory telemetry

### `GetPerformanceInfo`

Useful fields include:

- physical total / available pages
- commit total / commit limit
- system cache
- paged / nonpaged kernel memory
- process/thread/handle counts

Why it matters:

- gives a low-cost system snapshot;
- exposes commit independently from physical RAM;
- fits the lightweight default sampler.

Design decision: **Tier A / core**.

Reference:
- https://learn.microsoft.com/windows/win32/memory/memory-performance-information

### `GlobalMemoryStatusEx`

Useful as a second documented view of system memory and as a sanity/fallback source.

Design decision: **Tier A / core fallback/cross-check**.

Reference:
- https://learn.microsoft.com/windows/win32/api/sysinfoapi/nf-sysinfoapi-globalmemorystatusex

### Memory resource notifications

APIs:

- `CreateMemoryResourceNotification`
- `QueryMemoryResourceNotification`

Windows exposes low/high-memory resource notification objects specifically so applications can react to system-wide memory availability changes.

Design implications:

- do not rely only on a fixed percent threshold;
- use low-memory notification as a strong pressure signal;
- use high-memory/recovery state as part of hysteresis;
- waitable notifications can reduce wasteful high-frequency polling.

Design decision: **Tier A / core**.

References:
- https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-creatememoryresourcenotification
- https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-querymemoryresourcenotification

---

## 2. Commit pressure and pagefile reality

System commit is not the same as "RAM used".

Useful counters/signals:

- `Memory\Committed Bytes`
- `Memory\Commit Limit`
- `% Committed Bytes In Use`

Important finding:

If system commit charge approaches the commit limit, allocations can fail even if a simplistic physical-memory view looks acceptable. Windows documentation also notes system-managed page files may grow when commit charge becomes very high.

Design implications:

- commit ratio must be a first-class pressure signal;
- MemPilot must not recommend disabling the pagefile as a generic optimization;
- pagefile health belongs in `doctor`, not automatic destructive tuning.

Design decision: **Tier A telemetry, pagefile mutation rejected for default behavior**.

References:
- https://learn.microsoft.com/troubleshoot/windows-client/performance/introduction-to-the-page-file
- https://learn.microsoft.com/troubleshoot/windows-client/performance/how-to-determine-the-appropriate-page-file-size-for-64-bit-versions-of-windows

---

## 3. Hard faults and paging-I/O signals

Candidate performance counters:

- `Memory\Pages/sec`
- `Memory\Page Reads/sec`
- `Memory\Pages Input/sec`
- `Memory\Page Writes/sec`
- `Memory\Pages Output/sec`

Critical finding:

A hard page fault means data had to be retrieved from backing storage, but that backing storage is **not necessarily the pagefile**. It may be an executable image, DLL, memory-mapped file, or pagefile.

Design implications:

- do not label every hard fault as "pagefile usage";
- pressure scoring should use sustained rates/trends, not one isolated fault;
- after a trim, an increase in hard faults or paging I/O should count as a penalty;
- `Page Reads/sec` and `Pages Input/sec` answer different rate questions and should not be treated as identical raw units.

Design decision: **Tier A/B telemetry**.

Reference:
- https://learn.microsoft.com/troubleshoot/windows-client/performance/how-to-determine-the-appropriate-page-file-size-for-64-bit-versions-of-windows

---

## 4. Per-process memory telemetry

### `GetProcessMemoryInfo` + `PROCESS_MEMORY_COUNTERS_EX2`

Potential data:

- working set
- peak working set
- page-fault count
- private usage/commit
- private working set
- shared commit

Why it matters:

A large total working set is not automatically a good reclaim target. Private working set and private commit can better indicate how much unique process memory is involved.

Design implications:

- candidate scoring must not rank on `WorkingSetSize` alone;
- prefer private residency and activity signals;
- page-fault count should be sampled as a delta, not interpreted as an instantaneous rate.

Design decision: **Tier A / core**.

Reference:
- https://learn.microsoft.com/windows/win32/api/psapi/ns-psapi-process_memory_counters_ex2

### Process identity

PID alone is unsafe as long-lived identity because PIDs can be reused.

Planned identity:

```text
ProcessKey = PID + process creation time
```

Executable path/name is metadata, not the sole runtime identity.

---

## 5. Process metadata and least privilege

Useful APIs:

- `QueryFullProcessImageName`
- process creation/time APIs
- foreground window APIs
- `OpenProcess` with the minimum access required for each operation

Windows explicitly restricts access to protected processes. Some operations require `PROCESS_QUERY_INFORMATION`, `PROCESS_SET_INFORMATION`, or `PROCESS_SET_QUOTA`, while basic metadata can often use `PROCESS_QUERY_LIMITED_INFORMATION`.

Design implications:

- access denied is a normal state, not a reason to demand blanket administrator access;
- use separate read and mutation handles where practical;
- do not use `PROCESS_ALL_ACCESS` as the default open strategy;
- protected/system processes should usually become ineligible rather than generating repeated errors.

Design decision: **Tier A / permanent security rule**.

References:
- https://learn.microsoft.com/windows/win32/procthread/process-security-and-access-rights
- https://learn.microsoft.com/windows/win32/api/winbase/nf-winbase-queryfullprocessimagenamew

---

## 6. Memory priority: preferred first intervention

APIs:

- `GetProcessInformation(ProcessMemoryPriority)`
- `SetProcessInformation(ProcessMemoryPriority)`
- `MEMORY_PRIORITY_INFORMATION`

Documented values range from very low to normal. Windows uses memory priority as a hint when choosing pages to trim; lower-priority pages are preferred for trimming relative to higher-priority pages when other factors are equal.

Why this is attractive:

- works with the Windows Memory Manager instead of pretending MemPilot is a replacement memory manager;
- lower risk than forcibly emptying a process working set;
- current priority can be read and saved for restoration.

Design implications:

- v0.1's first active optimization should be **temporary background memory-priority demotion**;
- preserve original value before mutation;
- foreground/active processes should normally remain normal priority;
- use TTL and restore logic.

Design decision: **Tier A / first active control**.

References:
- https://learn.microsoft.com/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation
- https://learn.microsoft.com/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation
- https://learn.microsoft.com/windows/win32/api/processthreadsapi/ns-processthreadsapi-memory_priority_information

---

## 7. Working-set trimming

APIs:

- `EmptyWorkingSet`
- `SetProcessWorkingSetSizeEx`

Windows can remove as many pages as possible from a process working set. This can immediately reduce resident memory, but pages that are needed again must fault back into the working set.

Key consequence:

**Reclaimed MB alone is not success.**

A successful trim should satisfy something like:

```text
useful_reclaim
    > refault_cost
    + paging_io_cost
    + responsiveness_penalty
```

Design implications:

- never make global repeated trimming the core algorithm;
- only consider cold/background candidates;
- enforce per-process and global cooldowns;
- collect T+1s, T+10s, and T+60s outcomes;
- automatically back off when the process rapidly reloads memory.

Design decision: **Tier B / delayed until feedback infrastructure exists**.

References:
- https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-setprocessworkingsetsizeex
- https://learn.microsoft.com/windows/win32/memory/working-set

---

## 8. Working-set change observation

APIs:

- `InitializeProcessForWsWatch`
- `GetWsChangesEx`

These APIs can report pages newly added to a process working set after monitoring begins.

Potential use:

- estimate whether a trimmed/demoted process rapidly faults data back in;
- identify intervention thrash;
- add evidence to the feedback engine.

Important limitations:

- requires process query rights;
- buffer can overflow and records can be lost if not sampled sufficiently often;
- should not become the universal always-on fast path before overhead is measured.

Design decision: **Tier B / feedback experiment**.

References:
- https://learn.microsoft.com/windows/win32/api/psapi/nf-psapi-initializeprocessforwswatch
- https://learn.microsoft.com/windows/win32/api/psapi/nf-psapi-getwschangesex

---

## 9. Deep working-set inspection

API:

- `QueryWorkingSetEx`

The extended working-set page metadata can expose attributes such as:

- shared/share-count information
- protection
- NUMA node
- locked state
- large-page state

Potential use:

- targeted diagnostics;
- understand the quality/composition of resident memory;
- avoid simplistic treatment of shared or locked pages.

Why not default:

- it operates on supplied virtual addresses rather than being a cheap one-call process summary;
- broad virtual-address inspection could become expensive and complex;
- deep scans are unnecessary for the ordinary policy loop.

Design decision: **Tier B diagnostic mode only**.

References:
- https://learn.microsoft.com/windows/win32/api/psapi/nf-psapi-queryworkingsetex
- https://learn.microsoft.com/windows/win32/api/psapi/ns-psapi-psapi_working_set_ex_block

---

## 10. ETW as high-resolution diagnostics

Windows exposes system memory ETW keywords/events for areas including:

- general memory information
- hard faults
- all page faults
- pool activity
- working sets
- virtual allocations
- footprint/reference-set analysis

Potential use:

- benchmark ground truth;
- fault bursts by process;
- advanced leak/commit diagnostics;
- correlation between MemPilot interventions and system behavior.

Why not the normal sampler immediately:

- ETW adds complexity and collection overhead;
- a lightweight observer should be able to run without a full trace session;
- it is better suited to diagnostic mode and benchmark validation first.

Design decision: **Tier B / v0.3 diagnostics and benchmark validation**.

References:
- https://learn.microsoft.com/windows/win32/etw/system-providers
- https://learn.microsoft.com/windows-hardware/test/wpt/windows-performance-recorder
- https://learn.microsoft.com/windows-hardware/test/wpt/windows-performance-analyzer

---

## 11. WPR/WPA as an external truth source

Windows Performance Recorder and Windows Performance Analyzer can inspect memory utilization, page faults, hard faults, virtual allocations, resident/reference sets, pool memory, and more depending on the recording profile.

Why MemPilot needs this:

- prevents the project from validating itself only with its own counters;
- enables A/B comparisons against stock Windows behavior;
- allows investigation of regressions that the lightweight sampler misses.

Design decision: **mandatory benchmark tooling before v0.1 release**.

References:
- https://learn.microsoft.com/windows-hardware/test/wpt/memory-footprint-optimization
- https://learn.microsoft.com/windows-hardware/test/wpt/list-of-wpa-graphs

---

## 12. Job Objects

Windows Job Objects can group processes and apply or observe limits. Extended Job Object information supports process/job memory limits and peak memory accounting. Notification limits can be used without immediately enforcing a hard allocation failure.

Potential MemPilot use:

- workloads explicitly launched through MemPilot;
- child-process accounting;
- soft memory-budget notifications;
- optional hard commit budget for advanced controlled workloads.

Important design boundary:

MemPilot should not attempt to retroactively turn every arbitrary desktop process into a tightly constrained managed workload.

Design decision: **Tier B/C / v0.4 managed workload feature**.

References:
- https://learn.microsoft.com/windows/win32/procthread/job-objects
- https://learn.microsoft.com/windows/win32/api/winnt/ns-winnt-jobobject_extended_limit_information

---

## 13. Prefetch/recovery experiments

API candidate:

- `PrefetchVirtualMemory`

Idea:

After a known intervention, a process that is about to become foreground again may benefit from bounded prefetch of known relevant ranges.

Problems:

- blindly prefetching creates new memory pressure;
- identifying correct address ranges is nontrivial;
- benefit depends heavily on workload and storage latency;
- an optimizer can easily undo its own reclaim.

Design decision: **Tier C / experiment only after trim feedback works**.

Reference:
- https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-prefetchvirtualmemory

---

## 14. Windows Memory Management Agent features

MMAgent exposes feature state/control for areas such as:

- memory compression
- page combining
- application launch prefetching
- application prelaunch
- Operation API prefetch functionality

Interesting finding:

Page combining can reduce physical memory usage when private pageable pages contain identical content, but Microsoft notes that it also has CPU cost and is workload dependent.

MemPilot design:

- `doctor` may report relevant feature state;
- do not silently toggle these OS-level features in the default policy;
- any change must be explicit, reversible, and benchmarked.

Design decision: **Tier C / diagnostics first**.

References:
- https://learn.microsoft.com/powershell/module/mmagent/get-mmagent
- https://learn.microsoft.com/powershell/module/mmagent/enable-mmagent
- https://learn.microsoft.com/windows-server/administration/performance-tuning/subsystem/cache-memory-management/improvements-in-windows-server

---

## 15. Techniques deliberately rejected from the normal path

### Global standby-list purge loops

Reason:

Standby/cached memory is not equivalent to useless memory. Purging it to increase the "free" number can destroy useful cache and cause later I/O/fault cost.

Status: **rejected as normal optimization**.

### Automatic system-file-cache flushing

Reason:

Can produce attractive before/after screenshots while degrading real workload performance.

Status: **rejected by default; diagnostics/explicit experiments only**.

### Pagefile disabling/shrinking

Reason:

Reduces commit headroom and can turn high commit pressure into allocation failure.

Status: **rejected**.

### Undocumented `NtSetSystemInformation` memory-list manipulation

Reason:

- undocumented behavioral dependency;
- version risk;
- encourages cleaner-style global flushing instead of policy management.

Status: **rejected for v1 default architecture**.

### Kernel driver solely for forced reclaim

Reason:

- large security/maintenance cost;
- unnecessary for the current product goal;
- would substantially increase signing, compatibility, and crash-risk surface.

Status: **rejected unless a future capability cannot be achieved safely in documented user mode and has strong evidence of value**.

### Automatic process termination

Reason:

Killing an application is not memory optimization.

Status: **not a routine policy action**.

---

## 16. Proposed pressure model

Do not freeze weights until benchmark data exists.

Conceptual model:

```text
PressureScore =
    PhysicalPressure
  + CommitPressure
  + HardFaultPressure
  + PagingIOPressure
  + TrendPressure
  + ResourceNotificationBoost
```

Every component should be normalized and capped.

Requirements:

- hysteresis between pressure states;
- spike smoothing;
- sustained-pressure weighting;
- low-memory resource event may override a weak arithmetic score;
- high RAM utilization with no faults/commit stress should not automatically become HIGH/CRITICAL.

---

## 17. Proposed candidate model

Conceptual model:

```text
CandidateScore =
    ReclaimPotential
  + Coldness
  + BackgroundConfidence
  - RefaultRisk
  - ForegroundRisk
  - ProtectionPenalty
  - HistoricalPenalty
```

A candidate should be ineligible before scoring if it is:

- current foreground;
- explicitly pinned;
- protected/system-critical by policy;
- too recently active;
- in cooldown;
- inaccessible for the requested action;
- previously proven harmful within the current backoff window.

---

## 18. Research questions still open

Before implementation freeze, answer these explicitly:

- [ ] Which Windows versions are the initial supported baseline?
- [ ] Is `PROCESS_MEMORY_COUNTERS_EX2` available on every supported target or do we need an EX fallback?
- [ ] Which process-enumeration API gives the cleanest least-privilege implementation in Rust?
- [ ] Should hard-fault rate come from PDH, ETW, or a mixed implementation?
- [ ] What sampler interval gives useful feedback without noticeable overhead?
- [ ] What exact protected/system-process policy is safest?
- [ ] How do we detect user-perceived foreground responsiveness regressions in automated benchmarks?
- [ ] Can memory-priority changes on arbitrary processes be reliably restored after abnormal MemPilot termination?
- [ ] Is `GetWsChangesEx` signal quality worth its runtime overhead?
- [ ] Does targeted prefetch ever beat simply allowing Windows to fault/prefetch naturally?

---

## Current direction

The strongest architecture found so far is:

```text
cheap documented telemetry
        -> pressure state machine
        -> explainable candidate selection
        -> memory-priority hint
        -> measure outcome
        -> selective trim only under stronger pressure
        -> measure refault/paging penalty
        -> back off when harmful
```

The most important design principle remains:

> **MemPilot should cooperate with the Windows Memory Manager before it tries to force the Windows Memory Manager.**
