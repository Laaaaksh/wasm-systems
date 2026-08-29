use std::fs;

fn main() {
    println!("guest: starting up");

    match fs::read_to_string("secret.txt") {
        Ok(contents) => println!("guest: read secret.txt: {}", contents.trim()),
        Err(e) => println!("guest: could not read secret.txt: {e}"),
    }

    println!("guest: done");
}
