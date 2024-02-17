#![deny(missing_docs, rustdoc::broken_intra_doc_links, clippy::cargo)]

//! # Extensions for `windows-rs`
//! The extensions defined in this module aim to help make the windows crates more ergonomic to use, providing the
//! well-known macros from the SDK headers to us mere mortals who prefer memory safety and fearlessness.
//!
//! By default, the minwindef functions are included as these are frequently imported either directly or indirectly
//! (a lot of the other headers import it too) and a quick search for the name reveals it's at least _referenced_ in
//! 58 other headers. An additional feature will become available that makes the functions defined there also available
//! through implemented traits for some of the types defined in the windows crates.
//!
//! Exporting the minwindef functions, however useful they may be, can be turned off by using the `no-minwindef`
//! anti-feature. This only disables its export, not its use elsewhere in this crate. The functions are a bunch of
//! oneliners annotated for always inlinging. This was done both to keep in line with what C++ does with macros
//! (expanding the code inline at compile time) and because the function call would be inlined with any level of
//! optimization enabled (yes, even `-O1`). Sorry, but there is no plan to manually inline code being called in as many
//! places as this based on a feature flag when the output would be no different either way. The inline annotation was
//! added instead, as a way to not impact the resulting binary size; the function call is almost as big as the bitops.
//!
//! As it stands, this crate only implements a single header. Please do feel free to contribute to the project by
//! implementing more macros and header-only functions that aren't available in the windows-rs bindings!

#[cfg(feature = "minwindef")]
pub mod minwindef;
#[cfg(not(feature = "minwindef"))]
pub(crate) mod minwindef;

/// Wrapper for all extension traits this crate defines
///
/// All traits can be used with `use windows_ext::ext::*`.
#[cfg(feature = "ext-impls")]
pub mod ext {
    #[cfg(feature = "minwindef")]
    pub use crate::minwindef::ext::*;
}
