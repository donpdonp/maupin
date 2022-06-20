use watcher::*;

fn main() {
    println!("Hello, world!");
    match update::update() {
        Ok(_) => println!("ok"),
        Err(e) => println!("{}", e),
    }
}
