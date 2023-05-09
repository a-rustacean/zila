use zila::call_every_day;

fn main() {
    call_every_day(|| {
        println!("This will be printed every day.");
    });
}
