use tokio::stream::{Stream, StreamExt};
use pin_project::pin_project;
use tokio::io::AsyncRead;

use std::pin::Pin;
use std::task::{Context, Poll};

#[pin_project]
struct ReaderStream <R> where R: Clone {
    source: Vec<R>,
    buffer_length: usize,
    ptr_location: usize,
}

impl<R> Stream for ReaderStream<R> where R : AsyncRead + Unpin + Clone {
    type Item = R;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        if this.buffer_length == this.ptr_location {
            Poll::Ready(None)
        } else {
            *this.ptr_location += 1;
            let ptr_value = &this.source[*this.ptr_location - 1];
            Poll::Ready(Some(ptr_value.clone()))
        }
    }
}

#[tokio::test] 
async fn basic_usage() {
    let mut stream = ReaderStream { source: vec![1,2,3] , buffer_length: 3, ptr_location: 0 };
    assert_eq!(stream.next().await, Some(1));
    assert_eq!(stream.next().await, Some(2));
    assert_eq!(stream.next().await, Some(3));
    assert_eq!(stream.next().await, None);
}
