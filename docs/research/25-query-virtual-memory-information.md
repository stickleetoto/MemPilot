# 25 — QueryVirtualMemoryInformation / MemoryRegionInfo

## Mechanism

`QueryVirtualMemoryInformation` can return `WIN32_MEMORY_REGION_INFORMATION` for a valid allocation in another process. It is a newer region-oriented API available from Windows 10 version 1607.

## Why MemPilot cares

It can complement `VirtualQueryEx` with higher-level allocation-region metadata and become a capability-gated diagnostic path on modern Windows.

## Integration idea

- runtime-detect support;
- use only after identifying an interesting region;
- keep `VirtualQueryEx` as the broad compatibility baseline;
- serialize region metadata into diagnostic JSON without turning it into a default policy signal until validated.

## Risks

The API is narrower than a full address-space walk and fails for unallocated addresses. Do not assume availability on older systems.

## Experiment

Cross-check its region boundaries/metadata against a `VirtualQueryEx` walk for several process types and Windows versions.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-queryvirtualmemoryinformation

**Priority: medium-high for modern Windows diagnostics.**