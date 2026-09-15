# 27 — Process Snapshotting (PSS)

## Mechanism

Windows Process Snapshotting can capture a target process with selectable data such as virtual-address metadata, section information, threads, handles, and optionally a VA clone. `PssCaptureSnapshot` is available from Windows 8.1.

## Why MemPilot cares

PSS can give MemPilot a consistent-ish diagnostic snapshot without hand-walking every live structure while it changes underneath. It is suitable for one-shot forensic reports and benchmark investigations, not the fast sampling loop.

## Integration idea

Create `mempilot diagnose snapshot --pid <PID>` with the lightest useful flags first: `PSS_CAPTURE_VA_SPACE` and optionally `PSS_CAPTURE_VA_SPACE_SECTION_INFORMATION`. Avoid VA clone/thread-context capture unless explicitly requested.

## Risks

Snapshots have cost. More capture flags mean more latency/memory and greater access requirements. Never capture page contents by default.

## Experiment

Measure capture latency and MemPilot overhead across 100 MiB, 1 GiB, 5 GiB and multi-process workloads. Compare PSS-derived VA summaries with live `VirtualQueryEx` scans.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/processsnapshot/nf-processsnapshot-psscapturesnapshot
- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/processsnapshot/ne-processsnapshot-pss_capture_flags

**Priority: medium-high for forensic diagnostics.**