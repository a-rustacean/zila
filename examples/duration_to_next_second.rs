use zila::duration_to_next_second;

fn main() {
    let duration = duration_to_next_second();
    println!("{:?}", duration);
}
