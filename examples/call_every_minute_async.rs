use zila::call_every_minute_async;

async fn callback() {
    println!("This will be called every minute.");
}

#[tokio::main]
async fn main() {
    call_every_minute_async(callback).await;
}
