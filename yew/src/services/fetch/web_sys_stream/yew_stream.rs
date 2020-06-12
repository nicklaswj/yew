use super::{ReadableStream, ReadableStreamDefaultReader, sys};
use std::future::Future;
use futures::stream::{Stream, StreamExt};
use futures::task::{Poll, Context};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsValue, JsCast};
use std::pin::Pin;
use std::fmt;
use std::convert::From;
use std::marker::PhantomData;
use crate::callback::Callback;
use crate::format::Binary;
use anyhow::{anyhow, Error};
use std::convert::{TryFrom, TryInto};

/// Internal state of the YewStream stream
enum StreamState {
    ReadyPoll(ReadableStreamDefaultReader),
    Pending(ReadableStreamDefaultReader, Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>, Error>>>>),
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
pub struct YewStream<OUT> {
    stream: ReadableStream,
    state: Option<StreamState>,
    failed_once: bool,
    _marker: PhantomData<OUT>,
}

impl<OUT> TryFrom<ReadableStream> for YewStream<OUT> {
    type Error = Error;

    fn try_from(stream: ReadableStream) -> Result<Self, Error> {
        let reader = stream.get_reader()
            .map_err(|e| anyhow!(e.to_string().as_string().unwrap()))?;
        Ok(Self {
            stream,
            state: Some(StreamState::ReadyPoll(reader)),
            failed_once: false,
            _marker: PhantomData::default(),
        })
    }
}

/// From a JS ReadableStream
impl<OUT> TryFrom<JsValue> for YewStream<OUT>
{
    type Error = Error;

    fn try_from(stream: JsValue) -> Result<Self, Error> {
        let stream: ReadableStream = stream.dyn_into::<sys::ReadableStream>()
            .map_err(|_| anyhow!("Failed to cast JsValue into sys::ReadableStream"))?
            .into();

        stream.try_into()
    }
}

impl<OUT: From<Binary> + Unpin> Stream for YewStream<OUT> {
    type Item = OUT;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let into_stream: &mut Option<StreamState> = &mut self.get_mut().state;
        loop {
            match into_stream.take() {
                Some(StreamState::ReadyPoll(stream)) => {
                    let future_value = stream.read();

                    let stream_future = Box::pin(async move {
                        let value = future_value
                            .map_err(|e| anyhow!(e.to_string().as_string().unwrap()))?
                            .await
                            .map_err(|e| anyhow!(e.to_string().as_string().unwrap()))?;

                        if value.done() {
                            Ok(None)
                        } else {
                            Ok(value.value().map(|array| array.to_vec().into()))
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
                            Poll::Ready(Some(Ok(data).into()))
                        }
                        Poll::Ready(Err(err)) => {
                            *into_stream = Some(StreamState::ReadyPoll(stream));
                            Poll::Ready(Some(Err(err).into()))
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

impl<OUT> Drop for YewStream<OUT> {
   fn drop(&mut self) {
       if let Some(state) = &self.state {
           let stream = match &state {
               StreamState::ReadyPoll(stream) => stream,
               StreamState::Pending(stream, _) => stream 
           };

           stream.release_lock().unwrap();
       }
   }
}

/// Enum that represents a chunk of a stream
#[derive(Clone, Debug)]
pub enum StreamChunk<OUT: fmt::Debug> {
    /// The next read data chunk
    DataChunk(OUT),
    /// The stream finished
    Finished,
}

impl<OUT> YewStream<OUT>
where
    OUT: 'static + From<Binary> + Unpin + fmt::Debug
{
    /// Consumes the stream and calls the callback for every data chunk
    pub fn consume_with_callback(mut self, callback: Callback<StreamChunk<OUT>>) {
        let future = async move {
            while let Some(res) = self.next().await {
                callback.emit(StreamChunk::DataChunk(res));

                if self.failed_once {
                    break
                }
            }

            callback.emit(StreamChunk::Finished)
        };
        
        spawn_local(future)
    }
}
