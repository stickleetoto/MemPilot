# 36 — Installed RAM vs OS-Usable RAM

## Mechanism

`GetPhysicallyInstalledSystemMemory` reads installed RAM from SMBIOS, while `GlobalMemoryStatusEx` reports physical memory available to Windows. Firmware, devices, and drivers can reserve part of installed RAM.

## Why MemPilot cares

MemPilot should distinguish **hardware capacity** from **OS-usable capacity**. Otherwise users may think memory is missing or the optimizer is wrong when several hundred MiB or more are hardware-reserved.

## Integration idea

Expose both values in diagnostics:

- Installed RAM
- OS-usable RAM
- Hardware/firmware reserved delta

Use OS-usable RAM for pressure calculations, not the SMBIOS value.

## Experiment

Collect the delta across desktops, laptops, integrated-GPU machines, VMs, and servers; compare with Task Manager's hardware-reserved display where available.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getphysicallyinstalledsystemmemory

**Priority: medium, low-cost diagnostic quality.**