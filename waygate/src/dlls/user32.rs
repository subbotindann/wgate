use std::sync::atomic::Ordering;

use crate::{arg_value, parse_bool, parse_i32, parse_u64, state, WaygateResult, WindowInfo};

pub const SUPPORTED: &[&str] = &[
    "MessageBoxA",
    "SendInput",
    "mouse_event",
    "keybd_event",
    "GetCursorPos",
    "SetCursorPos",
    "GetAsyncKeyState",
    "GetKeyState",
    "MapVirtualKey",
    "ShowCursor",
    "ClipCursor",
    "RegisterClassExW",
    "CreateWindowExW",
    "DefWindowProcW",
    "DestroyWindow",
    "ShowWindow",
    "UpdateWindow",
    "GetClientRect",
    "AdjustWindowRectEx",
    "SetWindowTextW",
    "GetMessageW",
    "PeekMessageW",
    "TranslateMessage",
    "DispatchMessageW",
    "PostMessageW",
    "SendMessageW",
    "FindWindowW",
    "FindWindowExW",
    "GetWindowRect",
    "ClientToScreen",
    "ScreenToClient",
    "GetForegroundWindow",
    "SetForegroundWindow",
    "GetDesktopWindow",
    "GetDC",
    "ReleaseDC",
    "GetSystemMetrics",
    "WM_CREATE",
    "WM_DESTROY",
    "WM_PAINT",
    "WM_SIZE",
    "WM_CLOSE",
    "WM_QUIT",
];

const WM_CREATE: u32 = 0x0001;
const WM_DESTROY: u32 = 0x0002;
const WM_SIZE: u32 = 0x0005;
const WM_PAINT: u32 = 0x000F;
const WM_CLOSE: u32 = 0x0010;
const WM_QUIT: u32 = 0x0012;

pub fn dispatch(symbol: &str, args: &[String]) -> Option<WaygateResult> {
    let res = match symbol {
        "CreateWindowExW" => create_window(args),
        "DestroyWindow" => destroy_window(args),
        "ShowWindow" => show_window(args),
        "SetWindowTextW" => set_window_text(args),
        "GetClientRect" | "GetWindowRect" => get_rect(symbol, args),
        "AdjustWindowRectEx" => adjust_window_rect(args),
        "FindWindowW" | "FindWindowExW" => find_window(args),
        "GetForegroundWindow" => Ok(format!(
            "[waygate::user32] hwnd={}",
            state().foreground.load(Ordering::Relaxed)
        )),
        "SetForegroundWindow" => set_foreground(args),
        "SetCursorPos" => set_cursor(args),
        "GetCursorPos" => get_cursor(),
        "ShowCursor" => show_cursor(args),
        "PostMessageW" => post_message(args),
        "PeekMessageW" | "GetMessageW" => get_message(symbol),
        "DispatchMessageW" | "TranslateMessage" => ok(symbol, args),
        "SendInput" | "mouse_event" | "keybd_event" => ok(symbol, args),
        "ClientToScreen" | "ScreenToClient" | "GetDC" | "ReleaseDC" | "GetSystemMetrics"
        | "MapVirtualKey" | "GetAsyncKeyState" | "GetKeyState" | "ClipCursor" | "MessageBoxA"
        | "RegisterClassExW" | "DefWindowProcW" | "UpdateWindow" | "SendMessageW"
        | "GetDesktopWindow" => ok(symbol, args),
        "WM_CREATE" => Ok(format!("[waygate::user32] WM_CREATE={WM_CREATE:#06x}")),
        "WM_DESTROY" => Ok(format!("[waygate::user32] WM_DESTROY={WM_DESTROY:#06x}")),
        "WM_PAINT" => Ok(format!("[waygate::user32] WM_PAINT={WM_PAINT:#06x}")),
        "WM_SIZE" => Ok(format!("[waygate::user32] WM_SIZE={WM_SIZE:#06x}")),
        "WM_CLOSE" => Ok(format!("[waygate::user32] WM_CLOSE={WM_CLOSE:#06x}")),
        "WM_QUIT" => Ok(format!("[waygate::user32] WM_QUIT={WM_QUIT:#06x}")),
        _ => return None,
    };
    Some(res)
}

// store a synthetic window so later calls can query/update it.
fn create_window(args: &[String]) -> WaygateResult {
    let title = arg_value(args, "title")
        .unwrap_or("L\"window\"")
        .to_string();
    let x = parse_i32(arg_value(args, "x"), 100);
    let y = parse_i32(arg_value(args, "y"), 100);
    let w = parse_i32(arg_value(args, "w"), 640);
    let h = parse_i32(arg_value(args, "h"), 480);
    let handle = state().alloc_handle();

    if let Ok(mut windows) = state().windows.lock() {
        windows.insert(
            handle,
            WindowInfo {
                title,
                visible: false,
                x,
                y,
                w,
                h,
            },
        );
    }

    Ok(format!(
        "[waygate::user32] created_window hwnd={handle} {x},{y} {w}x{h}"
    ))
}

fn destroy_window(args: &[String]) -> WaygateResult {
    let hwnd = parse_u64(arg_value(args, "hwnd"), 0);
    let removed = state()
        .windows
        .lock()
        .ok()
        .and_then(|mut w| w.remove(&hwnd))
        .is_some();
    Ok(format!(
        "[waygate::user32] destroy_window hwnd={hwnd} removed={removed}"
    ))
}

fn show_window(args: &[String]) -> WaygateResult {
    let hwnd = parse_u64(arg_value(args, "hwnd"), 0);
    let visible = parse_i32(arg_value(args, "cmd"), 1) != 0;
    if let Ok(mut windows) = state().windows.lock() {
        if let Some(win) = windows.get_mut(&hwnd) {
            win.visible = visible;
        }
    }
    Ok(format!(
        "[waygate::user32] show_window hwnd={hwnd} visible={visible}"
    ))
}

fn set_window_text(args: &[String]) -> WaygateResult {
    let hwnd = parse_u64(arg_value(args, "hwnd"), 0);
    let text = arg_value(args, "text").unwrap_or("L\"\"").to_string();
    if let Ok(mut windows) = state().windows.lock() {
        if let Some(win) = windows.get_mut(&hwnd) {
            win.title = text.clone();
        }
    }
    Ok(format!(
        "[waygate::user32] set_window_text hwnd={hwnd} text={text}"
    ))
}

fn get_rect(name: &str, args: &[String]) -> WaygateResult {
    let hwnd = parse_u64(arg_value(args, "hwnd"), 0);
    let (x, y, w, h) = state()
        .windows
        .lock()
        .ok()
        .and_then(|windows| windows.get(&hwnd).cloned())
        .map(|win| (win.x, win.y, win.w, win.h))
        .unwrap_or((0, 0, 640, 480));
    Ok(format!(
        "[waygate::user32] {name} hwnd={hwnd} rect={x},{y},{w},{h}"
    ))
}

fn adjust_window_rect(args: &[String]) -> WaygateResult {
    let has_menu = parse_bool(arg_value(args, "menu"), false);
    Ok(format!(
        "[waygate::user32] AdjustWindowRectEx border=8 caption=30 menu={has_menu}"
    ))
}

fn find_window(args: &[String]) -> WaygateResult {
    let wanted = arg_value(args, "title")
        .unwrap_or("0")
        .replace('"', "")
        .replace('L', "");
    if let Ok(windows) = state().windows.lock() {
        if let Some((hwnd, _)) = windows
            .iter()
            .find(|(_, w)| wanted == "0" || w.title.contains(&wanted))
        {
            return Ok(format!("[waygate::user32] found_window hwnd={hwnd}"));
        }
    }
    Ok("[waygate::user32] found_window hwnd=0".to_string())
}

fn set_foreground(args: &[String]) -> WaygateResult {
    let hwnd = parse_u64(arg_value(args, "hwnd"), 0);
    state().foreground.store(hwnd, Ordering::Relaxed);
    Ok(format!("[waygate::user32] foreground_window={hwnd}"))
}

fn set_cursor(args: &[String]) -> WaygateResult {
    let x = parse_i32(arg_value(args, "x"), 0);
    let y = parse_i32(arg_value(args, "y"), 0);
    if let Ok(mut p) = state().cursor.lock() {
        *p = (x, y);
    }
    Ok(format!("[waygate::user32] cursor={x},{y}"))
}

fn get_cursor() -> WaygateResult {
    let (x, y) = state().cursor.lock().map(|p| *p).unwrap_or((0, 0));
    Ok(format!("[waygate::user32] cursor={x},{y}"))
}

fn show_cursor(args: &[String]) -> WaygateResult {
    let show = parse_bool(arg_value(args, "show"), true);
    let delta = if show { 1 } else { -1 };
    let count = state()
        .show_cursor_count
        .fetch_add(delta, Ordering::Relaxed)
        + delta;
    Ok(format!("[waygate::user32] show_cursor_count={count}"))
}

// queue messages locally to emulate the windows message pump.
fn post_message(args: &[String]) -> WaygateResult {
    let hwnd = parse_u64(arg_value(args, "hwnd"), 0);
    let msg = parse_u64(arg_value(args, "msg"), WM_CLOSE as u64) as u32;
    let w = parse_u64(arg_value(args, "wparam"), 0) as i64;
    let l = parse_u64(arg_value(args, "lparam"), 0) as i64;
    if let Ok(mut q) = state().messages.lock() {
        q.push_back((hwnd, msg, w, l));
    }
    Ok(format!(
        "[waygate::user32] posted_message hwnd={hwnd} msg={msg:#06x}"
    ))
}

fn get_message(name: &str) -> WaygateResult {
    let ev = state().messages.lock().ok().and_then(|mut q| q.pop_front());
    match ev {
        Some((hwnd, msg, w, l)) => Ok(format!(
            "[waygate::user32] {name} hwnd={hwnd} msg={msg:#06x} w={w} l={l}"
        )),
        None => Ok(format!("[waygate::user32] {name} empty_queue")),
    }
}

fn ok(name: &str, args: &[String]) -> WaygateResult {
    if args.is_empty() {
        Ok(format!("[waygate::user32] {name} called"))
    } else {
        Ok(format!(
            "[waygate::user32] {name} called with args: {}",
            args.join(", ")
        ))
    }
}
