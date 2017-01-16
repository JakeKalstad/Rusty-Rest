#![feature(optin_builtin_traits)]
extern crate futures;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

use iron::typemap::Key;

use std;
use r2d2::{Pool, PooledConnection, ManageConnection};
use r2d2_redis::{RedisConnectionManager, Error};
use redis::*;
pub type RedisPool = Pool<RedisConnectionManager>;
pub type RedisPooledConnection = PooledConnection<RedisConnectionManager>;

pub struct AppRedis;
impl Key for AppRedis {
    type Value = RedisPool;
}


pub fn setup_connection_pool(cn_str: &str, pool_size: u32) -> RedisPool {
    let manager = ::r2d2_redis::RedisConnectionManager::new(cn_str).unwrap();
    let config = ::r2d2::Config::builder().pool_size(pool_size).build();
    return r2d2::Pool::new(config, manager).unwrap();
}

pub fn set(conn: &RedisPooledConnection, key: String, data: String) {
    let _: std::result::Result<String, redis::RedisError> = conn.set(key, data);

}
