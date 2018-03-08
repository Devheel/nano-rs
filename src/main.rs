extern crate tokio;
extern crate futures;
extern crate tokio_io;

#[macro_use]
extern crate slog;
extern crate sloggers;

#[macro_use]
extern crate error_chain;

mod error;
use error::*;

fn main() {
    println!("Hello, world!");
}
