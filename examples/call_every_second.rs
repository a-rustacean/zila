use zila::call_every_second;

fn main() {
    call_every_second(|| {
        println!("This will be printed every second.");
    });
}
