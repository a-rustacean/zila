use zila::duration_to_next_minute;

fn main() {
    let duration = duration_to_next_minute();
    println!("{:?}", duration);
}
