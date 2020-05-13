use wasm_bindgen::prelude::*;
use super::readable_stream_default_reader::ReadableStreamDefaultReader;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type ReadableStream;

    #[wasm_bindgen(method, getter)]
    pub fn locked(this: &ReadableStream) -> bool;

    #[wasm_bindgen(method, catch)]
    pub fn cancel(this: &ReadableStream) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn cancel_with_reason(this: &ReadableStream, reason: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = getReader)]
    pub fn get_reader(this: &ReadableStream) -> Result<ReadableStreamDefaultReader, JsValue>;

    #[wasm_bindgen(method, catch, js_name = getReader)]
    pub fn get_reader_with_mode(this: &ReadableStream, mode: JsValue) -> Result<JsValue, JsValue>;
}
