extern crate redis;

use redis::{
    Commands,
    Connection,
    Client
};
use redis::{
    FromRedisValue,
    Value
};
use crate::common::types::HashMapType;

pub fn write_hashmap_key(con: &mut Connection, h: &String, k: &String, v: &String) -> redis::RedisResult<()> {
    Ok(con.hset(h, k, v)?)
}

pub fn read_hashmap_key(con: &mut Connection, h: &String) -> redis::RedisResult<HashMapType> {
    let hmap: Value = con.hgetall(h)?;
    let p: Result<HashMapType, _> = FromRedisValue::from_redis_value(&hmap);

    println!("[redis_wrapper] hmap {:?} hset {:?} converted_redis = {:?}", h, hmap, p);

    Ok(p.unwrap())
}

pub fn check_key_exists(con: &mut Connection, h: &String, k: &String) -> redis::RedisResult<bool> {
    con.hexists(h, k)
}

pub async fn check_key_exists_multiplexed(con: redis::aio::MultiplexedConnection, h: &String, k: &String) -> redis::RedisResult<bool> {
    redis::cmd("HEXISTS")
        .arg(h)
        .arg(k)
        .query_async(&mut con.clone())
        .await
}

pub async fn write_hashmap_key_multiplexed(con: redis::aio::MultiplexedConnection, h: String, k: String, v: String) -> redis::RedisResult<()> {
    redis::cmd("HSET")
        .arg(h)
        .arg(k)
        .arg(v)
        .query_async(&mut con.clone())
        .await
}

pub fn initialize_redis(url: String) -> redis::RedisResult<Client> {
    Ok(redis::Client::open(url)?)
}
