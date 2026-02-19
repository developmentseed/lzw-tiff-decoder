use wasm_bindgen::prelude::*;
use weezl::decode::Decoder;
use weezl::{BitOrder, LzwStatus};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Call this function at least once during initialization to get better error
// messages if the underlying Rust code ever panics (creates uncaught errors).
#[cfg(feature = "debug")]
#[wasm_bindgen(js_name = setPanicHook)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    console_error_panic_hook::set_once();
}

// Adapted from: https://github.com/image-rs/image-tiff/blob/4900c8287193158e0c9b391b0586a5aa4be23ba3/src/decoder/stream.rs#L301-L353
#[wasm_bindgen]
pub fn decompress(compressed: &[u8], max_uncompressed_length: usize) -> Vec<u8> {
    let mut decoder = Decoder::with_tiff_size_switch(BitOrder::Msb, 8);
    let mut uncompressed = Vec::with_capacity(max_uncompressed_length);
    let mut bytes_read = 0;

    while uncompressed.len() < max_uncompressed_length {
        let bytes_written = uncompressed.len();

        // Resize vector only if needed
        uncompressed.reserve(1 << 12);
        let buffer_space = uncompressed.capacity().min(max_uncompressed_length);
        // Initialize unwritten bytes with zeros
        uncompressed.resize(buffer_space, 0u8);

        // Decode unread portion into unwritten
        let result = decoder.decode_bytes(
            &compressed[bytes_read..],
            &mut uncompressed[bytes_written..],
        );

        bytes_read += result.consumed_in;
        uncompressed.truncate(bytes_written + result.consumed_out);

        if let Ok(LzwStatus::Done) = result.status {
            // Just check if it's finished.
            break;
        }
    }

    uncompressed
}
