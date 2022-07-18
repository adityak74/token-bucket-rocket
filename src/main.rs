#[macro_use] 
extern crate rocket;
extern crate redis;
use redis::Connection;

use redis::Commands;
use util::RedisClientInterface;

mod util;
use crate::util::RedisClient;

#[get("/")]
fn index() -> String {
    // throw away the result, just make sure it does not fail
    let redis: RedisClientInterface = RedisClient::init();
    let mut redis_connection: Connection = redis.get_connection();
    let _ : () = redis_connection.set("my_key", 42).expect("cannot set key");
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    let value: i32 = redis_connection.get("my_key").expect("cannot get key");
    // print!("value : {value}");
    return value.to_string();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
