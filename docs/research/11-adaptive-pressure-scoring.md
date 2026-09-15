# 11 — Adaptive Pressure Scoring

## Problem

A static `RAM used > X%` threshold cannot distinguish healthy cache use from real contention, nor can it react correctly to commit exhaustion or fault storms.

## Candidate signals

- physical available ratio;
- commit ratio and distance-to-limit;
- low/high memory notification state;
- Pages Input/sec / Page Reads/sec;
- paging writes;
- pressure slope;
- foreground-latency proxy;
- time spent in current state.

## Control techniques to stockpile

- EWMA/EMA smoothing;
- hysteresis with separate enter/exit thresholds;
- minimum dwell time;
- derivative/slope limiter;
- spike rejection / median-of-small-window;
- confidence weighting when a signal is unavailable;
- emergency override for explicit low-memory notification.

## Design principle

The score is not the product. The **state transition quality** is. A slightly noisy score is acceptable if NORMAL/WATCH/PRESSURE/HIGH/CRITICAL transitions are stable and early enough.

## Experiment

Replay captured telemetry traces through multiple scoring models offline and compare transition timing, oscillation count, false positives, and missed pressure events.

## Status

**Priority: critical.** This should evolve before aggressive mutations exist.
