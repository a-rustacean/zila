use zila::call_every_hour;

fn main() {
    call_every_hour(|| {
        println!("This will be printed every hour.");
    });
}
