use zila::duration_to_next_hour;

fn main() {
    let duration = duration_to_next_hour();
    println!("{:?}", duration);
}
