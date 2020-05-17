use super::{ReadableStreamDefaultReader, ReadableStreamDefaultReaderValue};
use std::future::Future;
use futures::stream::Stream;
use futures::task::{Poll, Context};
use futures::ready;
use wasm_bindgen_futures::JsFuture;
use std::pin::Pin;
use std::fmt;
use std::convert::From;

/// Internal state of the IntoStream stream
enum StreamState {
    ReadyPoll(ReadableStreamDefaultReader),
    Pending(ReadableStreamDefaultReader, Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>, js_sys::Error>>>>),
}

impl fmt::Debug for StreamState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StreamState::ReadyPoll(_) => f.write_str("StreamState::ReadyPoll"),
            StreamState::Pending(_,_) => f.write_str("StreamState::Pending"),
        }
    }
}

/// Implements futures::stream::Stream for ReadableStreamDefaultReader
#[derive(Debug)]
pub struct IntoStream (Option<StreamState>);

impl From<ReadableStreamDefaultReader> for IntoStream {
    fn from(reader: ReadableStreamDefaultReader) -> Self {
        Self(reader)
    }
}

impl Stream for IntoStream {
    type Item = Result<Vec<u8>, js_sys::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match self.get_mut().0.take() {
                Some(StreamState::ReadyPoll(stream)) => {
                    // TODO call stream.read() here instead this fake future
                    let future_value = async {
                        Err::<ReadableStreamDefaultReaderValue, js_sys::Error>(
                            js_sys::Error::new("Not implemented")
                        )
                    };

                    let stream_future = Box::pin(async move {
                        let value = future_value.await?;

                        if value.done() {
                            Ok(None)
                        } else {
                            Ok(value.value())
                        }
                    });

                    self.get_mut().0 = Some(StreamState::Pending(stream, stream_future));
                }
                Some(StreamState::Pending(stream, future_value)) => {
                    return match future_value.as_mut().poll(cx) {
                        Poll::Ready(Ok(None)) => {
                            self.get_mut().0 = None;
                            Poll::Ready(None)
                        }
                        Poll::Ready(Ok(Some(data))) => {
                            self.get_mut().0 = Some(StreamState::ReadyPoll(stream));
                            Poll::Ready(Some(Ok(data)))
                        }
                        Poll::Ready(Err(err)) => {
                            self.get_mut().0 = Some(StreamState::ReadyPoll(stream));
                            Poll::Ready(Some(Err(err)))
                        }
                        Poll::Pending => {
                            self.get_mut().0 = Some(StreamState::ReadyPoll(stream));
                            Poll::Pending
                        }
                    }
                },
                None => return Poll::Ready(None),
            }
        }
    }
}
