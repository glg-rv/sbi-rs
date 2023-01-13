// Copyright (c) 2023 by Rivos Inc.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use crate::consts::*;
use crate::error::*;
use crate::{EcallMessage, SbiMessage};

/// Trait to be implemented to specify a vendor extension. It is an extension to `EcallMessage` that
/// allows construction from a register set.
pub trait VendorExtension: EcallMessage + Sized {
    /// Reconstruct a `VendorExtension` from a register set.
    fn from_regs(args: &[u64]) -> Result<Self>;
}

/// A `SbiMessage` that supports vendor extension specified in V.
pub enum VendorSbiMessage<V: VendorExtension> {
    /// A standard SBI message.
    Sbi(SbiMessage),
    /// A message containing a vendor extension.
    Vendor(V),
}

impl<V: VendorExtension> VendorSbiMessage<V> {
    /// Same as `SbiMessage::from_regs` but supporting vendor extensions.
    pub fn from_regs(args: &[u64]) -> Result<Self> {
        use VendorSbiMessage::*;
        match args[7] {
            EXT_VENDOR_RANGE_START..=EXT_VENDOR_RANGE_END => V::from_regs(args).map(Vendor),
            _ => SbiMessage::from_regs(args).map(Sbi),
        }
    }
}

impl<V: VendorExtension> EcallMessage for VendorSbiMessage<V> {
    /// Returns the register value for this `SbiMessage`.
    fn a7(&self) -> u64 {
        use VendorSbiMessage::*;
        match self {
            Sbi(m) => m.a7(),
            Vendor(m) => m.a7(),
        }
    }

    /// Returns the register value for this `SbiMessage`.
    fn a6(&self) -> u64 {
        use VendorSbiMessage::*;
        match self {
            Sbi(m) => m.a6(),
            Vendor(m) => m.a6(),
        }
    }

    /// Returns the register value for this `SbiMessage`.
    fn a5(&self) -> u64 {
        use VendorSbiMessage::*;
        match self {
            Sbi(m) => m.a5(),
            Vendor(m) => m.a5(),
        }
    }

    /// Returns the register value for this `SbiMessage`.
    fn a4(&self) -> u64 {
        use VendorSbiMessage::*;
        match self {
            Sbi(m) => m.a4(),
            Vendor(m) => m.a4(),
        }
    }

    /// Returns the register value for this `SbiMessage`.
    fn a3(&self) -> u64 {
        use VendorSbiMessage::*;
        match self {
            Sbi(m) => m.a3(),
            Vendor(m) => m.a3(),
        }
    }

    /// Returns the register value for this `SbiMessage`.
    fn a2(&self) -> u64 {
        use VendorSbiMessage::*;
        match self {
            Sbi(m) => m.a2(),
            Vendor(m) => m.a2(),
        }
    }

    /// Returns the register value for this `SbiMessage`.
    fn a1(&self) -> u64 {
        use VendorSbiMessage::*;
        match self {
            Sbi(m) => m.a1(),
            Vendor(m) => m.a1(),
        }
    }

    /// Returns the register value for this `SbiMessage`.
    fn a0(&self) -> u64 {
        use VendorSbiMessage::*;
        match self {
            Sbi(m) => m.a0(),
            Vendor(m) => m.a0(),
        }
    }
}
