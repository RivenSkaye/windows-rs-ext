//! This module provides extension traits that expose the freestanding functions
//! from [minwindef][crate::minwindef] directly on the respective types.
//!
//! These traits can also be used with [new-types](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).

/// This trait provides the freestanding functions from [minwindef][crate::minwindef]
/// directly on DWORDs ([u32]).
///
/// When using [new-types](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) that wrap a DWORD,
/// you can implement this trait by implementing the [value][DWordExt::value] method.
///
/// This trait is included in the convenience wrapper [`windows_ext::ext`][crate::ext].
///
/// ```
/// use windows_ext::ext::DWordExt; // or: windows_ext::minwindef::ext::DWordExt;
///
/// assert_eq!(0x1234_5678u32.hiword(), 0x1234);
/// assert_eq!(0x1234_5678u32.loword(), 0x5678);
/// ```
pub trait DWordExt: Copy {
    /// Gets the value of the DWORD.
    fn value(self) -> u32;

    /// Get the low order word as [u16]
    ///
    /// ```
    /// # use windows_ext::ext::DWordExt;
    /// assert_eq!(0x1234_5678u32.loword(), 0x5678);
    /// ```
    #[inline(always)]
    fn loword(self) -> u16 {
        super::loword(self.value())
    }

    /// Get the high order word as [u16]
    ///
    /// ```
    /// # use windows_ext::ext::DWordExt;
    /// assert_eq!(0x1234_5678u32.hiword(), 0x1234);
    /// ```
    #[inline(always)]
    fn hiword(self) -> u16 {
        super::hiword(self.value())
    }

    /// Split a single dword into both of its words (lo, hi)
    ///
    /// ```
    /// # use windows_ext::ext::DWordExt;
    /// assert_eq!(0x1234_5678u32.split(), (0x5678, 0x1234));
    /// ```
    #[inline(always)]
    fn split(self) -> (u16, u16) {
        super::splitdword(self.value())
    }
}

impl DWordExt for u32 {
    #[inline(always)]
    fn value(self) -> u32 {
        self
    }
}

impl DWordExt for usize {
    /// DWord value of a usize. On 64-bit this truncates to the lower 32.
    #[inline(always)]
    fn value(self) -> u32 {
        self as u32
    }
}

/// This trait provides the freestanding functions from [minwindef][crate::minwindef]
/// directly on QWORDs ([u64]).
///
/// When using [new-types](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) that wrap a QWORD,
/// you can implement this trait by implementing the [value][QWordExt::value] method.
///
/// This trait is included in the convenience wrapper [`windows_ext::ext`][crate::ext].
///
/// ```
/// use windows_ext::ext::QWordExt; // or: windows_ext::minwindef::ext::QWordExt;
///
/// assert_eq!(0x1234_5678_9abc_def0u64.hidword(), 0x1234_5678);
/// assert_eq!(0x1234_5678_9abc_def0u64.lodword(), 0x9abc_def0);
/// ```
pub trait QWordExt: Copy {
    /// Gets the value of the QWORD.
    fn value(self) -> u64;

    /// Get the low order double word as [u32]
    ///
    /// ```
    /// # use windows_ext::ext::QWordExt;
    /// assert_eq!(0x1234_5678_9abc_def0u64.lodword(), 0x9abc_def0);
    /// ```
    #[inline(always)]
    fn lodword(self) -> u32 {
        super::lodword(self.value())
    }

    /// Get the high order double word as [u32]
    ///
    /// ```
    /// # use windows_ext::ext::QWordExt;
    /// assert_eq!(0x1234_5678_9abc_def0u64.hidword(), 0x1234_5678);
    /// ```
    #[inline(always)]
    fn hidword(self) -> u32 {
        super::hidword(self.value())
    }

    /// Split a single quad word into both of its double words (lo, hi)
    ///
    /// ```
    /// # use windows_ext::ext::QWordExt;
    /// assert_eq!(0x1234_5678_9abc_def0u64.split(), (0x9abc_def0, 0x1234_5678));
    /// ```
    #[inline(always)]
    fn split(self) -> (u32, u32) {
        super::splitqword(self.value())
    }
}

impl QWordExt for u64 {
    fn value(self) -> u64 {
        self
    }
}

impl QWordExt for usize {
    /// QWord for usize, on 32-bit this has normal padding semantics.
    fn value(self) -> u64 {
        self as u64
    }
}
