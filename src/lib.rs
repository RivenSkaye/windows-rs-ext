//! # Extensions for `windows-rs` and `windows-sys`
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
//! implementing more macros and header-only functions that aren't available in the windows-rs and windows-sys bindings!
//!
//! ## `windows-rs` or `windows-sys`?
//! That's right, it's a mutually exclusive choice here. For now at least. A fair few types defined in the crates are
//! effectively the same thing, but they conflict due to being defined separately. This means Rust will treat these
//! objects as different types. As such, a choice was made to disallow having both optional dependencies. Perhaps this
//! will change in the future if there's some easily applicable and readable way to implement on both copies.
//! Until then, you'll have to choose between:
//! - The default feature (windows-rs)
//! - no default features and windows-sys

#[cfg(any(all(feature = "winrs", feature = "winsys"), all(not(feature = "winrs"), not(feature = "winsys"))))]
compile_error!(
    "It's impossible to unify some of the code in this crate for windows-sys and windows-rs! \
        The default feature includes windows-rs, if you wish to use windows-sys, \
        please disable the default feature as well."
);

#[cfg(not(feature = "no-minwindef"))]
pub mod minwindef;
#[cfg(feature = "no-minwindef")]
pub(crate) mod minwindef;
