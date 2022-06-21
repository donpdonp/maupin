use redis;
use watcher::*;

fn main() {
    let redis = redis::Client::open("redis://127.0.0.1/").unwrap();
    println!("compound monitor");
    match update::update() {
        Ok(accounts) => update::merge(redis, accounts),
        Err(e) => println!("{}", e),
    }
}
