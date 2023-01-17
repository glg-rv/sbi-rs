// Copyright (c) 2023 by Rivos Inc.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use crate::error::*;
use crate::SbiReturn;

#[cfg(all(target_arch = "riscv64", target_os = "none"))]
use core::arch::asm;

/// Trait required for a type to be passed via a SBI ecall.
pub trait EcallMessage {
    /// Returns the register value of a7.
    fn a7(&self) -> u64;

    /// Returns the register value of a6.
    fn a6(&self) -> u64;

    /// Returns the register value of a5.
    fn a5(&self) -> u64;

    /// Returns the register value of a4.
    fn a4(&self) -> u64;

    /// Returns the register value of a3.
    fn a3(&self) -> u64;

    /// Returns the register value of a2.
    fn a2(&self) -> u64;

    /// Returns the register value of a1.
    fn a1(&self) -> u64;

    /// Returns the register value of a0.
    fn a0(&self) -> u64;

    /// Returns the result returned from the ecall. Intended for use after an SBI message has been
    /// handled by the firmware. Interprets the given registers based on the extension and function
    /// and returns the approprate result.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[cfg(all(target_arch = "riscv64", target_os = "none"))]
    /// pub fn ecall_send<M: EcallMessage>(msg: &M) -> Result<u64> {
    ///     let mut a0 = msg.a0(); // error code
    ///     let mut a1 = msg.a1(); // return value
    ///     unsafe {
    ///         // Safe, but relies on trusting the hypervisor or firmware.
    ///         asm!("ecall", inout("a0") a0, inout("a1")a1,
    ///                 in("a2")msg.a2(), in("a3") msg.a3(),
    ///                 in("a4")msg.a4(), in("a5") msg.a5(),
    ///                 in("a6")msg.a6(), in("a7") msg.a7());
    ///     }
    ///
    ///     msg.result(a0, a1)
    /// }
    /// ```
    fn result(&self, a0: u64, a1: u64) -> Result<u64> {
        // Default implementation.
        SbiReturn {
            error_code: a0 as i64,
            return_value: a1,
        }
        .into()
    }
}

/// Send an ecall to the firmware or hypervisor.
///
/// # Safety
///
/// The caller must verify that any memory references contained in `msg` obey Rust's memory
/// safety rules. For example, any pointers to memory that will be modified in the handling of
/// the ecall must be uniquely owned. Similarly any pointers read by the ecall must not be
/// mutably borrowed.
///
/// In addition the caller is placing trust in the firmware or hypervisor to maintain the promises
/// of the interface w.r.t. reading and writing only within the provided bounds.
#[cfg(all(target_arch = "riscv64", target_os = "none"))]
pub unsafe fn ecall_send<M: EcallMessage>(msg: &M) -> Result<u64> {
    // normally error code
    let mut a0;
    // normally return value
    let mut a1;
    asm!("ecall", inlateout("a0") msg.a0()=>a0, inlateout("a1")msg.a1()=>a1,
                in("a2")msg.a2(), in("a3") msg.a3(),
                in("a4")msg.a4(), in("a5") msg.a5(),
                in("a6")msg.a6(), in("a7") msg.a7(), options(nostack));

    msg.result(a0, a1)
}

#[cfg(not(all(target_arch = "riscv64", target_os = "none")))]
unsafe fn ecall_send<M: EcallMessage>(_msg: &M) -> Result<u64> {
    panic!("ecall_send called");
}
