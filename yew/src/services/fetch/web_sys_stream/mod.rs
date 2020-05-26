//! Binding for JS ReadableStream and ReadableStreamDefaultReader

/// Raw JS bindings to ReadableStream
pub mod sys;
/// Stream implementation of ReadableStreamDefaultReader
pub mod yew_stream;

use wasm_bindgen::prelude::*;
use futures::ready;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use std::pin::Pin;
use wasm_bindgen_futures::JsFuture;
use std::future::Future;

#[derive(Debug)]
/// ReadableStream
pub struct ReadableStream {
    /// Represents the raw object which is provided by bindgen
    inner: sys::ReadableStream,
}

#[derive(Debug)]
/// ReadableStreamDefaultReader
pub struct ReadableStreamDefaultReader {
    /// Represents the raw object which is provided by bindgen
    inner: sys::ReadableStreamDefaultReader,
}

#[derive(Debug)]
/// ReadableStreamDefaultReaderValue
pub struct ReadableStreamDefaultReaderValue {
    /// Represents the raw object which is provided by bindgen
    inner: sys::ReadableStreamDefaultReaderValue,
}

impl From<sys::ReadableStream> for ReadableStream {
    fn from(inner: sys::ReadableStream) -> Self {
        Self { inner }
    }
}

impl From<sys::ReadableStreamDefaultReader> for ReadableStreamDefaultReader {
    fn from(inner: sys::ReadableStreamDefaultReader) -> Self {
        Self { inner }
    }
}

impl From<sys::ReadableStreamDefaultReaderValue> for ReadableStreamDefaultReaderValue {
    fn from(inner: sys::ReadableStreamDefaultReaderValue) -> Self {
        Self { inner }
    }
}

impl ReadableStream {
    /// This function returns whether or not the readable stream is locked to a reader.
    pub fn locked(&self) -> bool {
        self.inner.locked()
    }

    /// Can be called either with or without a reason (just like the javascript version)
    pub fn cancel(&self, reason: Option<&str>) -> Result<impl Future<Output = Result<JsValue, JsValue>>, js_sys::Error> {

        let promise: JsFuture = if let Some(curr_reason) = reason {
            self.inner
                .cancel_with_reason(JsValue::from(curr_reason))
                .map(JsFuture::from)
                .map_err(js_sys::Error::from)?
        } else {
            self.inner
                .cancel()
                .map(JsFuture::from)
                .map_err(js_sys::Error::from)?
        };

        Ok(async {
            let result: Result<JsValue, JsValue> = promise.await;
            result
        })
    }

    /// Used to obtain a reader
    pub fn get_reader(&self) -> Result<ReadableStreamDefaultReader, js_sys::Error> {
        self.inner
            .get_reader()
            .map(ReadableStreamDefaultReader::from) //map the reader from sys to the one defined above
            .map_err(js_sys::Error::from)
    }

    /// Consumes self and return the raw bindings for ReadableStream
    pub fn into_inner(self) -> sys::ReadableStream {
        self.inner
    }
}

impl ReadableStreamDefaultReader {
    /// Creates a new ReadableStreamDefaultReader
    pub fn new() -> Self {
        Self {
            inner: sys::ReadableStreamDefaultReader::new(),
        }
    }

    /// Can be called either with or without a reason (just like the javascript version)
    pub fn cancel(&self, reason: Option<&str>) -> Result<impl Future<Output = Option<String>>, js_sys::Error> {

        let (promise, has_reason) = if let Some(curr_reason) = reason {
            (self.inner
                .cancel_with_reason(JsValue::from(curr_reason))
                .map(JsFuture::from)
                .map_err(js_sys::Error::from)?, true)

        } else {
            (self.inner
                .cancel()
                .map(JsFuture::from)
                .map_err(js_sys::Error::from)?, false)
        };

        Ok(async move {
            if has_reason {
                promise.await
                    .map(|value| Some(value.as_string().unwrap()))
                    .unwrap()
            }
            else {
                promise.await
                    .map(|_| None)
                    .unwrap()
            }
        })
    }

    /// Resolves once a stream closes
    pub fn closed(&self) -> impl Future<Output = Result<(), ()>> {
        let future: JsFuture = JsFuture::from(self.inner.closed());
        async {
            let res:Result<(), ()> = future
                .await
                .map(|_| ())
                .map_err(|_| ());
            res
        }
    }
    
    /// To read a value from the stream
    pub fn read(&self) -> Result<impl Future<Output = Result<ReadableStreamDefaultReaderValue, js_sys::Error>>, js_sys::Error> {

        let promise: JsFuture = self.inner
            .read()
            .map(JsFuture::from)
            .map_err(js_sys::Error::from)?;

        Ok(async {
            let result: Result<JsValue, JsValue> = promise.await;
            result
                .map(ReadableStreamDefaultReaderValue::from)
                .map_err(js_sys::Error::from)
        })
    }

    /// To release the lock on the reader
    pub fn release_lock(&self) -> Result<(), js_sys::Error> {
        self.inner
            .release_lock()
            .map_err(js_sys::Error::from)
    }
}

impl ReadableStreamDefaultReaderValue {
    /// Represents the anonymous object returned by read
    pub fn value(&self) -> Option<js_sys::Uint8ClampedArray> {
        let value = self.inner.value();
        if value.is_undefined() {
            None
        } else {
            Some(js_sys::Uint8ClampedArray::from(self.inner.value()))
        }
    }
    /// Whether the stream is done
    pub fn done(&self) -> bool {
        self.inner.done()
    }
}

impl From<JsValue> for ReadableStreamDefaultReaderValue {
    fn from(value: JsValue) -> Self {
        Self {
            inner: wasm_bindgen::JsCast::unchecked_from_js(value)
        }
    }
}

