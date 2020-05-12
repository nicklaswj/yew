use wasm_bindgen::prelude::*;
use super::readable_stream_default_reader::ReadableStreamDefaultReader;

#[wasm_bindgen]
extern "C" {
    pub(crate) type ReadableStream;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn locked(this: &ReadableStream) -> bool;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn cancel(this: &ReadableStream, reason: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = getReader)]
    pub(crate) fn get_reader(this: &ReadableStream) -> Result<ReadableStreamDefaultReader, JsValue>;

    #[wasm_bindgen(method, catch, js_name = getReader)]
    pub(crate) fn get_reader_with_mode(this: &ReadableStream, mode: JsValue) -> Result<JsValue, JsValue>;
}
