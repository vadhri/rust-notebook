extern crate redis;

use redis::Commands;
use redis::Connection;

use redis::{FromRedisValue, Value};
use crate::{common::types::HashMapType};

pub fn write_hashmap_key(con: &mut Connection, h: &String, k: &String, v: &String) -> redis::RedisResult<()> {
    Ok(con.hset(h, k, v)?)
}

pub fn read_hashmap_key(con: &mut Connection, h: &String) -> redis::RedisResult<HashMapType> {
    let hmap: Value = con.hgetall(h)?;

    let p: Result<HashMapType, _> = FromRedisValue::from_redis_value(&hmap);
    println!("[redis_wrapper] hmap {:?} hset {:?} converted_redis = {:?}", h, hmap, p);

    Ok(p.unwrap())
}

pub fn initialize_redis(url: String) -> redis::RedisResult<Connection> {
    let client = redis::Client::open(url)?;
    Ok(client.get_connection()?)
}
