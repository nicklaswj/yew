use wasm_bindgen::prelude::*;
use js_sys::Promise;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type ReadableStreamDefaultReader;

    #[wasm_bindgen(constructor)]
    pub fn new() -> ReadableStreamDefaultReader;
    
    #[wasm_bindgen(method, getter)]
    pub fn closed(this: &ReadableStreamDefaultReader) -> Promise;

    #[wasm_bindgen(method, catch)]
    pub fn cancel(this: &ReadableStreamDefaultReader) -> Result<Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name = cancel)]
    pub fn cancel_with_reason(this: &ReadableStreamDefaultReader, reason: JsValue) -> Result<Promise, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn read(this: &ReadableStreamDefaultReader) -> Result<Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name = releaseLock)]
    pub fn release_lock(this: &ReadableStreamDefaultReader) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type ReadableStreamDefaultReaderValue;

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &ReadableStreamDefaultReaderValue) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn done(this: &ReadableStreamDefaultReaderValue) -> bool;
}
