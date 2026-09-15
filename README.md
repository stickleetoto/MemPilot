# MemPilot

**Adaptive memory pressure manager and RAM optimizer for Windows, written in Rust.**

> Don't clean memory. Manage pressure.

MemPilot is an experimental Windows memory-pressure governor. Instead of chasing a lower "RAM used" number, it observes physical and commit pressure, protects the foreground workload, profiles processes, and ranks background reclaim candidates.

## Status

**v0.2.0 — functional observer + adaptive dry-run governor**

MemPilot now has live Windows system/process telemetry, a pressure state machine with smoothing/hysteresis, adaptive in-run candidate scoring, JSON output, and a safe optimization planner.

**v0.2 does not mutate another process.** Active memory-priority changes and working-set trimming remain behind later validation gates.

## What works

- Physical RAM / available RAM telemetry
- Commit charge / commit limit
- System cache and kernel pool visibility
- Windows low-memory resource signal
- Process enumeration and executable paths
- Working set, peak working set, private commit, page-fault counters
- Stable process identity using PID + creation time
- Foreground-process protection
- Pressure score: `NORMAL / WATCH / PRESSURE / HIGH / CRITICAL`
- Candidate ranking with page-fault risk feedback
- `status`, `analyze`, `watch`, and `optimize --dry-run`
- JSON output for status/analyze/optimization plans

## CLI

```powershell
cargo run -- status
cargo run -- status --json

cargo run -- analyze --top 15
cargo run -- analyze --top 15 --json

cargo run -- watch --interval 2 --count 30

cargo run -- optimize --dry-run --top 10
cargo run -- optimize --dry-run --top 10 --json
```

Example control loop:

```text
Windows telemetry
      |
      v
Pressure Engine
      |
      +------> Process profiler
      |              |
      +--------------+
             |
             v
      Candidate ranking
             |
             v
      Dry-run policy plan
             |
             v
      Adaptive feedback
```

## Safety rules

MemPilot v0.2 will **not**:

- purge standby/file cache to make free-RAM screenshots look better;
- disable or resize the Windows page file;
- repeatedly trim every process;
- terminate applications;
- modify process memory priority;
- execute a working-set trim.

`mempilot optimize` requires `--dry-run` and reports what a future governor could consider doing.

## Development

Requires a recent stable Rust toolchain. Windows is the live telemetry target.

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

See:

- [`docs/ROADMAP.md`](docs/ROADMAP.md)
- [`docs/TECH_RESEARCH.md`](docs/TECH_RESEARCH.md)
- [`docs/BENCHMARK_PLAN.md`](docs/BENCHMARK_PLAN.md)
- [`docs/V0_2_STATUS.md`](docs/V0_2_STATUS.md)

## Licensing

No software license is currently granted for reuse or redistribution. Commercial use requires a separate written license from the copyright holder. See [`COMMERCIAL.md`](COMMERCIAL.md).

A dedicated non-commercial/source-available license may be adopted before the first public release.
