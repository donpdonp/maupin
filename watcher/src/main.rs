use watcher::*;

fn main() {
    println!("Hello, world!");
    match update::update() {
        Ok(body) => println!("{:?}", body),
        Err(e) => println!("{}", e),
    }
}
