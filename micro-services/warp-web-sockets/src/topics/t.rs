use crate::redis_wrapper::init::{self, read_hashmap_key, write_hashmap_key};
use redis::Connection;

pub fn get_topics_list(con: &mut Connection) {
    let hm = read_hashmap_key(con, &("topics".to_string()));

    println!("hm = {:?}", hm);
}
