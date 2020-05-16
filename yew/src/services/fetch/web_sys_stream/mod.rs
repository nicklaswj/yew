//! Binding for JS ReadableStream and ReadableStreamDefaultReader

/// ReadableStream
pub mod readable_stream;
/// ReadableStreamDefaultReader and ReadableStreamDefaultReaderValue
pub mod readable_stream_default_reader;

use futures::ready;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

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
        /*
        //next_val = none
        if self.next_val.is_none() {
            //inner = some; next_val = none
            if let Some(inner) = self.inner {
                match inner.read() {
                    Ok(future_read) => {
                        self.get_mut().next_val = Some(future_read);
                    }
                    Err(err) => return Poll::Ready(Err(err)),
                }
            } else {
                //inner = none; next_val = none
                return Poll::Ready(None);
            }
        }

        //inner = ?; next_val = some
        let result = ready!(self.next_val.unwrap());
        self.next_val = None;

        Poll::Ready(result)*/

        match (inner, next_val) = (self.inner, self.next_val) {
            //inner = some, next_val = none
            (Some(inner_v), None) => {
                match inner.read() {
                    Ok(future_read) => {
                        self.get_mut().next_val = Some(future_read);
                        let result = ready!(self.next_val.unwrap());
                        self.next_val = None;

                        Poll::Ready(result)
                    }
                    Err(err) => return Poll::Ready(Err(err)),
                }
            }
            //inner = none, next_val = none
            (None, None) => {
                Poll::Ready(None)
            }
            //inner = ?, next_val = some
            (_, Some(next_val_v)) => {
                let result = ready!(self.next_val.unwrap());
                self.next_val = None;

                Poll::Ready(result)
            }
        }
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
