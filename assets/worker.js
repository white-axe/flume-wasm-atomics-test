import init, { worker_start } from '/flume-wasm-atomics-test.js';

onmessage = async function (e) {
    await init('/flume-wasm-atomics-test_bg.wasm', e.data);
    worker_start();
};
