// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com>

//! Logging utility with timestamp and CPU core ID, using a ring buffer.

use core::sync::atomic::{AtomicUsize, Ordering};
use core::cell::UnsafeCell;
use crate::arch::traits::TraitArch;

/// Maximum number of log records in the ring buffer
pub const LOG_BUFFER_SIZE: usize = 128;

/// Log record structure
#[derive(Copy, Clone, Debug)]
pub struct LogRecord {
    /// Timestamp value (unit is user-defined)
    pub timestamp: usize,
    /// CPU core ID
    pub core_id: usize,
    /// User-supplied identifier (e.g., function pointer, hash, etc.)
    pub ident: u64,
}

/// Type alias for timestamp getter function
pub type TimestampFn = fn() -> usize;

/// Ring buffer for log records
pub struct LogRingBuffer {
    buffer: [UnsafeCell<LogRecord>; LOG_BUFFER_SIZE],
    head: AtomicUsize, // 次に書き込むインデックス
    tail: AtomicUsize, // 最も古いデータのインデックス
    get_timestamp: AtomicTimestampFn,
}

/// Wrapper for atomic timestamp function pointer
pub struct AtomicTimestampFn {
    ptr: UnsafeCell<TimestampFn>,
}

unsafe impl Sync for LogRingBuffer {}

impl AtomicTimestampFn {
    pub const fn new(f: TimestampFn) -> Self {
        Self { ptr: UnsafeCell::new(f) }
    }
    pub fn set(&self, f: TimestampFn) {
        unsafe { *self.ptr.get() = f; }
    }
    pub fn get(&self) -> TimestampFn {
        unsafe { *self.ptr.get() }
    }
}

impl LogRingBuffer {
    pub const fn new(get_timestamp: TimestampFn) -> Self {
        const EMPTY: UnsafeCell<LogRecord> = UnsafeCell::new(LogRecord { timestamp: 0, core_id: 0, ident: 0 });
        Self {
            buffer: [EMPTY; LOG_BUFFER_SIZE],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            get_timestamp: AtomicTimestampFn::new(get_timestamp),
        }
    }

    /// Register a new timestamp getter function
    pub fn set_timestamp_fn(&self, f: TimestampFn) {
        self.get_timestamp.set(f);
    }

    /// Record a log entry with current timestamp and user identifier (core_id is fetched internally)
    pub fn log_record(&self, ident: u64) {
        let ts = (self.get_timestamp.get())();
        let core_id = crate::environment::Arch::get_cpuid();
        let head = self.head.load(Ordering::Relaxed);
        let next_head = (head + 1) % LOG_BUFFER_SIZE;
        unsafe {
            *self.buffer[head].get() = LogRecord { timestamp: ts, core_id, ident };
        }
        self.head.store(next_head, Ordering::Release);
        // Overwrite oldest if full
        if next_head == self.tail.load(Ordering::Acquire) {
            self.tail.store((self.tail.load(Ordering::Acquire) + 1) % LOG_BUFFER_SIZE, Ordering::Release);
        }
    }

    /// Get a log record by index (0 = oldest, up to len-1 = newest)
    pub fn get_record(&self, idx: usize) -> Option<LogRecord> {
        let tail = self.tail.load(Ordering::Acquire);
        let head = self.head.load(Ordering::Acquire);
        let len = if head >= tail {
            head - tail
        } else {
            LOG_BUFFER_SIZE - tail + head
        };
        if idx >= len {
            return None;
        }
        let pos = (tail + idx) % LOG_BUFFER_SIZE;
        Some(unsafe { *self.buffer[pos].get() })
    }

    /// Get the number of valid records in the buffer
    pub fn len(&self) -> usize {
        let tail = self.tail.load(Ordering::Acquire);
        let head = self.head.load(Ordering::Acquire);
        if head >= tail {
            head - tail
        } else {
            LOG_BUFFER_SIZE - tail + head
        }
    }
}

/// Global logger instance (example, can be placed in .bss for debugger visibility)
#[no_mangle]
pub static LOGGER: LogRingBuffer = LogRingBuffer::new(default_timestamp_fn);

/// Default timestamp getter (can be replaced by user)
pub fn default_timestamp_fn() -> usize {
    0 // Replace with actual timer/counter read
}

/// Macro for logging (captures timestamp and user identifier; core_id is fetched internally)
#[macro_export]
macro_rules! log_record {
    () => {
        $crate::utils::logging::LOGGER.log_record(0u64);
    };
    ($ident:expr) => {
        $crate::utils::logging::LOGGER.log_record($ident);
    };
}

/// Macro for getting a log record by index
#[macro_export]
macro_rules! get_record {
    ($idx:expr) => {
        $crate::utils::logging::LOGGER.get_record($idx)
    };
}

#[test_case]
pub fn test_log_record_and_retrieve() -> Result<(), &'static str> {
    fn fake_timestamp() -> usize { 42 }
    let logger = LogRingBuffer::new(fake_timestamp);
    logger.log_record(1234u64);
    if logger.len() != 1 { return Err("len != 1"); }
    let rec = logger.get_record(0).ok_or("no record")?;
    if rec.timestamp != 42 { return Err("timestamp != 42"); }
    if rec.ident != 1234u64 { return Err("ident != 1234"); }
    Ok(())
}

#[test_case]
pub fn test_ring_buffer_overwrite() -> Result<(), &'static str> {
    fn fake_timestamp() -> usize { 1 }
    let logger = LogRingBuffer::new(fake_timestamp);
    for i in 0..LOG_BUFFER_SIZE-1 {
        logger.log_record(i as u64);
    }
    if logger.len() != LOG_BUFFER_SIZE-1 { return Err("buffer not full"); }
    logger.log_record(0xdeadbeefu64);
    if logger.len() != LOG_BUFFER_SIZE-1 { return Err("buffer size changed"); }
    let first = logger.get_record(0).ok_or("no first record")?;
    let last = logger.get_record(LOG_BUFFER_SIZE-2).ok_or("no last record")?;
    if last.ident != 0xdeadbeefu64 { return Err("last ident not correct"); }
    Ok(())
}

#[test_case]
pub fn test_set_timestamp_fn() -> Result<(), &'static str> {
    let logger = LogRingBuffer::new(|| 1);
    logger.log_record(0x11u64);
    if logger.get_record(0).ok_or("no record")?.timestamp != 1 {
        return Err("timestamp != 1");
    }
    if logger.get_record(0).ok_or("no record")?.ident != 0x11u64 {
        return Err("ident != 0x11");
    }
    logger.set_timestamp_fn(|| 99);
    logger.log_record(0x22u64);
    if logger.get_record(1).ok_or("no record")?.timestamp != 99 {
        return Err("timestamp != 99");
    }
    if logger.get_record(1).ok_or("no record")?.ident != 0x22u64 {
        return Err("ident != 0x22");
    }
    Ok(())
}

#[test_case]
pub fn test_get_record_out_of_bounds() -> Result<(), &'static str> {
    fn fake_timestamp() -> usize { 0 }
    let logger = LogRingBuffer::new(fake_timestamp);
    if logger.get_record(0).is_some() {
        return Err("should be none");
    }
    logger.log_record(0x55u64);
    if logger.get_record(1).is_some() {
        return Err("should be none");
    }
    Ok(())
}
