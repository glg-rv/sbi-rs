// Copyright (c) 2023 by Rivos Inc.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// Salus vendor exception.

use crate::ecall::EcallMessage;
use crate::error::*;
use crate::vendor::*;
use crate::SbiFunction;

const EXT_SALUS_TEST: u64 = 0x09FFFFFF;

/// A SBI message cotaining Salus Vendor Extensions.
pub type SalusSbiMessage = VendorSbiMessage<SalusExtension>;

/// Salus vendor extension messages.
pub enum SalusExtension {
    /// Salus test, use internally to test salus.
    SalusTest(SalusTestFunction),
}

impl VendorExtension for SalusExtension {
    fn from_regs(args: &[u64]) -> Result<Self> {
        use SalusExtension::*;
        match args[7] {
            EXT_SALUS_TEST => SalusTestFunction::from_regs(args).map(SalusTest),
            _ => Err(Error::NotSupported),
        }
    }
}

impl EcallMessage for SalusExtension {
    fn a7(&self) -> u64 {
        use SalusExtension::*;
        match self {
            SalusTest(_) => EXT_SALUS_TEST,
        }
    }

    fn a6(&self) -> u64 {
        use SalusExtension::*;
        match self {
            SalusTest(m) => m.a6(),
        }
    }

    fn a5(&self) -> u64 {
        use SalusExtension::*;
        match self {
            SalusTest(m) => m.a5(),
        }
    }

    fn a4(&self) -> u64 {
        use SalusExtension::*;
        match self {
            SalusTest(m) => m.a4(),
        }
    }

    fn a3(&self) -> u64 {
        use SalusExtension::*;
        match self {
            SalusTest(m) => m.a3(),
        }
    }

    fn a2(&self) -> u64 {
        use SalusExtension::*;
        match self {
            SalusTest(m) => m.a2(),
        }
    }

    fn a1(&self) -> u64 {
        use SalusExtension::*;
        match self {
            SalusTest(m) => m.a1(),
        }
    }

    fn a0(&self) -> u64 {
        use SalusExtension::*;
        match self {
            SalusTest(m) => m.a0(),
        }
    }
}

/// Functions defined for the Rivos test extension
#[derive(Clone, Copy, Debug)]
pub enum SalusTestFunction {
    /// Returns the implemented version of the SBI standard.
    MemCopy(MemCopyArgs),
}

impl SalusTestFunction {
    /// Attempts to parse `Self` from the passed in `a0-a7`.
    pub(crate) fn from_regs(args: &[u64]) -> Result<Self> {
        use SalusTestFunction::*;

        match args[6] {
            0 => Ok(MemCopy(MemCopyArgs {
                to: args[0],
                from: args[1],
                len: args[2],
            })),
            _ => Err(Error::NotSupported),
        }
    }
}

impl SbiFunction for SalusTestFunction {
    fn a0(&self) -> u64 {
        use SalusTestFunction::*;
        match self {
            MemCopy(args) => args.to,
        }
    }

    fn a1(&self) -> u64 {
        use SalusTestFunction::*;
        match self {
            MemCopy(args) => args.from,
        }
    }

    fn a2(&self) -> u64 {
        use SalusTestFunction::*;
        match self {
            MemCopy(args) => args.len,
        }
    }

    fn a6(&self) -> u64 {
        use SalusTestFunction::*;
        match self {
            MemCopy(_) => 0,
        }
    }
}

/// Arguments to the memcpy test function
#[derive(Clone, Copy, Debug)]
pub struct MemCopyArgs {
    /// Destination Address.
    pub to: u64,
    /// Source Address.
    pub from: u64,
    /// Length in bytes of the copy.
    pub len: u64,
}
