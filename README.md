# Nannou Tokio

> Nannou Tokio is about to explode...

Shout out to [davidbegin](https://github.com/davidbegin/) and [rockerBOO](https://github.com/rockerBOO/)

This project is an experiment running Nannou in-process with Tokio. It's harder than it initially seemed because they each fight for runtime supremacy. These are notes taken analysing the problem and hopefully useful to understanding solutions.

The main goal is to incorporate nannou in an existing application that uses Tokio to manage the main event loop.

Tokio is more flexible because unlike nannou, tokio can be configured to run in a child threadpool spawned from the main thread.

See [src/main.rs](src/main.rs). Using the `tokio::runtime` API you can spawn a tokio main event loop and later let the main thread be blocked by nannou without affecting the tokio async code. Nannou probably cannot be made to work inside a tokio managed runtime so running spawning tokio explicitly gets around this.


To run the demo:
```shell
cargo run --release 

```
This opens a black nannou window but the tcp echo service is also running inside tokio, so you can interact with it with something like telnet:

```shell
telnet 127.0.0.1 8080
```
and everything you type is echoed back to you:
```console 
yeah
yeah
foo
foo
bar
bar
```

While the nannou main loop is not blocked.

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


tokio runtime is usually used like this:

```rust
#[tokio::main]
async fn main() {
    //...
}
```

But more flexibility comes from using [tokio::runtime::Runtime](https://docs.rs/tokio/latest/tokio/runtime/index.html) from [this example](https://docs.rs/tokio/latest/tokio/runtime/index.html).

Examples of tokio async integration: 

* Integrating tokio with async libraries: https://stackoverflow.com/questions/76467985/how-can-i-extend-the-lifetime-of-nannou-model-using-an-async-websocket-client
* Tokio runtime inside tokio runtime: https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th

## See Also

### Async / Await in Rust

* [How Rust optimizes async/await I](https://tmandry.gitlab.io/blog/posts/optimizing-await-1/)
* [why async fn in traits are hard](https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/)
* Youtube [The Talk You've Been Await-ing for](https://www.youtube.com/watch?v=NNwK5ZPAJCk) QCon SF 2019
* Youtube [Async I/O in Depth](https://www.youtube.com/watch?v=fdxhcDne2Ww): Thread Pools, Radix Trees, Channels and More - High Performance HTTP Web Servers 

### Nannou

* Youtube _@timClicks_ [Creative Coding in Rust - Playing around with Nannou 0.18](https://www.youtube.com/watch?v=41p5tBGMfxI) see [39:41](https://www.youtube.com/live/41p5tBGMfxI?si=HDVPTQWmHKc-Pmcn&t=2381) for discussion about how update changes the model.
* YouTube _@GitHub_ part where model updates are explained: [Nannou: creative coding with Rust - GitHub Universe 2020](https://youtu.be/Ml6tpyTyXhM?si=wi202ZzytEHBEdnC&t=1034)
----