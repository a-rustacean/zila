use std::{cell::Cell, rc::Rc};
use zila::call_every_minute_mut;

fn main() {
    let num = Rc::new(Cell::new(0));
    call_every_minute_mut({
        let num = num.clone();
        move || {
            num.set(num.get() + 1);
            println!("This is printed {} times.", { num.get() });
        }
    });
}
