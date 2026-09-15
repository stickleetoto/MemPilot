use super::{ProcessKey, ProcessSnapshot, SystemSnapshot};
use std::{
    io,
    mem::{size_of, zeroed},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use windows_sys::Win32::{
    Foundation::{CloseHandle, FILETIME},
    System::{
        Memory::{
            CreateMemoryResourceNotification, LowMemoryResourceNotification,
            QueryMemoryResourceNotification,
        },
        ProcessStatus::{
            K32EnumProcesses, K32GetPerformanceInfo, K32GetProcessMemoryInfo,
            PERFORMANCE_INFORMATION, PROCESS_MEMORY_COUNTERS, PROCESS_MEMORY_COUNTERS_EX,
        },
        SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX},
        Threading::{
            GetProcessTimes, OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
            PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
        },
    },
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
};

pub(super) fn system_snapshot() -> io::Result<SystemSnapshot> {
    unsafe {
        let mut memory: MEMORYSTATUSEX = zeroed();
        memory.dwLength = size_of::<MEMORYSTATUSEX>() as u32;
        if GlobalMemoryStatusEx(&mut memory) == 0 {
            return Err(io::Error::last_os_error());
        }

        let mut perf: PERFORMANCE_INFORMATION = zeroed();
        if K32GetPerformanceInfo(&mut perf, size_of::<PERFORMANCE_INFORMATION>() as u32) == 0 {
            return Err(io::Error::last_os_error());
        }

        let page_size = perf.PageSize as u64;

        Ok(SystemSnapshot {
            sampled_at_unix_ms: unix_ms(),
            total_physical_bytes: memory.ullTotalPhys,
            available_physical_bytes: memory.ullAvailPhys,
            commit_total_bytes: (perf.CommitTotal as u64).saturating_mul(page_size),
            commit_limit_bytes: (perf.CommitLimit as u64).saturating_mul(page_size),
            system_cache_bytes: (perf.SystemCache as u64).saturating_mul(page_size),
            paged_pool_bytes: (perf.KernelPaged as u64).saturating_mul(page_size),
            nonpaged_pool_bytes: (perf.KernelNonpaged as u64).saturating_mul(page_size),
            memory_load_percent: memory.dwMemoryLoad,
            low_memory_signal: query_low_memory_signal(),
        })
    }
}

pub(super) fn process_snapshots() -> io::Result<Vec<ProcessSnapshot>> {
    unsafe {
        let foreground_pid = foreground_pid();
        let pids = enumerate_pids()?;
        let mut snapshots = Vec::with_capacity(pids.len());

        for pid in pids {
            if pid == 0 {
                continue;
            }

            if let Some(snapshot) = inspect_process(pid, foreground_pid) {
                snapshots.push(snapshot);
            }
        }

        Ok(snapshots)
    }
}

unsafe fn enumerate_pids() -> io::Result<Vec<u32>> {
    let mut capacity = 1024usize;

    loop {
        let mut pids = vec![0u32; capacity];
        let mut bytes_needed = 0u32;
        let buffer_bytes = pids
            .len()
            .checked_mul(size_of::<u32>())
            .and_then(|value| u32::try_from(value).ok())
            .ok_or_else(|| io::Error::other("process buffer is too large"))?;

        if unsafe { K32EnumProcesses(pids.as_mut_ptr(), buffer_bytes, &mut bytes_needed) } == 0 {
            return Err(io::Error::last_os_error());
        }

        let count = bytes_needed as usize / size_of::<u32>();
        if count < capacity {
            pids.truncate(count);
            return Ok(pids);
        }

        capacity = capacity
            .checked_mul(2)
            .ok_or_else(|| io::Error::other("process buffer capacity overflow"))?;
    }
}

unsafe fn inspect_process(pid: u32, foreground_pid: u32) -> Option<ProcessSnapshot> {
    let handle = unsafe {
        OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            0,
            pid,
        )
    };

    if handle.is_null() {
        return None;
    }

    let result = (|| {
        let mut counters: PROCESS_MEMORY_COUNTERS_EX = unsafe { zeroed() };
        counters.cb = size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;

        let counters_ok = unsafe {
            K32GetProcessMemoryInfo(
                handle,
                (&mut counters as *mut PROCESS_MEMORY_COUNTERS_EX).cast::<PROCESS_MEMORY_COUNTERS>(),
                size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32,
            )
        };

        if counters_ok == 0 {
            return None;
        }

        let creation_time_100ns = unsafe { process_creation_time(handle) }?;
        let image_path = unsafe { process_image_path(handle) };
        let image_name = image_path
            .as_deref()
            .and_then(|path| Path::new(path).file_name())
            .map(|value| value.to_string_lossy().into_owned())
            .unwrap_or_else(|| format!("pid-{pid}"));

        Some(ProcessSnapshot {
            key: ProcessKey {
                pid,
                creation_time_100ns,
            },
            image_name,
            image_path,
            working_set_bytes: counters.WorkingSetSize as u64,
            peak_working_set_bytes: counters.PeakWorkingSetSize as u64,
            private_commit_bytes: counters.PrivateUsage as u64,
            page_fault_count: counters.PageFaultCount as u64,
            foreground: pid == foreground_pid,
        })
    })();

    let _ = unsafe { CloseHandle(handle) };
    result
}

unsafe fn process_image_path(handle: windows_sys::Win32::Foundation::HANDLE) -> Option<String> {
    let mut buffer = vec![0u16; 32_768];
    let mut size = buffer.len() as u32;

    if unsafe {
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            buffer.as_mut_ptr(),
            &mut size,
        )
    } == 0
    {
        return None;
    }

    Some(String::from_utf16_lossy(&buffer[..size as usize]))
}

unsafe fn process_creation_time(
    handle: windows_sys::Win32::Foundation::HANDLE,
) -> Option<u64> {
    let mut creation: FILETIME = unsafe { zeroed() };
    let mut exit: FILETIME = unsafe { zeroed() };
    let mut kernel: FILETIME = unsafe { zeroed() };
    let mut user: FILETIME = unsafe { zeroed() };

    if unsafe { GetProcessTimes(handle, &mut creation, &mut exit, &mut kernel, &mut user) } == 0 {
        return None;
    }

    Some(filetime_to_u64(creation))
}

fn filetime_to_u64(value: FILETIME) -> u64 {
    ((value.dwHighDateTime as u64) << 32) | value.dwLowDateTime as u64
}

unsafe fn foreground_pid() -> u32 {
    let window = unsafe { GetForegroundWindow() };
    if window.is_null() {
        return 0;
    }

    let mut pid = 0u32;
    let _ = unsafe { GetWindowThreadProcessId(window, &mut pid) };
    pid
}

unsafe fn query_low_memory_signal() -> Option<bool> {
    let handle = unsafe { CreateMemoryResourceNotification(LowMemoryResourceNotification) };
    if handle.is_null() {
        return None;
    }

    let mut state = 0;
    let ok = unsafe { QueryMemoryResourceNotification(handle, &mut state) };
    let _ = unsafe { CloseHandle(handle) };

    if ok == 0 {
        None
    } else {
        Some(state != 0)
    }
}

fn unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
        .unwrap_or(0)
}
