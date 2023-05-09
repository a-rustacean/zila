use std::time::Duration;
use zila::set_timeout;

fn main() {
    set_timeout(
        || {
            println!("This will be printed after two second");
        },
        Duration::from_secs(2),
    );
}
