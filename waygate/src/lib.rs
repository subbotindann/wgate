mod dlls;

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicI32, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

pub type WaygateResult = Result<String, String>;

#[derive(Clone)]
pub(crate) struct WindowInfo {
    pub title: String,
    pub visible: bool,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub(crate) struct CompatState {
    pub started: Instant,
    pub last_error: AtomicU32,
    pub next_handle: AtomicU64,
    pub event_state: Mutex<HashMap<u64, bool>>,
    pub windows: Mutex<HashMap<u64, WindowInfo>>,
    pub foreground: AtomicU64,
    pub cursor: Mutex<(i32, i32)>,
    pub show_cursor_count: AtomicI32,
    pub messages: Mutex<VecDeque<(u64, u32, i64, i64)>>,
}

impl CompatState {
    fn new() -> Self {
        Self {
            started: Instant::now(),
            last_error: AtomicU32::new(0),
            next_handle: AtomicU64::new(1000),
            event_state: Mutex::new(HashMap::new()),
            windows: Mutex::new(HashMap::new()),
            foreground: AtomicU64::new(0),
            cursor: Mutex::new((0, 0)),
            show_cursor_count: AtomicI32::new(0),
            messages: Mutex::new(VecDeque::new()),
        }
    }

    pub(crate) fn alloc_handle(&self) -> u64 {
        self.next_handle.fetch_add(1, Ordering::Relaxed)
    }
}

static STATE: OnceLock<CompatState> = OnceLock::new();

pub(crate) fn state() -> &'static CompatState {
    STATE.get_or_init(CompatState::new)
}

pub(crate) fn arg_value<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter().find_map(|a| {
        let (k, v) = a.split_once('=')?;
        (k.trim() == key).then_some(v.trim())
    })
}

pub(crate) fn parse_i32(value: Option<&str>, default: i32) -> i32 {
    value
        .and_then(|v| v.trim_matches('"').parse::<i32>().ok())
        .unwrap_or(default)
}

pub(crate) fn parse_u64(value: Option<&str>, default: u64) -> u64 {
    value
        .and_then(|v| v.trim_matches('"').parse::<u64>().ok())
        .unwrap_or(default)
}

pub(crate) fn parse_bool(value: Option<&str>, default: bool) -> bool {
    match value.map(|v| v.trim_matches('"').to_ascii_lowercase()) {
        Some(v) if v == "true" || v == "1" => true,
        Some(v) if v == "false" || v == "0" => false,
        _ => default,
    }
}

pub fn supported_symbols() -> &'static [&'static str] {
    static SYMBOLS: OnceLock<Vec<&'static str>> = OnceLock::new();
    SYMBOLS
        .get_or_init(|| {
            let mut out =
                Vec::with_capacity(dlls::kernel32::SUPPORTED.len() + dlls::user32::SUPPORTED.len());
            out.extend_from_slice(dlls::kernel32::SUPPORTED);
            out.extend_from_slice(dlls::user32::SUPPORTED);
            out
        })
        .as_slice()
}

pub fn dispatch(symbol: &str, args: &[String]) -> WaygateResult {
    if let Some(result) = dlls::kernel32::dispatch(symbol, args) {
        return result;
    }
    if let Some(result) = dlls::user32::dispatch(symbol, args) {
        return result;
    }
    Err(format!("waygate: symbol '{symbol}' is not implemented"))
}
