use zila::duration_to_next_day;

fn main() {
    let duration = duration_to_next_day();
    println!("{:?}", duration);
}
