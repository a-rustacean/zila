use std::time::Duration;
use zila::set_interval_async;

async fn callback() {
    println!("This will be printed every two second");
}

#[tokio::main]
async fn main() {
    set_interval_async(callback, Duration::from_secs(2)).await;
}
