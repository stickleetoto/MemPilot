# 54 — Bad Memory Notifications

## Mechanism

`RegisterBadMemoryNotification` lets an application register a callback when Windows detects bad memory pages that it cannot fully remove from use.

## Why MemPilot cares

Rare hardware-memory faults can contaminate diagnostics: apparent instability under pressure may actually be a hardware reliability event. MemPilot should be able to distinguish these from policy failures.

## Integration idea

Use this only as a reliability signal for MemPilot itself and diagnostics. Log the event, disable active mutations, and advise hardware validation rather than attempting to compensate in software.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-registerbadmemorynotification

**Priority: low-frequency but high-value reliability telemetry.**