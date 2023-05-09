use zila::call_every_hour_async;

async fn callback() {
    println!("This will be called every hour.");
}

#[tokio::main]
async fn main() {
    call_every_hour_async(callback).await;
}
