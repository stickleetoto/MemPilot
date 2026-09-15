# MemPilot Roadmap

MemPilot is being designed as a **Windows memory-pressure governor**, not a cosmetic RAM cleaner.

The project follows four rules:

1. **Observe before mutate.** Every active policy must first exist as a measurable dry-run policy.
2. **Prefer hints over force.** Memory-priority demotion comes before working-set trimming.
3. **Protect responsiveness.** Foreground, protected, explicitly pinned, and recently active workloads win over reclaim targets.
4. **Evidence beats reclaimed-MB screenshots.** A change only graduates when reclaim benefit exceeds refault, I/O, latency, and CPU cost.

---

## v0.1 — Safe measurable baseline

Goal: build a trustworthy observer, pressure model, process profiler, dry-run engine, and one conservative intervention path.

### Phase 0 — Research and contract freeze

- [x] Rust project skeleton
- [x] pressure-level model skeleton
- [x] policy/action boundary
- [x] architecture and safety invariants
- [x] Windows CI baseline
- [x] initial Win32 technology survey
- [ ] freeze supported Windows versions for v0.1
- [ ] choose/freeze `windows` vs `windows-sys` binding surface
- [ ] create Win32 API capability matrix: API, minimum OS, required access rights, failure/fallback behavior
- [ ] define telemetry snapshot schema
- [ ] define process identity model (`pid` + creation time; PID alone is not stable identity)
- [ ] define monotonic sampling/timestamp rules
- [ ] define config schema versioning
- [ ] define error taxonomy: unsupported / access-denied / transient / partial-data / fatal
- [ ] freeze v0.1 safety invariants

Exit condition: implementation can begin without architectural ambiguity.

### Phase 1 — System telemetry

Primary APIs/signals:

- `GetPerformanceInfo`
- `GlobalMemoryStatusEx`
- `CreateMemoryResourceNotification` / `QueryMemoryResourceNotification`
- Windows performance counters / PDH where API snapshots do not provide rate data

Tasks:

- [ ] physical total / available memory
- [ ] commit total / commit limit / commit ratio
- [ ] system cache visibility
- [ ] paged / nonpaged pool visibility
- [ ] low-memory notification integration
- [ ] high-memory notification integration for hysteresis/recovery
- [ ] hard-fault indicators (`Pages Input/sec`, `Page Reads/sec`, or equivalent sampled rate)
- [ ] paging write indicators for diagnostics
- [ ] pressure trend / rate-of-change fields
- [ ] sampler overhead measurement
- [ ] periodic snapshot loop
- [ ] `mempilot status`
- [ ] `mempilot watch`
- [ ] JSON output mode

Exit condition: MemPilot can continuously observe real memory pressure with no system mutation and low overhead.

### Phase 2 — Process inventory and profiler

Primary APIs/signals:

- process enumeration
- `QueryFullProcessImageName`
- `GetProcessMemoryInfo` + `PROCESS_MEMORY_COUNTERS_EX2` where available
- `GetProcessInformation(ProcessMemoryPriority)`
- foreground-window detection
- process times / CPU activity deltas

Tasks:

- [ ] enumerate processes with least-required access rights
- [ ] process executable path/name
- [ ] stable process identity using PID + creation time
- [ ] working set / peak working set
- [ ] private working set
- [ ] private commit / shared commit where supported
- [ ] page-fault count deltas
- [ ] CPU activity delta
- [ ] foreground-process detection
- [ ] process age / recent activity signals
- [ ] process memory-priority observation
- [ ] protected/inaccessible process classification
- [ ] service/system-process exclusion policy
- [ ] allowlist / denylist / pin model
- [ ] `mempilot analyze`

Exit condition: MemPilot can explain which processes consume reclaimable memory without assuming every inaccessible process is an error.

### Phase 3 — Pressure Engine v1

A single RAM percentage is not enough. The first pressure score should combine multiple independent signals.

Candidate inputs:

- physical-available ratio
- commit ratio
- low-memory resource notification state
- hard-fault rate
- paging-I/O rate
- pressure slope / trend

Tasks:

- [ ] normalized signal model
- [ ] `NORMAL / WATCH / PRESSURE / HIGH / CRITICAL` state machine
- [ ] entry/exit hysteresis to prevent rapid oscillation
- [ ] minimum dwell time per state
- [ ] spike rejection / smoothing
- [ ] low-memory event override
- [ ] 8 GB calibration scenario
- [ ] 16 GB calibration scenario
- [ ] 32 GB+ sanity scenario
- [ ] false-positive tests: high RAM use with low actual pressure
- [ ] false-negative tests: commit exhaustion with apparently available physical memory

Exit condition: pressure classification correlates with real contention better than raw RAM-used percentage.

### Phase 4 — Candidate scoring and dry-run policy

Candidate score should estimate **expected useful reclaim**, not simply rank by working-set size.

Inputs may include:

- private working set
- private commit
- foreground state
- recent CPU/activity
- recent page-fault activity
- current memory priority
- process age
- previous MemPilot intervention history
- protection/allowlist rules

Tasks:

- [ ] coldness score
- [ ] reclaim-potential score
- [ ] refault-risk score
- [ ] composite candidate score
- [ ] action cooldown model
- [ ] per-process exclusion reason
- [ ] deterministic explain output
- [ ] `mempilot optimize --dry-run`
- [ ] dry-run action plan serialization
- [ ] before/after metric schema
- [ ] policy unit tests against synthetic process snapshots

Exit condition: every proposed action answers **what, why, expected gain, risk, and why now**.

### Phase 5 — Memory-priority governor

First real optimization action: use documented memory priority as a hint to the Windows Memory Manager.

Primary APIs:

- `GetProcessInformation(ProcessMemoryPriority)`
- `SetProcessInformation(ProcessMemoryPriority)`

Tasks:

- [ ] read and preserve original priority
- [ ] conservative demotion ladder
- [ ] foreground auto-protection
- [ ] recent-activity protection
- [ ] TTL for temporary demotion
- [ ] automatic restore path
- [ ] restore on MemPilot exit where appropriate
- [ ] audit log of every mutation
- [ ] access-denied handling without privilege escalation loops
- [ ] default-off mutation gate until benchmark acceptance

Exit condition: MemPilot can apply and safely undo a low-risk memory policy change.

### Phase 6 — Selective working-set trim

Primary APIs:

- `EmptyWorkingSet`
- `SetProcessWorkingSetSizeEx`

This phase is intentionally later because immediate reclaimed RAM can hide costly refaults.

Tasks:

- [ ] explicit trim eligibility rules
- [ ] HIGH/CRITICAL-only default policy
- [ ] minimum cold-time threshold
- [ ] minimum expected reclaim threshold
- [ ] never trim current foreground process
- [ ] never trim pinned/protected/inaccessible categories
- [ ] per-process cooldown
- [ ] global trim-rate limit
- [ ] reclaim measurement at T+1s / T+10s / T+60s
- [ ] page-fault/refault penalty
- [ ] paging-I/O penalty
- [ ] automatic process-level backoff
- [ ] repeated-thrash detection
- [ ] `--aggressive` remains explicit opt-in

Exit condition: trim produces repeatable useful reclaim without causing sustained fault storms or responsiveness regressions.

### Phase 7 — Feedback Engine v1

Primary technologies:

- sampled process counters
- `InitializeProcessForWsWatch` / `GetWsChangesEx` experiments
- optional targeted `QueryWorkingSetEx`
- performance-counter correlation

Tasks:

- [ ] intervention outcome object
- [ ] reclaimed/resident delta
- [ ] refault activity delta
- [ ] hard-fault delta
- [ ] paging-I/O delta
- [ ] recovery time measurement
- [ ] per-process penalty score
- [ ] temporary auto-blacklist after harmful intervention
- [ ] global emergency backoff
- [ ] explain why a previous candidate is no longer eligible

Exit condition: MemPilot learns **not to repeat a bad action** during the current run.

### Phase 8 — Benchmark and v0.1 release gate

Validation technologies:

- Windows Performance Recorder (WPR)
- Windows Performance Analyzer (WPA)
- ETW traces as an external ground truth

Required scenarios:

- [ ] idle desktop baseline
- [ ] browser-heavy workload
- [ ] game + browser + Discord-style background workload
- [ ] IDE/build workload
- [ ] artificial commit-pressure workload
- [ ] physical-memory-pressure workload
- [ ] repeated foreground switching
- [ ] pagefile-enabled normal Windows configuration

Release metrics:

- [ ] MemPilot CPU overhead
- [ ] MemPilot own working set/private commit
- [ ] useful reclaim MB
- [ ] reclaim persistence
- [ ] hard-fault increase
- [ ] paging-I/O increase
- [ ] foreground latency/regression proxy
- [ ] number of policy reversals/backoffs

v0.1 release rule: **no default-on intervention without benchmark evidence**.

---

## v0.2 — Adaptive governor

Goal: move from one-run feedback to persistent, explainable adaptation.

### Phase 9 — Persistent intervention history

- [ ] local per-executable history
- [ ] separate identity from PID
- [ ] rolling reclaim effectiveness
- [ ] rolling refault penalty
- [ ] versioned local state format
- [ ] retention/expiry policy
- [ ] reset/export commands
- [ ] privacy rule: no remote telemetry by default

### Phase 10 — Adaptive scoring

- [ ] process-specific trim tolerance
- [ ] process-specific demotion tolerance
- [ ] adaptive cooldown duration
- [ ] candidate-score calibration from local history
- [ ] confidence score / minimum evidence count
- [ ] deterministic fallback when history is absent
- [ ] prevent one anomalous run from permanently poisoning a process score

### Phase 11 — Workload awareness

- [ ] game/latency-sensitive mode
- [ ] build/compile mode
- [ ] VM/container-heavy mode
- [ ] battery/power-aware mode candidate
- [ ] foreground-chain protection
- [ ] child-process relationship awareness
- [ ] user-defined profiles
- [ ] automatic profile suggestions, never silent irreversible changes

### Phase 12 — Benchmark harness

- [ ] reproducible workload runner
- [ ] 8 GB / 16 GB / 32 GB test profiles
- [ ] A/B: Windows default vs MemPilot observe-only vs MemPilot active
- [ ] JSON/CSV export
- [ ] percentile reporting, not only averages
- [ ] confidence intervals / repeated runs
- [ ] benchmark version stamped into results

---

## v0.3 — Deep diagnostics

Goal: understand *why* memory pressure exists, not merely react to it.

### Phase 13 — ETW memory diagnostics

Candidate provider keywords include system memory general, hard faults, all faults, working-set, footprint, and virtual allocation events.

- [ ] optional ETW session integration
- [ ] hard faults by process/file
- [ ] fault burst detection
- [ ] virtual allocation/commit trend diagnostics
- [ ] working-set event diagnostics
- [ ] driver pool growth observation where feasible
- [ ] ETW overhead guardrails
- [ ] diagnostic mode separate from lightweight default sampler

### Phase 14 — Leak / growth diagnostics

- [ ] private-commit trend detection
- [ ] working-set growth trend
- [ ] paged/nonpaged pool trend warnings
- [ ] process restart boundaries
- [ ] distinguish burst allocation from sustained growth
- [ ] diagnostics only: no automatic termination

### Phase 15 — Deep working-set inspection

Candidate technology:

- `QueryWorkingSetEx`

Potential uses:

- [ ] targeted page-attribute sampling
- [ ] estimate shared-vs-private residency quality
- [ ] locked/large-page awareness
- [ ] NUMA metadata observation on supported systems
- [ ] never use full address-space scanning in the normal fast path

---

## v0.4 — Managed workload controls

Goal: provide stronger controls only for workloads that MemPilot owns or explicitly launches.

### Phase 16 — Job Object integration

- [ ] launch workloads inside a Job Object
- [ ] observe peak process/job memory
- [ ] soft notification limits first
- [ ] optional process/job commit budgets
- [ ] child-process accounting
- [ ] clear failure semantics when an allocation would exceed a hard limit
- [ ] hard memory caps remain explicit opt-in

Possible CLI shape:

```text
mempilot run --profile build -- command.exe
mempilot run --memory-notify 6G -- command.exe
mempilot run --hard-limit 8G -- command.exe   # advanced/explicit
```

---

## v0.5 — Recovery and prefetch experiments

Goal: investigate whether selected post-pressure workloads can recover residency faster without creating new pressure.

Candidates:

- `PrefetchVirtualMemory`
- Windows Memory Management Agent feature-state inspection
- application-launch prefetch / Operation Recorder research

Tasks:

- [ ] identify safe, bounded prefetch use cases
- [ ] prove prefetch does not simply trade faults for excess pressure
- [ ] only prefetch known/justified ranges
- [ ] inspect MMAgent feature state
- [ ] report memory compression / page combining state where reliably available
- [ ] do not silently toggle OS memory-management features by default

This version is experimental and may be dropped if benchmarks show no practical value.

---

## v0.6 — Productization

### Phase 17 — Configuration and UX

- [ ] stable CLI contract
- [ ] TOML/JSON configuration
- [ ] `status`, `watch`, `analyze`, `optimize`, `history`, `doctor`
- [ ] human-readable reason codes
- [ ] optional tray UI
- [ ] optional notifications
- [ ] safe defaults for non-technical users

### Phase 18 — Optional background service

- [ ] evaluate whether a service is actually required
- [ ] split privileged mutation component from unprivileged UI if needed
- [ ] least-privilege IPC design
- [ ] service install/uninstall lifecycle
- [ ] watchdog and crash recovery
- [ ] no always-on service if a user-session agent is sufficient

### Phase 19 — Packaging

- [ ] signed release artifacts
- [ ] reproducible builds where practical
- [ ] portable single-exe build
- [ ] installer candidate
- [ ] update policy
- [ ] release notes and compatibility matrix

---

## v0.7 — Policy hardening

- [ ] fuzz/config parser testing
- [ ] long-duration soak test
- [ ] PID-reuse tests
- [ ] rapid process create/exit tests
- [ ] privilege boundary tests
- [ ] protected-process tests
- [ ] resume-from-sleep tests
- [ ] multiple-user-session tests
- [ ] Windows update/version compatibility tests
- [ ] corrupt history/config recovery

---

## v0.8 — Comparative evaluation

- [ ] compare against Windows default behavior
- [ ] compare against observe-only mode
- [ ] compare memory-priority-only vs trim-enabled policy
- [ ] test with/without pagefile pressure
- [ ] publish reproducible benchmark methodology
- [ ] document workloads where MemPilot does **not** help

---

## v0.9 — Release candidate

- [ ] default policy freeze
- [ ] CLI/config freeze
- [ ] migration tests
- [ ] documentation audit
- [ ] licensing audit
- [ ] security review
- [ ] performance regression suite
- [ ] known-limitations document

---

## v1.0 — Evidence-based Windows memory governor

v1.0 should ship only when the following are true:

- system pressure can be measured reliably;
- foreground workloads are protected by default;
- all active interventions are explainable and auditable;
- original process policy can be restored;
- harmful candidates trigger backoff;
- benchmark data demonstrates useful scenarios;
- MemPilot remains useful in observe-only mode;
- no undocumented kernel dependency is required for the default product.

---

## Technology tiers

### Tier A — Core / expected to ship

- documented system memory APIs
- documented process memory APIs
- memory resource notifications
- memory-priority observation/change
- performance counters where needed
- foreground protection
- dry-run policy engine
- reversible audit trail

### Tier B — Advanced / likely useful

- `GetWsChangesEx`
- targeted `QueryWorkingSetEx`
- ETW memory diagnostics
- WPR/WPA validation
- selective working-set trim with feedback
- Job Object notification limits

### Tier C — Experimental

- targeted prefetch/recovery
- hard Job Object memory limits
- MMAgent feature-state integrations
- automated profile suggestions

### Tier D — Explicitly rejected for default behavior

- global standby-list purge loops
- arbitrary system-file-cache flushing
- automatic page-file disabling or shrinking
- undocumented `NtSetSystemInformation` memory-list hacks
- kernel driver/hooks solely to force reclaim
- `PROCESS_ALL_ACCESS` as the normal process-open strategy
- automatic process termination as routine optimization
- repeated working-set trimming solely to make Task Manager show a lower number

---

## Permanent release rule

A MemPilot feature does not graduate because it frees memory.

It graduates only when testing shows that the reclaimed memory is **useful**, the system remains **responsive**, refault/paging costs are **acceptable**, and MemPilot can explain **why the action was taken**.