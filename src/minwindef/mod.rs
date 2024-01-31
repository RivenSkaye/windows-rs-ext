//! # `minwindef.h` macros for use in Rust with the windows-rs and windows-sys crates
//! This module defines equivalents to the macros defined in Include/<version>/shared/minwindef.h
//! as part of the Windows SDK. These macros serve to (un)pack numeric values into larger numeric
//! types.
//!
//! Besides the existing `#define`s, this module adds a couple functions useful for those working
//! with 64-bit integers and pointers:
//! - [`lodword`] and [`hidword`] to get the lower and higher 32 bits from them
//! - [`makelonglong`] to pack two 32-bit ints into a 64-bit int.
//! These were added because they're needed for working with e.g. `CreateFileMapping*` functions.
//!
//! Lastly some additional methods were added for splitting and returning both values at once.
//! Consider them syntactical sugar over the others, as that really is all they are.
//! They just return a tuple containing (in order) the low order and high order components. This
//! is achieved by just calling both of those functions one after the other.
//! ```rs
//! // existing minwindef.h macros:
//! let full: u32 = 9548625;
//! let lo = loword(full);
//! let hi = hiword(full);
//!
//! // convenience wrapper:
//! let full: u32 = 9548625;
//! let (hi, lo) = splitdword(full)
//! ```

#[cfg(feature = "ext-impls")]
pub mod ext;

/// Get the low order word as u16
#[inline(always)]
pub const fn loword(dw: u32) -> u16 {
    dw as u16 // truncate the first 16 bits
}

/// Get the high order word as u16
#[inline(always)]
pub const fn hiword(dw: u32) -> u16 {
    (dw >> 16) as u16 // truncate the zero-padding
}

/// Split a single dword (`u32`) in both of its words (`u16`s)
#[inline(always)]
pub const fn splitdword(dw: u32) -> (u16, u16) {
    (loword(dw), hiword(dw))
}

/// Get the low order byte from a word (u16)
#[inline(always)]
pub const fn lobyte(word: u16) -> u8 {
    word as u8
}

/// Get the high order byte from a word (u16)
#[inline(always)]
pub const fn hibyte(word: u16) -> u8 {
    (word >> 8) as u8
}

/// Split a single word (`u16`) in both of its bytes (`u8`s)
#[inline(always)]
pub const fn splitword(word: u16) -> (u8, u8) {
    (lobyte(word), hibyte(word))
}

/// Get the low order double word from a u64
#[inline(always)]
pub const fn lodword(longlong: u64) -> u32 {
    longlong as u32
}

/// Get the high order double word from a u64
#[inline(always)]
pub const fn hidword(longlong: u64) -> u32 {
    (longlong >> 32) as u32
}

/// Split a single dword (`u32`) in both of its words (`u16`s)
#[inline(always)]
pub const fn splitqword(qw: u64) -> (u32, u32) {
    (lodword(qw), hidword(qw))
}

/// Turn two bytes into a word
#[inline(always)]
pub const fn makeword(low: u8, high: u8) -> u16 {
    (low as u16) | ((high as u16) << 8)
}

/// Turn two words into a dword/long
/// docs are somewhat weird, the macro is named "MAKELONG" but returns a DWORD?
pub const fn makelong(low: u16, high: u16) -> u32 {
    (low as u32) | ((high as u32) << 16)
}

/// Not part of the standard macros, but a logical step to accommodate 64-bit
/// Named after the underlying C datatype, in Win32 terms this would be a QWORD
pub const fn makelonglong(low: u32, high: u32) -> u64 {
    (low as u64) | ((high as u64) << 32)
}
