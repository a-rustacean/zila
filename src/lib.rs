#![warn(
    missing_debug_implementations,
    missing_docs,
)]

//! A library for calling function on certain events.
//!
//! Zila is a fast and reliable library for performing tasks on certian events.
//! It propvides both syncronuos and asyncronuos functions to make writing Rust code esier.
//! 
//! # A Tour of Zila
//!
//! Zila consists of a number of functions that provide
//! essential for implementing event based applications in Rust. In this
//! section, we will take a brief tour of zila, summarizing the major APIs and
//! their uses.
//!
//! The easiest way to get started is to enable all features. Do this by
//! enabling the `full` feature flag:
//!
//! ```toml
//! zila = { version = "1", features = ["full"] }
//! ```
//!
//! ### Authoring applications
//!
//! Zila is great for writing applications and most users in this case shouldn't
//! worry too much about what features they should pick. If you're unsure, we suggest
//! going with `full` to ensure that you don't run into any road blocks while you're
//! building your application.
//!
//! #### Example
//!
//! This example shows the quickest way to get started with Zila.
//!
//! ```toml
//! zila = { version = "1", features = ["full"] }
//! ```


use chrono::{Local, Timelike};
use std::future::Future;
use tokio::time::{sleep, Duration};


/// Returns the duration to next day (00:00:00.000.000.000)
#[cfg(feature = "day")]
pub fn duration_to_next_day() -> Duration {
    let time = Local::now();

    if time.hour() == 0 && time.minute() == 0 && time.second() == 0 && time.timestamp_subsec_nanos() == 0 {
        Duration::ZERO
    } else {
        Duration::from_nanos(
            (24 * 60 * 60 * 1000 * 1000 * 1000)
                - (time.hour() as u64 * 60 * 60 * 1000 * 1000 * 1000 + time.minute() as u64 * 60 * 1000 * 1000 * 1000
                    + time.second() as u64 * 1000 * 1000 * 1000
                    + time.timestamp_subsec_nanos() as u64),
        )
    }
}

/// Returns the duration to next hour (\_\_:00::00.000.000.000)
#[cfg(feature = "hour")]
pub fn duration_to_next_hour() -> Duration {
    let time = Local::now();

    if time.minute() == 0 && time.second() == 0 && time.timestamp_subsec_nanos() == 0 {
        Duration::ZERO
    } else {
        Duration::from_nanos(
            (60 * 60 * 1000 * 1000 * 1000)
                - (time.minute() as u64 * 60 * 1000 * 1000 * 1000
                    + time.second() as u64 * 1000 * 1000 * 1000
                    + time.timestamp_subsec_nanos() as u64),
        )
    }
}

/// Returns the duration to next minute (\_\_:\_\_:00.000.000.000)
#[cfg(feature = "minute")]
pub fn duration_to_next_minute() -> Duration {
    let time = Local::now();

    if time.second() == 0 && time.timestamp_subsec_nanos() == 0 {
        Duration::ZERO
    } else {
        Duration::from_nanos(
            (60 * 1000 * 1000 * 1000)
                - (time.second() as u64 * 1000 * 1000 * 1000
                    + time.timestamp_subsec_nanos() as u64),
        )
    }
}


/// Returns the duration to next second (\_\_:\_\_:\_\_.000.000.000)
#[cfg(feature = "second")]
pub fn duration_to_next_second() -> Duration {
    let time = Local::now();

    if time.timestamp_subsec_nanos() == 0 {
        Duration::ZERO
    } else {
        Duration::from_nanos(1000 * 1000 * 1000 - time.timestamp_subsec_nanos() as u64)
    }
}

/// calls the given function every day
#[cfg(feature = "day")]
pub fn call_every_day<F>(callback: F)
where
    F: Fn(),
{
    loop {
        let duration = duration_to_next_day();
        std::thread::sleep(duration);
        callback();
    }
}

/// calls the given function every day, takes a `FnMut()` as the argument
#[cfg(feature = "day")]
pub fn call_every_day_mut<F>(mut callback: F)
where
    F: FnMut(),
{
    loop {
        let duration = duration_to_next_day();
        std::thread::sleep(duration);
        callback();
    }
}

/// calls the given async function every day
#[cfg(feature = "day")]
pub async fn call_every_day_async<F, Fut>(callback: F)
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    loop {
        let duration = duration_to_next_day();
        sleep(duration).await;
        callback().await;
    }
}


/// calls the given async function every day, takes a `FnMut()` as the argument
#[cfg(feature = "day")]
pub async fn call_every_day_async_mut<F, Fut>(mut callback: F)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = ()>,
{
    loop {
        let duration = duration_to_next_day();
        sleep(duration).await;
        callback().await;
    }
}

/// calls the given function every hour
#[cfg(feature = "hour")]
pub fn call_every_hour<F>(callback: F)
where
    F: Fn(),
{
    loop {
        let duration = duration_to_next_hour();
        std::thread::sleep(duration);
        callback();
    }
}

/// calls the given function every hour, takes a `FnMut()` as the argument
#[cfg(feature = "hour")]
pub fn call_every_hour_mut<F>(mut callback: F)
where
    F: FnMut(),
{
    loop {
        let duration = duration_to_next_hour();
        std::thread::sleep(duration);
        callback();
    }
}

/// calls the given async function every hour
#[cfg(feature = "hour")]
pub async fn call_every_hour_async<F, Fut>(callback: F)
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    loop {
        let duration = duration_to_next_hour();
        sleep(duration).await;
        callback().await;
    }
}

/// calls the given async function every hour, takes a `FnMut()` as the argument
#[cfg(feature = "hour")]
pub async fn call_every_hour_async_mut<F, Fut>(mut callback: F)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = ()>,
{
    loop {
        let duration = duration_to_next_hour();
        sleep(duration).await;
        callback().await;
    }
}

/// calls the given function every minute
#[cfg(feature = "minute")]
pub fn call_every_minute<F>(callback: F)
where
    F: Fn(),
{
    loop {
        let duration = duration_to_next_minute();
        std::thread::sleep(duration);
        callback();
    }
}

/// calls the given function every minute, takes a `FnMut()` as the argument
#[cfg(feature = "minute")]
pub fn call_every_minute_mut<F>(mut callback: F)
where
    F: FnMut(),
{
    loop {
        let duration = duration_to_next_minute();
        std::thread::sleep(duration);
        callback();
    }
}

/// calls the given async function every minute
#[cfg(feature = "minute")]
pub async fn call_every_minute_async<F, Fut>(callback: F)
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    loop {
        let duration = duration_to_next_minute();
        sleep(duration).await;
        callback().await;
    }
}

/// calls the given async function every minute, takes a `FnMut()` as the argument
#[cfg(feature = "minute")]
pub async fn call_every_minute_async_mut<F, Fut>(mut callback: F)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = ()>,
{
    loop {
        let duration = duration_to_next_minute();
        sleep(duration).await;
        callback().await;
    }
}

/// calls the given function every second
#[cfg(feature = "second")]
pub fn call_every_second<F>(callback: F)
where
    F: Fn(),
{
    loop {
        let duration = duration_to_next_second();
        std::thread::sleep(duration);
        callback();
    }
}

/// calls the given function every second, takes a `FnMut()` as the argument
#[cfg(feature = "second")]
pub fn call_every_second_mut<F>(mut callback: F)
where
    F: FnMut(),
{
    loop {
        let duration = duration_to_next_second();
        std::thread::sleep(duration);
        callback();
    }
}

/// calls the given async function every second
#[cfg(feature = "second")]
pub async fn call_every_second_async<F, Fut>(callback: F)
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    loop {
        let duration = duration_to_next_second();
        sleep(duration).await;
        callback().await;
    }
}

/// calls the given async function every second, takes a `FnMut()` as the argument
#[cfg(feature = "second")]
pub async fn call_every_second_async_mut<F, Fut>(mut callback: F)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = ()>,
{
    loop {
        let duration = duration_to_next_second();
        sleep(duration).await;
        callback().await;
    }
}
