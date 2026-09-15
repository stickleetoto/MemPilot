# 13 — Stable Process Identity

## Problem

A PID is recyclable. If MemPilot stores cooldown/protection state by PID alone, a newly started unrelated process can inherit stale decisions.

## Identity model

Use at minimum:

```text
ProcessKey = PID + process creation FILETIME
```

For persistent per-application history, separate process instance identity from application identity:

```text
InstanceKey = PID + creation_time
AppKey      = normalized executable identity
```

Potential AppKey inputs later: canonical path, file ID, publisher/signature metadata, version, user scope.

## Safety rule

Every mutation must revalidate that the target handle/process identity still matches the decision snapshot immediately before acting.

## Experiment

Spawn/exit processes quickly enough to force PID reuse and verify that cooldowns, restoration maps, and candidate records never cross process generations.

## Status

**Priority: critical safety infrastructure.** v0.2 already uses PID + creation time; extend it before mutation history becomes persistent.
