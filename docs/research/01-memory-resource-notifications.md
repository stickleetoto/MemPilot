# 01 — Memory Resource Notifications

## Mechanism

Windows exposes system-wide low/high available-memory notification objects through `CreateMemoryResourceNotification`, queryable with `QueryMemoryResourceNotification` and usable with wait functions.

## Why MemPilot cares

This is an OS-provided pressure signal, not a guessed percentage threshold. It can be used as a high-confidence override or wake-up trigger for the pressure engine.

## Integration idea

- Keep low and high notification handles alive instead of recreating them every sample.
- Feed low-memory state into the pressure engine as a strong signal.
- Use high-memory state as recovery evidence.
- Preserve a neutral band where neither event is signaled; do not infer high/low from absence alone.

## Risks

The event is coarse and should not replace physical/commit/fault telemetry. A low-memory event says pressure exists; it does not say which process should be acted on.

## Experiment

Record notification transitions alongside available RAM, commit ratio, page-input rate, and foreground latency during controlled allocation ramps. Measure lead/lag versus MemPilot's own pressure state.

## Status

**Priority: high.** v0.2 already samples the low-memory state; next step is persistent event handles plus transition timing.
