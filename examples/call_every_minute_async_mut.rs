#![feature(async_closure)]

use std::{cell::Cell, rc::Rc};
use zila::call_every_minute_async_mut;

#[tokio::main]
async fn main() {
    let num = Rc::new(Cell::new(0));
    call_every_minute_async_mut(move || {
        let num = num.clone();
        async move || {
            num.set(num.get() + 1);
            println!("This is printed {} times.", { num.get() });
        }
    }())
    .await;
}
