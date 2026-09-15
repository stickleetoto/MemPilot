# 02 — Commit & Pagefile Pressure

## Mechanism

Windows commit charge and commit limit are distinct from physical RAM usage. The page file contributes to the system commit limit, and hard faults are not synonymous with page-file reads: executable images and memory-mapped files can also back hard faults.

## Why MemPilot cares

A system can be in danger of allocation failure even when physical-memory numbers do not look catastrophic. Raw `RAM used %` misses this failure mode.

## Integration idea

Track:

- `commit_total / commit_limit`;
- distance-to-limit in GiB;
- trend/slope of commit growth;
- `Pages Input/sec`, `Page Reads/sec`, `Page Writes/sec`, `Page Output/sec`;
- pagefile configuration as diagnostic metadata only.

## Policy rule

MemPilot should **never recommend disabling the page file as a generic optimization**. Commit exhaustion is a correctness/stability problem, not merely a performance number.

## Experiment

Generate controlled private-commit growth while leaving some physical RAM available. Compare physical-only pressure scoring with commit-aware scoring and measure which detects danger earlier.

## Status

**Priority: critical.** Commit pressure should remain an independent axis in every future pressure model.
