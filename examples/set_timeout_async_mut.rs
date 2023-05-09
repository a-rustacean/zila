#![feature(async_closure)]

use std::{cell::Cell, rc::Rc, time::Duration};
use zila::set_timeout_async_mut;

#[tokio::main]
async fn main() {
    let is_called = Rc::new(Cell::new(false));
    set_timeout_async_mut(
        move || {
            let is_called = is_called.clone();
            async move || {
                is_called.set(true);
                println!("This function is called: {}", is_called.get());
            }
        }(),
        Duration::from_secs(2),
    ).await;
}
