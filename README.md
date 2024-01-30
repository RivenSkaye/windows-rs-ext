# Extensions for the `windows-rs` and `windows-sys` crates

The code here is _not_ (yet?) meant as a publishable crate. It exists to provide basic implementations of macros and header-only functions from the Windows SDK. Initially proposed as a collection of snippets to be added to [Kenny Kerr's blog](https://github.com/kennykerr/blog) in a [formalized feature request](https://github.com/microsoft/windows-rs/issues/2798) where it was initially decided to add snippets to the blog.

When it became clear that perhaps blog snippets weren't the best option [due to no testing and high review burdens](https://github.com/kennykerr/blog/pull/3), the choice was made to move out the snippets to an automatically testable crate [in the last comment](https://github.com/kennykerr/blog/pull/3#issuecomment-1900045764). This is the repo for that.

I'd like to extend my thanks to @kennykerr and @tim-weis for their patience and flexibility in handling this situation. I hope they'll hold a continued involvement in keeping this crate accurate as we move forward. I'd also like to motivate additional community contributors. If anyone has any ideas for this, feel free to open an issue or PR. Things like writing contributing guidelines aren't my wheelhouse, so feel free to bring that in too.

# General structure

The code here is all part of one big crate, but like the `windows` and `windows-sys` crates all components are locked behind features. This will, in time, allow for testing to be as granular as possible and to prevent massive binary sizes from having a lot of unused features. This should also help organize things into logical bits and pieces that mirror the existing headers as closely as possible. Taking the approach of the crates we extend also allows for defining features that will make all extensions available that are relevant to a specific workload in them. The end goal is, after all, making life easier for people that want to build against the Windows SDK.

In the future, other crates might be created to further split things out. For example WinRT for COM and stuff for the graphics stack are defined mostly separate from the core OS interfaces exposed in the windows crates. These might at some point split off if the feature list grows too large or if the amount of utility functions warrants it being used independently. Or maybe even to just debloat initial download sizes for use cases that don't make use of those components.

## Using `unsafe`

Any and all unsafe should be avoided as much as possible; though a lot of the `windows` crates' functions and components require it in their API so it won't be considered grounds for rejection if there is proper reasoning for it. A big part of that is explaining why the use of unsafe is still sound code in either comments or, even better, docs (also comments but alas). If not all calls in a function are actually `unsafe`, there are grounds to consider using `unsafe {}` blocks rather than marking the entire function as unsafe. All functions using unsafe **must have a unit test** to ensure it works as intended.

## Functions or macros?

The C++ header counterparts of most of what happens here are macros. Except when pre-processed the emitted result is usually relatively simple ops. This means that for the ones implemented so far, an inlined function would provide the exact same result while keeping readability. If something is more suited for implementation as a Rust macro, feel free to provide an implementation and give a brief explanation why. I'm comfortable reading and using macros, but for writing them I'd like to find someone more experienced to help implement them.

# FAQ

- Q: I really want to use this crate in my code, can I?  
  A: Git dependencies are a feature cargo provides, so you _can_. The real question is if you _should_.

- Q: Okay, so should I?  
  A: No. At least in its current state, this crate provides almost nothing of interest except some snippets. Maybe in the future though, who knows how much traction this gets?

- Q: I'm missing macro X from header Y!  
  A: Questions ususally have question marks. But I understand the problem and frustration, so please open an issue or roll your own and hit us with a PR!

- Q: I want to add stuff, but where can I find the C++ implementations to base myself on?  
  A: In the Windows SDK. Look around on MSDN and the web on where to get that. It should come with any Visual Studio install that includes Windows 10/11 SDK workloads.

- Q: I'm missing all of a family of functions, where are they? ([this was actually opened for the full `MI_*` suite](https://github.com/microsoft/windows-rs/issues/1572))  
  A: Yyou can add them yourself! Or open an issue to ask for them and hope it's picked up in a timely manner. This project is a volunteer effort and time is sparse.

- Q: I requested something but it was closed as wontfix, why is that?  
  A: Some macros are actually only relevant to the C++ stuff and this repo only exists for actual added value. If you believe the request to add value for Rust as well, feel free to keep the discussion on the issue thread going, or PR it in to show where the added value lies.
