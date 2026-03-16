// Copyright (C) 2020-2026 Loongson Technology Corporation Limited
// SPDX-License-Identifier: Apache-2.0

#[allow(clippy::all)]
#[allow(clippy::undocumented_unsafe_blocks)]
pub mod bindings;
#[cfg(feature = "fam-wrappers")]
pub mod fam_wrappers;

#[cfg(feature = "serde")]
mod serialize;

pub use self::bindings::*;
#[cfg(feature = "fam-wrappers")]
pub use self::fam_wrappers::*;
