use std::{cell::Cell, rc::Rc, time::Duration};
use zila::set_interval_mut;

fn main() {
    let num = Rc::new(Cell::new(0));
    set_interval_mut(
        {
            let num = num.clone();
            move || {
                num.set(num.get() + 1);
                println!("This message is printed {} times", num.get());
            }
        },
        Duration::from_secs(2),
    );
}
