#[allow(dead_code)]
use std::future::Future;

fn main() {
    println!("Hello, world!");
}

async fn foo1() -> usize {
    0
}

fn foo2() -> impl Future<Output = usize> {
    async { 0 }
}
