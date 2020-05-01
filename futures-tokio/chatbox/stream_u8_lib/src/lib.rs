use tokio::stream::{Stream, StreamExt};

use pin_project::pin_project;

use std::pin::Pin;
use std::task::{Context, Poll};

#[pin_project]
pub struct StreamReader<R> {
    source: Vec<R>,
    buffer_length: usize,
    ptr_location: usize,
}

impl <R> StreamReader<R> {
    pub fn new(r: Vec<R>, window_length: usize) -> Self {
        StreamReader {
            source: r,
            buffer_length: window_length,
            ptr_location: 0
        }
    }
}

impl<R: std::clone::Clone> Stream for StreamReader<R> {
    type Item = Vec<R>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let vector_state: &mut Vec<R> = this.source; // Normal reference to the field
        let unpinned_ptr_location: &mut usize = this.ptr_location; // Normal reference to the field

        if this.buffer_length == unpinned_ptr_location {
            Poll::Ready(None)
        } else {
            *unpinned_ptr_location += 2;
            let start = *unpinned_ptr_location-2;
            let end = match *unpinned_ptr_location {
                value if value >= *this.buffer_length => *this.buffer_length,
                _rest => _rest
            };

            Poll::Ready(Some(vector_state[start..end].to_vec()))
        }
    }
}

#[tokio::test]
async fn u32_vector() {
    let mut stream = StreamReader::<u32> { source: vec![1,2,3,4,5] , buffer_length: 5, ptr_location: 0 };

    // the stream goes back and forth
    assert_eq!(stream.next().await, Some(vec![1,2]));
    assert_eq!(stream.next().await, Some(vec![3,4]));
    assert_eq!(stream.next().await, Some(vec![5]));

}

#[tokio::test]
async fn char_vector() {
    let mut stream = StreamReader::<char> { source: "This is a test string.".chars().collect() , buffer_length: 22, ptr_location: 0 };

    // the stream goes back and forth
    assert_eq!(stream.next().await, Some(vec!['T', 'h']));
}

