use tokio;

#[tokio::main]
async fn main() {
    println!("start!");
    println!("result is {}", add_async_after_1sec(1, 1).await);
}

async fn add_async_after_1sec<T>(a: T, b: T) -> T
    where T: std::ops::Add<Output=T> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    a + b
}