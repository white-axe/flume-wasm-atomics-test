use wasm_bindgen::prelude::*;

fn main() {}

struct WorkerData {
    worker_tx: flume::Sender<()>,
    worker_rx: flume::Receiver<()>,
}

static WORKER_DATA: parking_lot::Mutex<Option<WorkerData>> = parking_lot::Mutex::new(None);

#[wasm_bindgen]
pub fn main_start() {
    console_error_panic_hook::set_once();

    let (main_tx, worker_rx) = flume::bounded(1000);
    let (worker_tx, main_rx) = flume::bounded(1000);

    *WORKER_DATA.lock() = Some(WorkerData {
        worker_tx,
        worker_rx,
    });

    wasm_bindgen_futures::spawn_local(async move {
        loop {
            let _ = main_tx.try_send(());

            main_rx
                .recv_async()
                .await
                .expect("failed to receive to main from worker");

            let _ = main_rx.try_iter().count();
        }
    });

    let mut worker_options = web_sys::WorkerOptions::new();
    worker_options.name("worker");
    worker_options.type_(web_sys::WorkerType::Module);
    let worker = web_sys::Worker::new_with_options("/worker.js", &worker_options)
        .expect("failed to spawn web worker");

    worker
        .post_message(&wasm_bindgen::memory())
        .expect("failed to post message to web worker");
}

#[wasm_bindgen]
pub fn worker_start() {
    let WorkerData {
        worker_tx,
        worker_rx,
    } = WORKER_DATA.lock().take().unwrap();

    let mut count = 0u64;
    loop {
        let new = worker_rx.try_iter().count() as u64;
        count += new;
        if new != 0 && (count < 100 || count % 100 == 0) {
            web_sys::console::log_1(&JsValue::from(format!(
                "Worker thread received {count} times"
            )));
        }

        std::thread::sleep(std::time::Duration::from_nanos(123456));
        let _ = worker_tx.try_send(());
    }
}
