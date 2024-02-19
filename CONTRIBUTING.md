# Contributing to `windows-rs`

First off, thanks for taking an interest in helping this crate expand! Contributions of any kind are very much welcome and appreciated, whether they're administrative or functional.

If you came here looking for one of the macros or functions in the SDK and couldn't find it, feel free to open an issue. If you know the inner workings, or if you can translate it to Rust, you're also welcome to just open a PR or both an issue and a closing PR. Just make sure to point to [a reference for the function or macro](https://learn.microsoft.com/en-us/windows/win32) or to point to a rough location where its Win32 SDK definition lives.
If instead you ended up here looking to help with no idea what to do, take a look at [the open issues](https://github.com/RivenSkaye/windows-rs-ext/issues/) and pick whatever you feel is feasible.

If you're only here to open an issue, please try to link the matching documentation for the SDK implementation or point to someplace in the files it can be found. If you have absolutely no idea, a simple "I'm missing function/macro XYZ used for this magical task or workload" suffices. For anything else, try to see if an issue template exists. Otherwise feel free to open a custom issue.

Regardless of the type of contribution, if you want to get in touch you can do so through here, on Discord (@rivenskaye) or 

## Using `unsafe`

Any and all unsafe should be avoided as much as possible; though a lot of the `windows` crates' functions and components require it in their API so it won't be considered grounds for rejection if there is proper reasoning for it. A big part of that is explaining why the use of unsafe is still sound code in either comments or, even better, docs (also comments but alas). If not all calls in a function are actually `unsafe`, there are grounds to consider using `unsafe {}` blocks rather than marking the entire function as unsafe. All functions using unsafe **must have a unit test** to ensure it works as intended.

When adding unsafe functions, please try to avoid SEH pitfalls. This crate does not (yet) have a mechanism to deal with this and the standard library only provides bare crash and unwind semantics. We're hoping recent developments around expired patents will allow for easier x86 SEH recovery in compilers other than MSVC. For now, [microSEH provides MSVC-only handling](https://github.com/sonodima/microseh) that uses a very minimal C wrapper. It seems [at least LLVM offers support here](https://llvm.org/docs/ExceptionHandling.html#exception-handling-using-the-windows-runtime) and apparently GCC is also capable of emitting stuff for it these days. So who knows, maybe someone will PR in a contribution for microSEH that enables LLVM and GCC builds as well!

## Functions or macros?

The C++ header counterparts of most of what happens here are macros. Except when pre-processed the emitted result is usually relatively simple ops or compile time calculated constants. This means that for the ones implemented so far, an inlined [`const fn`](https://doc.rust-lang.org/reference/const_eval.html#const-functions) would provide the exact same result while keeping readability. If something is more suited for implementation as a Rust macro, feel free to provide an implementation and give a brief explanation why. I'm comfortable reading and using macros, but for writing them I'd like to find someone more experienced to help implement them. It's simply a matter of not having enough experience to be certain of what I'm doing. If we find something that warrants a macro, maybe *you* are the one we want!
