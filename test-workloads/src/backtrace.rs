// SPDX-FileCopyrightText: 2023 Rivos Inc.
//
// SPDX-License-Identifier: Apache-2.0

/// Get an array of backtrace addresses.
///
/// This needs `force-frame-pointers` enabled.
use alloc::fmt::{Display, Formatter, Result};
use core::arch::asm;
use core::mem::size_of;

extern crate alloc;

extern "C" {
    static _stack_start: u8;
    static _stack_end: u8;
}

#[derive(Copy, Clone)]
pub enum BTReturnAddress {
    ReturnAddress(u64),
    InvalidFramePointer(u64),
    InvalidReturnAddress(u64),
}

impl Display for BTReturnAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            BTReturnAddress::ReturnAddress(addr) => {
                writeln!(f, "0x{addr:x}")
            }
            BTReturnAddress::InvalidFramePointer(addr) => {
                writeln!(f, "Invalid frame pointer: 0x{addr:x}")
            }
            BTReturnAddress::InvalidReturnAddress(addr) => {
                writeln!(f, "Null address at frame pointer: 0x{addr:x}")
            }
        }
    }
}

pub struct BackTrace {
    fp: Option<u64>,
    stack_start: u64,
    stack_end: u64,
    stack: &'static [u64],
}

impl BackTrace {
    fn new(fp: u64) -> Self {
        let stack_start = unsafe { core::ptr::addr_of!(_stack_start) as u64 };
        let stack_end = unsafe { core::ptr::addr_of!(_stack_end) as u64 };
        let stack = unsafe {
            core::slice::from_raw_parts(
                stack_start as *const u64,
                (stack_end - stack_start) as usize,
            )
        };

        Self {
            fp: Some(fp),
            stack_start,
            stack_end,
            stack,
        }
    }

    fn next_address(&self, fp: u64) -> Option<(u64, u64)> {
        // if offset is out of bounds get will return none, and the whole function
        // will return none
        let offset = ((fp - self.stack_start) / size_of::<u64>() as u64) as usize;
        let address = *self.stack.get(offset - 1)?;
        let current_fp = *self.stack.get(offset - 2)?;
        Some((address, current_fp))
    }
}

impl Iterator for BackTrace {
    type Item = BTReturnAddress;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut fp) = self.fp {
            if fp < self.stack_start || fp > self.stack_end {
                self.fp = None;
                return Some(BTReturnAddress::InvalidFramePointer(fp));
            }
            if let Some((address, current_fp)) = self.next_address(fp) {
                fp = current_fp;
                self.fp = Some(fp);
                if address == 0 {
                    self.fp = None;
                    return Some(BTReturnAddress::InvalidReturnAddress(fp));
                }
                return Some(BTReturnAddress::ReturnAddress(address));
            }
            self.fp = None;
            Some(BTReturnAddress::InvalidReturnAddress(fp))
        } else {
            None
        }
    }
}

pub(crate) fn backtrace() -> BackTrace {
    // Safe because we are just reading a register
    let fp = unsafe {
        let mut tmp: u64;
        asm!("mv {0}, fp", out(reg) tmp);
        tmp
    };

    BackTrace::new(fp)
}
