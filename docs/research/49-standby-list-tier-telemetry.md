# 49 — Standby-List Tier Telemetry

## Mechanism

Windows performance counters expose standby cache categories such as Reserve, Normal Priority, and Core bytes, along with free/zero and modified page-list metrics.

## Why MemPilot cares

A machine with little `Free` memory can still have substantial immediately reusable standby memory. Treating all non-free RAM as pressure is therefore wrong.

## Integration idea

Collect standby tiers in diagnose/benchmark mode and use them to explain why a high used-memory percentage may still be healthy. Do not purge standby lists merely to increase a free-memory number.

## Experiment

Compare standby composition against low-memory notifications, page-input rates, and foreground latency across idle-cache-heavy and true-pressure workloads.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/memory/memory-performance-information

**Priority: high for pressure-model calibration.**