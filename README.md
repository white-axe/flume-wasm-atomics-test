> [!NOTE]
> The issue that this repository was designed to demonstrate has been fixed upstream.
>
> It can be fixed in this repository by updating to flume 0.11.1 and enabling the `"spin"` feature of flume (in Cargo.toml):
>
> ```
> flume = { version = "0.11.1", features = ["spin"] }
> ```

This is a demonstration showing how to use flume to send data between the main thread and a worker thread using a WebAssembly program with atomics support. Currently, it triggers the following error after a varying number of messages are sent:

```
Uncaught (in promise) RuntimeError: Atomics.wait cannot be called in this context
    at core::core_arch::wasm32::atomic::memory_atomic_wait32::h96376a6414a86a87 (flume-wasm-atomics-test_bg.wasm:0xe12d2)
    at std::sys::wasm::futex::futex_wait::h292a0a973fc726d3 (flume-wasm-atomics-test_bg.wasm:0xa5924)
    at std::sys::wasm::locks::futex_mutex::Mutex::lock_contended::hd45bb12cab0c22e6 (flume-wasm-atomics-test_bg.wasm:0x702c4)
    at std::sys::wasm::locks::futex_mutex::Mutex::lock::h73cff9beb960ea5f (flume-wasm-atomics-test_bg.wasm:0x98dd2)
    at std::sync::mutex::Mutex<T>::lock::h1b6b582f0a571882 (flume-wasm-atomics-test_bg.wasm:0xdc6de)
    at flume::wait_lock::h1c6a6297de59474f (flume-wasm-atomics-test_bg.wasm:0xaa943)
    at flume::Shared<T>::recv::h41d724fcd222a605 (flume-wasm-atomics-test_bg.wasm:0x457ca)
    at flume::Shared<T>::recv_sync::h75768094a9f2c6b2 (flume-wasm-atomics-test_bg.wasm:0x9caf9)
    at flume::async::RecvFut<T>::poll_inner::hbd655e3c0d43e014 (flume-wasm-atomics-test_bg.wasm:0x328d0)
    at <flume::async::RecvFut<T> as core::future::future::Future>::poll::hbd3ea43ef3f7793f (flume-wasm-atomics-test_bg.wasm:0xca7d9)
```

To build this program, download Rustup and [Trunk](https://github.com/thedodd/trunk). Use `trunk serve` to build and serve this program, and then go to http://localhost:8080 in a Chromium-based browser. Look in the developer console for errors.

See https://github.com/zesterer/flume/issues/137 for the issue that this example is demonstrating for.
