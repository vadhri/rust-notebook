use chatbox_lib::ChatBox;
use futures::executor;

use tokio::stream::{Stream, StreamExt};
use tokio::stream::iter;

mod reader;

use stream_u8_lib::StreamReader;

fn main() {
    let vector = vec![1,2,3];

    let cb = ChatBox::<String>::new();

    let stream_ft = async {
        let mut stream = StreamReader::<u32>::new(vec![1,2,3,4,5], 5);

        assert_eq!(stream.next().await, Some(vec![1,2]));

    };

    executor::block_on(stream_ft);
}
