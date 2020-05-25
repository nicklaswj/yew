use super::{ReadableStreamDefaultReader, ReadableStreamDefaultReaderValue};
use std::future::Future;
use futures::stream::Stream;
use futures::task::{Poll, Context};
use futures::ready;
use wasm_bindgen_futures::JsFuture;
use std::pin::Pin;
use std::fmt;
use std::convert::From;

/// Internal state of the YewStream stream
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
pub struct YewStream (Option<StreamState>);

impl From<ReadableStreamDefaultReader> for YewStream {
    fn from(reader: ReadableStreamDefaultReader) -> Self {
        Self(Some(StreamState::ReadyPoll(reader)))
    }
}

impl Stream for YewStream {
    type Item = Result<Vec<u8>, js_sys::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let into_stream: &mut Option<StreamState> = &mut self.get_mut().0;
        loop {
            match into_stream.take() {
                Some(StreamState::ReadyPoll(stream)) => {
                    let future_value = stream.read();

                    let stream_future = Box::pin(async move {
                        let value = future_value?.await?;

                        if value.done() {
                            Ok(None)
                        } else {
                            Ok(value.value().map(|array| array.to_vec()))
                        }
                    });

                    *into_stream = Some(StreamState::Pending(stream, stream_future));
                }
                Some(StreamState::Pending(stream, mut future_value)) => {
                    return match future_value.as_mut().poll(cx) {
                        Poll::Ready(Ok(None)) => {
                            *into_stream = None;
                            Poll::Ready(None)
                        }
                        Poll::Ready(Ok(Some(data))) => {
                            *into_stream = Some(StreamState::ReadyPoll(stream));
                            Poll::Ready(Some(Ok(data)))
                        }
                        Poll::Ready(Err(err)) => {
                            *into_stream = Some(StreamState::ReadyPoll(stream));
                            Poll::Ready(Some(Err(err)))
                        }
                        Poll::Pending => {
                            *into_stream = Some(StreamState::Pending(stream, future_value));
                            Poll::Pending
                        }
                    }
                },
                None => return Poll::Ready(None),
            }
        }
    }
}

impl Drop for YewStream {
    fn drop(&mut self) {
        if let Some(state) = self.0 {
            let stream = match &state {
                StreamState::ReadyPoll(stream) => stream,
                StreamState::Pending(stream, _) => stream 
            };

            stream.release_lock().unwrap();
        }
    }
}


