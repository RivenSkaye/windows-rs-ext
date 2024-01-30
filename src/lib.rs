//! # Extensions for `windows-rs` and `windows-sys`
//! The extensions defined in this module aim to help make the windows crates more
//! ergonomic to use, providing the well-known macros from the SDK headers to us
//! mere mortals who prefer memory safety and fearlessness.
//!
//! By default, the minwindef functions are included as these are frequently imported
//! either directly or indirectly (a lot of the other headers import it too) and
//! a quick search for the name reveals it's at least _referenced_ in 58 other headers.
//! An additional feature is available that makes the functions defined there also
//! available through implemented traits

#[cfg(feature = "minwindef")]
pub use minwindef;
