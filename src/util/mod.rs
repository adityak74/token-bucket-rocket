use dotenv::dotenv;
use redis::Connection;
use std::env;

pub struct RedisClientInterface {
    redis_conn_url: String,
}

pub trait RedisClient {
    fn init () -> Self;
    fn get_connection(&self) -> Connection;
}

impl RedisClient for RedisClientInterface {
    fn init() -> RedisClientInterface {
        dotenv().ok();
        println!("baf!");
        let redis_host = env::var("REDIS_HOST")
            .expect("REDIS_HOST must be set");
        let formatted_redis_url = format!("redis://{}", redis_host);
        let redis_conn_url = formatted_redis_url.as_str();
        return RedisClientInterface {
            redis_conn_url: redis_conn_url.to_string(),
        };
    }

    fn get_connection(&self) -> Connection {
        let client = redis::Client::open(self.redis_conn_url.as_str());
        let con = client
            .expect("Invalid connection URL")
            .get_connection()
            .expect("failed to connect to Redis");
        return con;
    }
}