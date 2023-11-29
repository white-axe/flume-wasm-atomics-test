import init, { main_start } from '/flume-wasm-atomics-test.js';

await init('/flume-wasm-atomics-test_bg.wasm');
main_start();
