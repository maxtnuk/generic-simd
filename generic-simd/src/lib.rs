#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
pub use arch_types;

pub use generic_simd_macros::dispatch;

pub mod shim;
pub mod vector;

#[macro_use]
mod implementation;

pub mod slice;

pub mod generic;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;