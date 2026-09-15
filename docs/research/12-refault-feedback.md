# 12 — Refault Feedback & Reclaim Persistence

## Problem

A memory intervention is useless if pages return immediately. `freed MB` at T+0 is therefore not a success metric.

## Core metrics

For each intervention create an outcome record:

- resident bytes before;
- resident bytes at T+1/T+10/T+60;
- process page-fault deltas;
- system hard-fault/paging-I/O deltas;
- foreground-latency proxy;
- time to regain 25/50/75% of reclaimed residency.

## Derived scores

- **persistence** = retained reclaim after a time horizon;
- **refault penalty** = normalized post-action fault burst;
- **useful reclaim** = persistence × reclaimed bytes;
- **harm score** = I/O + refault + responsiveness penalties.

## Adaptive use

Bad outcomes increase per-executable cooldown and risk score. Good outcomes increase confidence slowly. One outlier must not permanently poison an executable's history.

## Experiment

Run repeated trims/demotions on cold and hot synthetic processes and measure whether the metrics separate the two populations.

## Status

**Priority: critical.** Required before selective trim can be considered safe.
