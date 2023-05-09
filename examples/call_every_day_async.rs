use zila::call_every_day_async;

async fn callback() {
    println!("This will be called every day.");
}

#[tokio::main]
async fn main() {
    call_every_day_async(callback).await;
}
