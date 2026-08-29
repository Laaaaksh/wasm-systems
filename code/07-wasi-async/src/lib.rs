// A guest export that returns a WASI 0.3 async `stream<u32>` instead of a
// plain value. `count_to` doesn't compute anything before returning - it
// creates a stream pair, spawns a task that writes into it, and hands the
// read half back immediately. The caller receives values as they're
// produced, not all at once after the call returns.
//
// See README.md for exactly how much of this is runnable today (short
// version: this builds and validates as a real component; running it
// needs an async-aware host, which is not what `wasmtime run` is).
wit_bindgen::generate!({
    path: "wit",
    world: "async-demo",
});

struct Component;

impl Guest for Component {
    fn count_to(n: u32) -> wit_bindgen::rt::async_support::StreamReader<u32> {
        let (mut writer, reader) = wit_stream::new::<u32>();
        wit_bindgen::rt::async_support::spawn(async move {
            for i in 0..n {
                writer.write(vec![i]).await;
            }
            // Dropping `writer` here closes the stream, so the reader's
            // last `read()` reports `StreamResult::Dropped` instead of
            // hanging forever waiting for one more value.
        });
        reader
    }
}

export!(Component);
