use std::time::Duration;
use zila::set_timeout_async;

async fn callback() {
    println!("This will be printed after two second");
}

#[tokio::main]
async fn main() {
    set_timeout_async(callback, Duration::from_secs(2)).await;
}
