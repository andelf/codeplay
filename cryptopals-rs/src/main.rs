#[cfg(not(test))]
#[path = "lib.rs"]
mod cryptopals;


#[cfg(not(test))]
fn main() {}


#[cfg(test)]
fn main() {
    println!("Hello Rust!");
}
