use zila::call_every_second_async;

async fn callback() {
    println!("This will be called every second.");
}

#[tokio::main]
async fn main() {
    call_every_second_async(callback).await;
}
