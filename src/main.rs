#[macro_use] 
extern crate rocket;
extern crate redis;

use redis::Commands;
use dotenv::dotenv;
use std::env;

#[get("/")]
fn index() -> String {
    dotenv().ok();
    let redis_host = env::var("REDIS_HOST")
        .expect("REDIS_HOST must be set");
    let redis_conn_url = format!("redis://{}", redis_host);
    let client = redis::Client::open(redis_conn_url);
    let mut con = client
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis");
    // throw away the result, just make sure it does not fail
    let _ : () = con.set("my_key", 42).expect("cannot set key");
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    let value: i32 = con.get("my_key").expect("cannot get key");
    // print!("value : {value}");
    return value.to_string();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
