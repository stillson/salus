// SPDX-FileCopyrightText: 2023 Rivos Inc.
//
// SPDX-License-Identifier: Apache-2.0

//! This crate provides test workloads to exercise the salus hypervisor and the Confidential
//! computing API.

#![no_std]

use crate::backtrace::backtrace;
use s_mode_utils::abort::abort;
use s_mode_utils::print::*;

mod asm;
mod backtrace;
pub mod consts;

/// Panics the running test workload.
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("panic : {:?}", info);
    println!("panic backtrace:");
    // Skip the first 2 stack which are the panic code.
    backtrace().for_each(|frame| {
        print!("{}", frame);
    });
    abort();
}
