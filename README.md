# Nannou Tokio

> Nannou Tokio is about to explode...

This project is an experiment running Nannou in-process with Tokio. It's harder than it initially seemed because they each fight for runtime supremacy. These are notes taken analysing the problem and hopefully useful to understanding solutions.

The main goal is to incorporate Nannou in an existing application that uses Tokio to manage the main event loop.

```rust

/// the names of these methods are arbitrary, their signatures have interdependent constraits 
/// from derived how they get passed to the app() "fluent builder" API

/// SomeModel is mutable only within this callback
fn update(_app: &App, _model: &mut SomeModel, _update: Update) {}

/// SomeModel is immutable inside view
fn view(_app: &App, _model: &SomeModel, _frame: Frame) {}

/// return type of this method defines the model type
fn model(_app: &App) -> SomeModel {
    SomeModel {
      // ...
    }
}
```

The model in a nannou app can be used for dynamic rendering of something like Twitch Chat by calling methods on the passed-in `mut` reference to the user-defined struct that is the return type of the previously registered function passed to `app()`. Updates to the model are made in a registered callback with a signature like this `fn update(_app: &App, model: &mut SomeModel, _: Update) { ... }` where `SomeModel` is a user-defined struct returned from the function passed to `app(fn_returning_SomeModel)`. Note the update callback, unlike the `view(...)` callback, receives a `mut` model reference. 




## Running Nannou in its own process

Avoiding running Nannou in-process is probably easier than embedding and could still be orchestrated centrally using something like `std::process::Command`.

## Nannou

[docs](https://docs.rs/nannou/latest/nannou/) | [github](https://github.com/nannou-org/nannou) | [website](https://nannou.cc/)

* calls to `run()` to start nannou block forever
* nannou has `run_async()` and `build_async()` but they are undocumented
  * Google search returns **FOUR RESULTS** for ["nannou" "run_async"](https://www.google.com/search?q=%22nannou%22+%22run_async%22)
  * Github code search [nannou run_async](https://github.com/search?q=nannou+run_async&type=code) returns 29 files including nannou forks and code targeting wasm which needs it. `"nannou" "tokio"` is similarly sparse 
* nannou not initially designed for async but the [changelog](https://guide.nannou.cc/changelog) shows regular progress e.g. [#815](https://github.com/nannou-org/nannou/pull/815), [#826](https://github.com/nannou-org/nannou/issues/826) 
* Open issue [#831](https://github.com/nannou-org/nannou/issues/831) 
_Thoughts on async support and API implications_ says the update callback is notably not async and unlike `build_async()` or `run_async()`, fixing this "seems quite a bit tricker".

> If we want to allow users to create windows during update too, then we'd also need update to be async, which seems quite a bit trickier to achieve. update is called within the blocking call to winit::EventLoop::run callback, which is not async and already runs within the futures::executor::block_on context created on App::run.
* nannou dependency [winit](https://github.com/rust-windowing/winit) has open github issue [Integration with async ecosystem #1199](https://github.com/rust-windowing/winit/issues/1199) which suggests a root cause that might need to be fixed before it's easy to embed nannou in apps with async runtimes.
* the winit event loop needs to be run on the main application thread on Mac OS or it explicitly explodes saying this.
* nannou seems to assume a concrete native window is essential (except for a web target)


## Tokio

Q: Will `tokio::runtime` run its event loop in a child thread so we can run nannou blocking the main thread?
* we could wrap the nannou invocation in a loop to catch a panic and maybe reset and try again

tokio runtime is usually used like this:

```rust
#[tokio::main]
async fn main() {
    //...
}
```

But more flexibility comes from using [tokio::runtime::Runtime](https://docs.rs/tokio/latest/tokio/runtime/index.html) like this:

```rust

```

Examples of tokio async integration: 

* Integrating tokio with async libraries: https://stackoverflow.com/questions/76467985/how-can-i-extend-the-lifetime-of-nannou-model-using-an-async-websocket-client
* Tokio runtime inside tokio runtime: https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th



## See Also

### Async / Await in Rust

* [ ] [How Rust optimizes async/await I](https://tmandry.gitlab.io/blog/posts/optimizing-await-1/)
* [ ] [why async fn in traits are hard](https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/)
* [ ] Youtube [The Talk You've Been Await-ing for](https://www.youtube.com/watch?v=NNwK5ZPAJCk) QCon SF 2019
* [ ] Youtube [Async I/O in Depth](https://www.youtube.com/watch?v=fdxhcDne2Ww): Thread Pools, Radix Trees, Channels and More - High Performance HTTP Web Servers 

### Nannou

* [ ] github [@altunenes/rusty_art](https://github.com/altunenes/rusty_art) interesting general rust creative coding repo including nannou
* [ ] Youtube _@timClicks_ [Creative Coding in Rust - Playing around with Nannou 0.18](https://www.youtube.com/watch?v=41p5tBGMfxI) see [39:41](https://www.youtube.com/live/41p5tBGMfxI?si=HDVPTQWmHKc-Pmcn&t=2381)

----