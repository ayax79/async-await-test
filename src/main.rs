#![feature(await_macro, async_await, futures_api)]

use std::process::Command;
use tokio_process::CommandExt;

fn main() {
    tokio::run_async(main_async());
}

async fn main_async() {
    let out = Command::new("echo")
        .arg("hello")
        .arg("world")
        .output_async();
    let s = tokio::await!(out);
    println!("{:?}", s);
}