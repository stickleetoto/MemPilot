# 10 — WPR/WPA Reference Sets

## Mechanism

Windows Performance Recorder / Analyzer can collect reference-set information. Microsoft describes reference sets as a better way than a momentary working set to understand the memory a scenario may take away from the rest of the system and the pages that may need to be faulted back if evicted.

## Why MemPilot cares

Working set alone is heavily affected by machine RAM size, current pressure, and Windows trimming policy. Reference-set analysis gives MemPilot an **external ground truth** for whether an app is truly costly to evict.

## Use in validation

For each benchmark scenario:

1. record baseline Windows run;
2. record MemPilot observe-only run;
3. record active-policy run;
4. inspect reference-set outstanding size, page faults, hard faults, disk I/O, and focus/UI timing.

## Product implication

Do not embed WPA itself in MemPilot. Use WPR/WPA as a development and release-gate tool.

## Status

**Priority: high.** Key external validator for any claim that candidate scoring reflects real memory cost.
