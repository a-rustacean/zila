use std::time::Duration;
use zila::set_interval;

fn main() {
    set_interval(
        || {
            println!("This will be printed every two second");
        },
        Duration::from_secs(2),
    );
}
