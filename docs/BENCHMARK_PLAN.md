# MemPilot Benchmark Plan

MemPilot must prove that an intervention improves **useful memory availability without unacceptable refault, I/O, CPU, or responsiveness cost**.

A screenshot showing lower RAM usage is not sufficient evidence.

---

## 1. Comparison modes

Every benchmark scenario should run in at least these modes:

1. **Windows Default** — MemPilot not running.
2. **Observe Only** — MemPilot collects telemetry but never mutates process/system state.
3. **Priority Policy** — memory-priority governor enabled, no forced trim.
4. **Priority + Selective Trim** — later, after trim is implemented and independently accepted.

This separates MemPilot overhead from MemPilot policy benefit.

---

## 2. Required hardware classes

Primary target classes:

- 8 GB RAM — intentionally constrained
- 16 GB RAM — mainstream target
- 32 GB RAM — ensure policy does not overreact when memory is abundant

Optional later:

- 64 GB+
- HDD vs SATA SSD vs NVMe
- laptop power/battery scenario

Record for every run:

- Windows edition/build
- RAM capacity
- storage type
- pagefile mode/size
- CPU
- MemPilot version/commit
- benchmark scenario version

---

## 3. Baseline scenarios

### A. Idle desktop

Purpose:

- measure MemPilot overhead;
- detect false pressure;
- ensure no unnecessary intervention.

Pass condition:

- no mutation under healthy pressure;
- negligible CPU wakeups/overhead;
- stable own memory footprint.

### B. Browser-heavy

Example shape:

- many tabs;
- several active media/document tabs;
- background tabs left idle.

Purpose:

- test cold-process/tab-host-like behavior;
- test whether reclaimed memory simply refaults immediately.

### C. Game + background apps

Example shape:

- latency-sensitive foreground game;
- browser in background;
- chat/voice app;
- launcher/update utility.

Purpose:

- foreground protection;
- background memory-priority demotion;
- detect stutter caused by aggressive trim.

### D. IDE/build workload

Example shape:

- editor/IDE;
- compiler/build process;
- browser documentation tabs;
- terminal tools.

Purpose:

- burst allocation behavior;
- child-process/process-tree awareness;
- avoid classifying temporarily idle build tools as cold too early.

### E. Commit pressure

Purpose:

- distinguish commit exhaustion from simple resident-RAM utilization;
- validate commit-ratio signal;
- ensure MemPilot warns before allocations fail.

### F. Physical memory pressure

Purpose:

- drive available physical memory low while keeping the workload controlled;
- validate low-memory resource notifications and pressure escalation.

### G. Foreground switching

Repeatedly switch between multiple large applications.

Purpose:

- expose bad trim decisions;
- measure recovery/refault time;
- test rapid protection changes.

---

## 4. Core metrics

### MemPilot cost

- average CPU usage
- p95 CPU usage
- own working set
- own private commit
- handles/threads
- sampling wakeup rate where measurable

### System memory

- physical available bytes
- commit total
- commit limit
- commit ratio
- system cache
- paged/nonpaged pool trend

### Fault / paging cost

- page faults per process delta
- hard-fault rate
- page reads/input rate
- page writes/output rate
- disk I/O correlated with intervention window

### Intervention outcome

For each action:

- target process identity
- action type
- action timestamp
- pressure state/score
- candidate score
- resident memory before
- resident memory at T+1s
- resident memory at T+10s
- resident memory at T+60s
- hard faults before/after
- paging I/O before/after
- restored memory amount
- recovery time
- backoff triggered yes/no

---

## 5. Derived metrics

### Immediate reclaim

```text
ImmediateReclaim = WS_before - WS_T+1s
```

Useful only as the first measurement, never as the final score.

### Persistent reclaim

```text
PersistentReclaim = WS_before - WS_T+60s
```

If immediate reclaim is huge but persistent reclaim is near zero, the process likely reloaded quickly.

### Refill ratio

```text
RefillRatio = (WS_T+60s - WS_T+1s) / max(ImmediateReclaim, 1)
```

High refill ratio suggests a poor trim candidate.

### Fault penalty

Normalize post-action hard-fault increase relative to the pre-action baseline.

### Utility score

Exact weights are intentionally not frozen yet.

Conceptual form:

```text
Utility =
    PersistentReclaimBenefit
  - HardFaultPenalty
  - PagingIOPenalty
  - ForegroundLatencyPenalty
  - MemPilotOverheadPenalty
```

---

## 6. Statistical rules

Do not publish one-run results.

Minimum approach:

- warm-up run separated from measured runs;
- multiple repeated runs per mode;
- same workload/order where possible;
- report median and p95/p99 for latency-sensitive metrics;
- include variance/confidence interval where practical;
- preserve raw JSON/CSV output.

Randomize test mode order later if cache/warm-state ordering becomes a bias.

---

## 7. External validation with Windows Performance Toolkit

MemPilot's own telemetry is not enough to validate MemPilot.

Use:

- Windows Performance Recorder (WPR)
- Windows Performance Analyzer (WPA)
- ETW-backed memory graphs/events

Useful WPA areas include:

- Memory Utilization
- Page Faults
- Hard Faults
- VirtualAlloc Commit Lifetimes
- Resident/Reference Set analysis
- pool/driver diagnostics when relevant

Goal:

Correlate MemPilot action timestamps with Windows trace data and verify that the project's interpretation is not hiding a cost.

References:

- https://learn.microsoft.com/windows-hardware/test/wpt/windows-performance-recorder
- https://learn.microsoft.com/windows-hardware/test/wpt/windows-performance-analyzer
- https://learn.microsoft.com/windows-hardware/test/wpt/memory-footprint-optimization
- https://learn.microsoft.com/windows-hardware/test/wpt/list-of-wpa-graphs

---

## 8. v0.1 acceptance gates

### Observer gate

- no mutations;
- correct physical/commit values against trusted Windows views;
- stable long-running sampler;
- access-denied processes handled cleanly;
- no meaningful foreground performance regression.

### Pressure-engine gate

- no HIGH/CRITICAL classification from RAM percentage alone;
- commit exhaustion escalates appropriately;
- transient spikes do not cause oscillation;
- low-memory notification is reflected quickly;
- recovery uses hysteresis rather than instant state flapping.

### Memory-priority gate

- original priority can be recorded/restored;
- foreground process is protected;
- failed mutation is auditable and nonfatal;
- measurable benefit exists in at least one constrained-memory scenario;
- no significant regression in healthy-memory scenarios.

### Trim gate

Selective trim remains disabled by default until all are true:

- positive persistent reclaim in target scenarios;
- bounded hard-fault increase;
- bounded paging-I/O increase;
- no repeated trim/refault loop;
- harmful candidates automatically enter backoff;
- foreground/recently-active protection verified.

---

## 9. Negative-result policy

A benchmark may prove that a proposed feature is not useful.

That is a valid result.

Examples:

- prefetch causes more pressure than it saves;
- page combining state changes cost too much CPU for target workloads;
- `GetWsChangesEx` monitoring is too expensive for always-on mode;
- forced trim only improves Task Manager numbers but worsens application switching.

If evidence is negative, the roadmap should remove or demote the feature rather than tune metrics to justify it.

---

## 10. Benchmark artifact format

Planned output tree:

```text
benchmarks/
  scenarios/
  raw/
  summaries/
  traces/
```

Example run metadata:

```json
{
  "mempilot_version": "0.1.0-dev",
  "git_commit": "...",
  "scenario": "game-background-v1",
  "mode": "observe-only",
  "windows_build": "...",
  "ram_bytes": 17179869184,
  "pagefile_mode": "system-managed",
  "run_index": 3
}
```

Raw data should remain machine-readable so policy changes can be re-evaluated against old benchmark sets.

---

## Success definition

MemPilot wins when a pressured Windows system retains or improves foreground responsiveness while avoiding unnecessary allocation failures and reducing harmful memory contention.

It does **not** win merely because Task Manager's "In use" number becomes smaller.