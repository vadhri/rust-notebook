use tokio::stream::{Stream, StreamExt};

use pin_project::pin_project;

use std::pin::Pin;
use std::task::{Context, Poll};

#[pin_project]
pub struct StreamReader<R> {
    source: Vec<R>,
    buffer_length: usize,
    ptr_location: usize,
    window_size: usize
}

use tokio::net::TcpListener;

impl <R> StreamReader<R> {
    pub fn new(r: Vec<R>, window_length: usize) -> Self {
        StreamReader {
            source: r,
            buffer_length: window_length,
            ptr_location: 0,
            window_size: window_length
        }
    }
    pub fn append_source(self: &mut Self, r: Vec<R>) {
        self.buffer_length += r.len();
        for item in r {
            self.source.push(item);
        }
    }

    pub fn reset_pointer_index(self: &mut Self) {
        self.ptr_location = 0;
    }
    pub fn clear_source(self: &mut Self) {
        self.ptr_location = 0;
        self.buffer_length = 0;
        self.source.clear();
    }
    pub fn set_window_size(self: &mut Self, window_size: usize) {
        self.window_size = window_size;
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
            let start = *unpinned_ptr_location;
            let end = match start + *this.window_size {
                value if value >= *this.buffer_length => *this.buffer_length,
                _rest => _rest
            };

            *unpinned_ptr_location += end - start;

            Poll::Ready(Some(vector_state[start..end].to_vec()))
        }
    }
}

#[tokio::test]
async fn u32_vector() {
    let mut stream = StreamReader::<u32> { source: vec![1,2,3,4,5] , buffer_length: 5, ptr_location: 0, window_size: 2 };

    assert_eq!(stream.next().await, Some(vec![1,2]));
    assert_eq!(stream.next().await, Some(vec![3,4]));
    assert_eq!(stream.next().await, Some(vec![5]));

}

#[tokio::test]
async fn char_vector() {
    let mut stream = StreamReader::<char> { source: "This is a test string.".chars().collect() , buffer_length: 22, ptr_location: 0, window_size: 2 };

    assert_eq!(stream.next().await, Some(vec!['T', 'h']));
}

#[tokio::test]
async fn string_vector() {
    let mut stream = StreamReader::<String> { source: vec!["#test 1".to_string(), "#test 2".to_string()] , buffer_length: 2, ptr_location: 0, window_size: 2 };

    assert_eq!(stream.next().await, Some(vec!["#test 1".to_string(), "#test 2".to_string()]));
}


#[tokio::test]
async fn str_vector() {
    let mut stream = StreamReader::<&str> { source: vec!["#test 1", "#test 2"] , buffer_length: 2, ptr_location: 0, window_size: 2 };

    assert_eq!(stream.next().await, Some(vec!["#test 1", "#test 2"]));
}

#[tokio::test]
async fn append_str_vector() {
    let mut stream = StreamReader::<&str> { source: vec!["#test 1", "#test 2"] , buffer_length: 2, ptr_location: 0, window_size: 2 };
    stream.append_source(vec!["#test 3"]);

    assert_eq!(stream.next().await, Some(vec!["#test 1", "#test 2"]));
    assert_eq!(stream.next().await, Some(vec!["#test 3"]));
}


#[tokio::test]
async fn empty_init_append_str_vector() {
    let mut stream = StreamReader::<&str> { source: vec![] , buffer_length: 0, ptr_location: 0, window_size: 2 };
    stream.append_source(vec!["#test 1"]);
    stream.append_source(vec!["#test 2"]);
    stream.append_source(vec!["#test 3"]);

    assert_eq!(stream.next().await, Some(vec!["#test 1", "#test 2"]));
    assert_eq!(stream.next().await, Some(vec!["#test 3"]));
}

#[tokio::test]
async fn empty_init_none() {
    let mut stream = StreamReader::<&str> { source: vec![] , buffer_length: 0, ptr_location: 0, window_size: 2 };

    assert_eq!(stream.next().await, None);
}

#[tokio::test]
async fn reset_ptr_init_none() {
    let mut stream = StreamReader::<&str> { source: vec![] , buffer_length: 0, ptr_location: 0, window_size: 2 };
    stream.append_source(vec!["#test 1"]);

    assert_eq!(stream.next().await, Some(vec!["#test 1"]));
    assert_eq!(stream.next().await, None);

    stream.reset_pointer_index();
    assert_eq!(stream.next().await, Some(vec!["#test 1"]));
}

#[tokio::test]
async fn clear_source () {
    let mut stream = StreamReader::<&str> { source: vec![] , buffer_length: 0, ptr_location: 0, window_size: 2 };
    stream.append_source(vec!["#test 1"]);

    assert_eq!(stream.next().await, Some(vec!["#test 1"]));
    stream.clear_source();
    assert_eq!(stream.next().await, None);
    stream.append_source(vec!["#test 1"]);
    assert_eq!(stream.next().await, Some(vec!["#test 1"]));
}

#[tokio::test]
async fn window_size_less_than_source () {
    let mut stream = StreamReader::<&str> { source: vec![] , buffer_length: 0, ptr_location: 0, window_size: 0 };
    stream.append_source(vec!["#test 1"]);

    assert_eq!(stream.next().await, Some(vec![]));
    stream.set_window_size(2);
    assert_eq!(stream.next().await, Some(vec!["#test 1"]));
}

#[tokio::test]
async fn window_size_greater_than_source () {
    let mut stream = StreamReader::<&str> { source: vec![] , buffer_length: 0, ptr_location: 0, window_size: 0 };
    stream.append_source(vec!["#test 1"]);

    assert_eq!(stream.next().await, Some(vec![]));
    stream.set_window_size(20);
    assert_eq!(stream.next().await, Some(vec!["#test 1"]));
}
