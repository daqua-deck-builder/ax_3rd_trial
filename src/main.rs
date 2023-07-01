use std::fmt;
use std::fmt::{Debug, Display};
use tokio;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    println!("start!");
    println!("result is {}", add_async_after_1sec(1, 1).await);

    let returned: JoinHandle<SomeValue> = tokio::spawn(async_struct());
    println!("{:?}", returned);
    println!("{}", returned.await.unwrap());
}

async fn add_async_after_1sec<T>(a: T, b: T) -> T
    where T: std::ops::Add<Output=T> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    a + b
}

#[derive(Debug)]
struct SomeValue(i32);

impl Display for SomeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SomeValue's value is ({})", self.0)
    }
}

async fn async_struct() -> SomeValue {
    SomeValue(100)
}