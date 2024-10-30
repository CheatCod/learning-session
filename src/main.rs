#![allow(dead_code, unused_variables, unused_imports, clippy::useless_vec)]

pub mod intro_to_async;
pub mod intro_to_multi_threading;

async fn async_fn() {
    println!("Hello, world!");
}

#[tokio::main]
async fn main() {
    tokio::spawn(async_fn());
}
