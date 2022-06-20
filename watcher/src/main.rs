use watcher::*;

fn main() {
    println!("Hello, world!");
    match update::update() {
        Ok(response) => println!("{}", response.accounts.len()),
        Err(e) => println!("{}", e),
    }
}
