use wasm_bindgen::prelude::*;
use js_sys::Promise;

#[wasm_bindgen]
extern "C" {
    pub(crate) type ReadableStreamDefaultReader;

    #[wasm_bindgen(constructor)]
    pub(crate) fn new() -> ReadableStreamDefaultReader;
    
    #[wasm_bindgen(method, getter)]
    pub(crate) fn closed(this: &ReadableStreamDefaultReader) -> Promise;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn cancel(this: &ReadableStreamDefaultReader) -> Result<Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name = cancel)]
    pub(crate) fn cancel_with_reason(this: &ReadableStreamDefaultReader) -> Result<Promise, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn read(this: &ReadableStreamDefaultReader) -> Result<Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name = releaseLock)]
    pub(crate) fn release_lock(this: &ReadableStreamDefaultReader) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
    pub(crate) type ReadableStreamDefaultReaderValue;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn value(this: &ReadableStreamDefaultReaderValue) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn done(this: &ReadableStreamDefaultReaderValue) -> bool;
}
