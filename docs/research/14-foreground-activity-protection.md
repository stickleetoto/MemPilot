# 14 — Foreground & Activity Protection

## Problem

The active user workload can include more than the executable owning the foreground window: renderers, game launchers, child processes, audio helpers, compilers, terminals, and short-lived workers may belong to one interactive task.

## Signals to research

- foreground-window PID;
- process ancestry / job grouping where available;
- recent CPU delta;
- recent page-fault delta;
- working-set growth;
- visible-window state;
- audio/session activity candidate;
- user pin/allowlist;
- recently-foreground grace period.

## Proposed model

Classify processes as:

```text
ACTIVE -> WARM -> COLD -> DORMANT
```

Transitions should require time, not one quiet sample. ACTIVE/WARM should receive strong protection from mutation.

## Experiment

Record task switches across a browser, IDE build, game, media playback, and terminal workload. Test how quickly a pure foreground-PID rule incorrectly marks supporting processes as cold.

## Status

**Priority: high.** Needed to reduce the biggest class of user-visible false positives.
