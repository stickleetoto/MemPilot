# MemPilot

**Adaptive memory pressure manager and RAM optimizer for Windows, written in Rust.**

> Don't clean memory. Manage pressure.

MemPilot is an experimental Windows memory policy engine. Instead of chasing a lower "RAM used" number, it is designed to observe real memory pressure, protect active workloads, rank background processes, and intervene only when the system is actually under pressure.

## Status

**Pre-alpha / v0.1 foundation**

The repository currently contains the policy/pressure-engine skeleton. Live Windows telemetry and memory-management actions are the next implementation milestone.

## Design goals

- Measure physical-memory and commit pressure instead of blindly clearing RAM.
- Prefer low-risk policy changes before forced working-set trimming.
- Protect foreground and explicitly allow-listed processes.
- Use selective trimming only when pressure is high enough to justify it.
- Measure page-fault/regression feedback after every intervention.
- Keep aggressive or destructive behavior opt-in.
- Be a small, auditable Windows utility rather than a permanent heavyweight service.

## Non-goals

MemPilot is **not** intended to:

- purge standby/file cache just to make the free-RAM number look larger;
- disable the Windows page file as a "performance tweak";
- repeatedly call working-set trim APIs on every process;
- terminate applications automatically in v0.1;
- claim performance improvements without before/after measurements.

## Planned control loop

```text
Telemetry
   |
   v
Pressure Engine -----> Process Profiler
   |                         |
   +-----------+-------------+
               v
          Policy Engine
               |
     +---------+---------+
     |         |         |
   Observe   Demote    Selective
             memory      trim
            priority
     \         |         /
      +--------+--------+
               v
         Feedback Loop
               |
         regression?
          /        \
        yes         no
        |            |
   back off       keep policy
```

## Pressure model

The initial engine uses a provisional `0..100` pressure score from:

- physical memory pressure;
- committed-memory pressure;
- hard-page-fault activity.

The heuristic is intentionally isolated so it can be benchmarked and replaced without rewriting the rest of the system.

```text
0-29    NORMAL
30-54   WATCH
55-74   PRESSURE
75-89   HIGH
90-100  CRITICAL
```

## Repository layout

```text
src/
  telemetry/   System and process observations
  pressure/    Pressure scoring/classification
  policy/      Safe policy decisions
  optimizer/   Action planning/execution boundary
docs/
  ARCHITECTURE.md
  ROADMAP.md
```

## Development

Requires a recent stable Rust toolchain.

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run -- status
```

The live optimizer will target Windows. Pure policy logic should remain testable without invoking Windows memory-management APIs.

## Safety rule

**No action should be considered an optimization unless MemPilot can measure its effect.**

Every future mutating action should support dry-run, cooldown, and rollback/backoff behavior where applicable.

## Licensing

No software license is currently granted for reuse or redistribution. Commercial use requires a separate written license from the copyright holder. See [`COMMERCIAL.md`](COMMERCIAL.md).

A dedicated non-commercial/source-available license may be adopted before the first public release.
