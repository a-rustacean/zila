use zila::call_every_minute;

fn main() {
    call_every_minute(|| {
        println!("This will be printed every minute.");
    });
}
