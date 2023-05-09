#![feature(async_closure)]

use std::{cell::Cell, rc::Rc, time::Duration};
use zila::set_interval_async_mut;

#[tokio::main]
async fn main() {
    let num = Rc::new(Cell::new(0));
    set_interval_async_mut(
        move || {
            let num = num.clone();
            async move || {
                num.set(num.get() + 1);
                println!("This message is printed {} times", num.get());
            }
        }(),
        Duration::from_secs(2),
    ).await;
}
