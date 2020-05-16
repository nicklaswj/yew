//! Binding for JS ReadableStream and ReadableStreamDefaultReader

/// ReadableStream
pub mod readable_stream;
/// ReadableStreamDefaultReader and ReadableStreamDefaultReaderValue
pub mod readable_stream_default_reader;

use std::future::Future;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsValue;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use futures::ready;
use std::pin::Pin;

struct YewStream {
    inner: Option<ReadableStreamDefaultReader>,
    next_val: Option<Pin<Box<dyn Future<Item = Result<JsValue, JsValue>>>>>,
}

impl std::convert::From<ReadableStreamDefaultReader> for YewStream {
    fn from(reader: ReadableStreamDefaultReader) -> Self {
        Self {
            inner: Some(reader),
            next_val: None,
        }
    }
}

impl Stream for YewStream {
    type Item = Result<JsValue, JsValue>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        if self.next_val.is_none() {
            if let Some(inner) = self.inner {
                match inner.read() {
                    Ok(future_read) => {
                        self.get_mut().next_val = Some(
                            future_read
                        );
                    }
                    Err(err) => return Poll::Ready(Err(err)),
                }
            } else {
                return Poll::Ready(None);
            }
        }

        let result = ready!(self.next_val.unwrap());
        self.next_val = None;

        Poll::Ready(result)
    }
}

pub struct ReadableStreamDefaultReader {
    raw: readable_stream_default_reader::ReadableStreamDefaultReader,
}

impl ReadableStreamDefaultReader {
     pub fn closed(&self) -> Future<Output = Result<JsValue, JsValue>> {
        JsFuture::from(self.raw.closed())
     }
}
