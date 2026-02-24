use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{arg_value, parse_u64, state, WaygateResult};

pub const SUPPORTED: &[&str] = &[
    "CreateFileA",
    "ReadFile",
    "WriteFile",
    "CloseHandle",
    "VirtualAlloc",
    "VirtualFree",
    "GetLastError",
    "SetLastError",
    "ExitProcess",
    "GetCurrentProcess",
    "Sleep",
    "GetTickCount",
    "GetModuleHandle",
    "GetProcAddress",
    "LoadLibrary",
    "FreeLibrary",
    "CreateThread",
    "WaitForSingleObject",
    "CreateEvent",
    "SetEvent",
    "ResetEvent",
    "QueryPerformanceCounter",
    "QueryPerformanceFrequency",
    "GetSystemTime",
    "GetLocalTime",
];

pub fn dispatch(symbol: &str, args: &[String]) -> Option<WaygateResult> {
    let res = match symbol {
        "Sleep" => sleep_impl(args),
        "GetTickCount" => Ok(format!("[waygate::kernel32] tick={}", tick_ms())),
        "SetLastError" => set_last_error(args),
        "GetLastError" => get_last_error(),
        "CreateEvent" => create_event(args),
        "SetEvent" => set_event(args, true),
        "ResetEvent" => set_event(args, false),
        "WaitForSingleObject" => wait_for_single_object(args),
        "QueryPerformanceCounter" => Ok(format!("[waygate::kernel32] qpc={}", tick_ms() * 1_000)),
        "QueryPerformanceFrequency" => Ok("[waygate::kernel32] qpf=1000000".to_string()),
        "GetSystemTime" => time_now("system"),
        "GetLocalTime" => time_now("local"),
        "CreateThread" => {
            Ok("[waygate::kernel32] created synthetic thread handle=2000".to_string())
        }
        "CreateFileA" | "ReadFile" | "WriteFile" | "CloseHandle" | "VirtualAlloc"
        | "VirtualFree" | "ExitProcess" | "GetCurrentProcess" | "GetModuleHandle"
        | "GetProcAddress" | "LoadLibrary" | "FreeLibrary" => ok(symbol, args),
        _ => return None,
    };
    Some(res)
}

fn sleep_impl(args: &[String]) -> WaygateResult {
    let ms = parse_u64(arg_value(args, "ms"), 0);
    thread::sleep(Duration::from_millis(ms));
    Ok(format!("[waygate::kernel32] slept_ms={ms}"))
}

// keep tickcount monotonic like windows uptime.
fn tick_ms() -> u64 {
    state().started.elapsed().as_millis() as u64
}

fn set_last_error(args: &[String]) -> WaygateResult {
    let code = arg_value(args, "code")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);
    state()
        .last_error
        .store(code, std::sync::atomic::Ordering::Relaxed);
    Ok(format!("[waygate::kernel32] last_error_set={code}"))
}

fn get_last_error() -> WaygateResult {
    let code = state()
        .last_error
        .load(std::sync::atomic::Ordering::Relaxed);
    Ok(format!("[waygate::kernel32] last_error={code}"))
}

// map event handles into local linux-side state.
fn create_event(_args: &[String]) -> WaygateResult {
    let handle = state().alloc_handle();
    if let Ok(mut map) = state().event_state.lock() {
        map.insert(handle, false);
    }
    Ok(format!("[waygate::kernel32] event_handle={handle}"))
}

fn set_event(args: &[String], value: bool) -> WaygateResult {
    let handle = parse_u64(arg_value(args, "handle"), 0);
    if let Ok(mut map) = state().event_state.lock() {
        map.insert(handle, value);
    }
    Ok(format!(
        "[waygate::kernel32] event_handle={handle} signaled={value}"
    ))
}

fn wait_for_single_object(args: &[String]) -> WaygateResult {
    let handle = parse_u64(arg_value(args, "handle"), 0);
    let timeout_ms = parse_u64(arg_value(args, "timeout"), 0);
    let signaled = state()
        .event_state
        .lock()
        .ok()
        .and_then(|map| map.get(&handle).copied())
        .unwrap_or(false);
    if !signaled && timeout_ms > 0 {
        thread::sleep(Duration::from_millis(timeout_ms.min(5)));
    }
    Ok(format!(
        "[waygate::kernel32] wait handle={handle} signaled={signaled} timeout_ms={timeout_ms}"
    ))
}

fn time_now(kind: &str) -> WaygateResult {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    Ok(format!("[waygate::kernel32] {kind}_time_unix_ms={now}"))
}

fn ok(name: &str, args: &[String]) -> WaygateResult {
    if args.is_empty() {
        Ok(format!("[waygate::kernel32] {name} called"))
    } else {
        Ok(format!(
            "[waygate::kernel32] {name} called with args: {}",
            args.join(", ")
        ))
    }
}
