# 60 — Mapped-File Writeback & Flush Semantics

## Mechanism

`FlushViewOfFile` initiates writing dirty pages from a mapped file view to disk. It does not by itself guarantee that file metadata or hardware caches are fully flushed; durable persistence may also require `FlushFileBuffers`.

## Why MemPilot cares

Mapped-file workloads can produce dirty-memory and storage pressure that looks like ordinary RAM pressure. Forcing reclaim during writeback can make latency worse.

## Integration idea

When diagnostics identify large writable mapped regions, correlate modified-page pressure and disk I/O before ranking the process as a reclaim candidate. Treat explicit application flushes as activity, not idle time.

## Source

- Microsoft Learn: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-flushviewoffile

**Priority: medium-high for database/editor/cache workloads.**