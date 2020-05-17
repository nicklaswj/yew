//! Binding for JS ReadableStream and ReadableStreamDefaultReader

/// Raw JS bindings to ReadableStream
pub mod sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

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
    pub fn cancel(&self, reason: Option<&str>) -> Result<JsFuture, js_sys::Error> {
        if let Some(curr_reason) = reason {
            self.inner
                .cancel_with_reason(JsValue::from(curr_reason))
                .map(JsFuture::from)
                .map_err(js_sys::Error::from)
        } else {
            self.inner
                .cancel()
                .map(JsFuture::from)
                .map_err(js_sys::Error::from)
        }
    }

    /// Used to obtain a reader
    pub fn get_reader(&self) -> Result<ReadableStreamDefaultReader, js_sys::Error> {
        self.inner
            .get_reader()
            .map(ReadableStreamDefaultReader::from) //map the reader from sys to the one defined above
            .map_err(js_sys::Error::from)
    }
}

impl ReadableStreamDefaultReader {}

impl ReadableStreamDefaultReaderValue {}

//struct YewStream {
//    inner: Option<ReadableStreamDefaultReader>,
//    next_val: Option<Pin<Box<dyn Future<Item = Result<JsValue, JsValue>>>>>,
//}
//
//impl std::convert::From<ReadableStreamDefaultReader> for YewStream {
//    fn from(reader: ReadableStreamDefaultReader) -> Self {
//        Self {
//            inner: Some(reader),
//            next_val: None,
//        }
//    }
//}
//
//impl Stream for YewStream {
//    type Item = Result<JsValue, JsValue>;
//
//    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
//        /*
//        //next_val = none
//        if self.next_val.is_none() {
//            //inner = some; next_val = none
//            if let Some(inner) = self.inner {
//                match inner.read() {
//                    Ok(future_read) => {
//                        self.get_mut().next_val = Some(future_read);
//                    }
//                    Err(err) => return Poll::Ready(Err(err)),
//                }
//            } else {
//                //inner = none; next_val = none
//                return Poll::Ready(None);
//            }
//        }
//
//        //inner = ?; next_val = some
//        let result = ready!(self.next_val.unwrap());
//        self.next_val = None;
//
//        Poll::Ready(result)*/
//
//        match (inner, next_val) = (self.inner, self.next_val) {
//            //inner = some, next_val = none
//            (Some(inner_v), None) => {
//                match inner.read() {
//                    Ok(future_read) => {
//                        self.get_mut().next_val = Some(future_read);
//                        let result = ready!(self.next_val.unwrap());
//                        self.next_val = None;
//
//                        Poll::Ready(result)
//                    }
//                    Err(err) => return Poll::Ready(Err(err)),
//                }
//            }
//            //inner = none, next_val = none
//            (None, None) => {
//                Poll::Ready(None)
//            }
//            //inner = ?, next_val = some
//            (_, Some(next_val_v)) => {
//                let result = ready!(self.next_val.unwrap());
//                self.next_val = None;
//
//                Poll::Ready(result)
//            }
//        }
//    }
//}
//
//pub struct ReadableStreamDefaultReader {
//    raw: readable_stream_default_reader::ReadableStreamDefaultReader,
//}
//
//impl ReadableStreamDefaultReader {
//    pub fn closed(&self) -> Future<Output = Result<JsValue, JsValue>> {
//        JsFuture::from(self.raw.closed())
//    }
//}
