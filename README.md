# Extensions for the windows-rs and windows-sys crates

The code here is _not_ (yet?) meant as a publishable crate. It exists to provide basic implementations of macros and header-only functions from the Windows SDK. Initially proposed as a collection of snippets to be added to [Kenny Kerr's blog](https://github.com/kennykerr/blog) in a [formalized feature request](https://github.com/microsoft/windows-rs/issues/2798) where it was initially decided to add snippets to the blog.

When it became clear that perhaps blog snippets weren't the best option [due to no testing and high review burdens](https://github.com/kennykerr/blog/pull/3), the choice was made to move out the snippets to an automatically testable crate [in the last comment](https://github.com/kennykerr/blog/pull/3#issuecomment-1900045764). This is the repo for that.

I'd like to extend my thanks to @kennykerr and @tim-weis for their patience and flexibility in handling this situation. I hope they'll hold a continued involvement in keeping this crate accurate as we move forward. I'd also like to motivate additional community contributors. If anyone has any ideas for this, feel free to open an issue or PR. Things like writing contributing guidelines aren't my wheelhouse, so feel free to bring that in too.

# General structure

The code here is organized into a single large workspace. It is my intent to have the `windows-rs-ext` crate be a central place to import all the other ones into, maybe behind feature gates to provide granularity for testing. It'd be a shame to run _all_ tests if only a single crate in the workspace fails. And if we do decide to publish this at some point, it'd be nice to take the same approach as the crates we extend: using features to opt in to extensions with only very common functions being default. ~~and let's be real, _someone_ is going to add a git dependency for this at some point.~~

## Using `unsafe`

Any and all unsafe should be avoided as much as possible; though a lot of the `windows` crates require it in their API so it won't be considered grounds for rejection if there is proper argumentation for it. A big part of that is explaining why the use of unsafe is still sound code in either comments or, even better, docs (also comments but alas). If not all calls in a function are actually `unsafe`, there are grounds to consider using `unsafe {}` blocks rather than marking the entire function as unsafe. All functions using unsafe **must have a unit test** to ensure it works as intended.

## Functions or macros?

The C++ header counterparts of most of what happens here are macros. Except when pre-processed the emitted result is usually simple ops. This means that for the ones encountered so far, an inlined function would provide the exact same result while keeping readability. If something is more suited for implementation as a Rust macro, feel free to provide an implementation and give a brief explanation why.

# FAQ

Q: I really want to use these crates in my code, can I?
A: Git dependencies are a feature cargo provides, so you _can_. The real question is if you _should_.

Q: Okay, so should I?
A: No. At least in its current state, this workspace provides nothing of interest except some snippets. Maybe in the future though, who knows how much traction this gets?

Q: I'm missing macro X from header Y!
A: Questions ususally have question marks. But I understand the problem and frustration, so please open an issue or roll your own and hit us with a PR!

Q: I want to add stuff, but where can I find the C++ implementations to base myself on?
A: In the Windows SDK. Look around on MSDN and the web on where to get that. It should come with any Visual Studio install that includes Windows 10/11 SDK workloads.

Q: I'm missing all of a family of functions, where are they? ([this was actually opened](https://github.com/microsoft/windows-rs/issues/1572))
A: Now you can add them yourself! Or open an issue to ask for them.

Q: I requested something but it was closed as wontfix, why is that?
A: Some macros are actually only relevant to the C++ stuff and this repo only exists for actual added value. Sorry!
