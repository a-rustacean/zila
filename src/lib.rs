#![warn(missing_debug_implementations, missing_docs)]

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
//! zila = { version = "0.1.8", features = ["full"] }
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
//! A basic logger with zila.
//!
//! Make sure you activated the `second` featureof the zill crate on Cargo.toml
//!
//! ```toml
//! zila = { version = "0.1.8", features = ["second"] }
//! ```
//! on your main.rs:
//! ```rust
//! use zila::call_every_second;
//!
//! fn main() {
//!     call_every_second(|| {
//!         println!("Hi");
//!     });
//! }
//! ```
//!
//! More examples can be found [here](https://github.com/a-rustacean/zila/tree/master/examples)

use chrono::{Local, Timelike};
use std::future::Future;
use tokio::time::{sleep, Duration};

/// Returns the duration to next day (00:00:00.000.000.000)
///
/// # Example
///
/// ```rust
/// use zila::duration_to_next_day;
/// use chrono::{Local, TimeLike};
///
/// let duration = duration_to_next_day();
/// std::thread::sleep(duration);
/// let time = Local::new().time();
/// println!("{}h {}m {}s", time.hour(), time.minute(), time.second()); // 0h 0m 0s
/// ```
///
/// *This function requires the following crate features to be activated: `day`*
#[cfg(feature = "day")]
pub fn duration_to_next_day() -> Duration {
    let time = Local::now();
    if time.hour() == 0
        && time.minute() == 0
        && time.second() == 0
        && time.timestamp_subsec_nanos() == 0
    {
        Duration::ZERO
    } else {
        Duration::from_nanos(
            (24 * 60 * 60 * 1000 * 1000 * 1000)
                - (time.hour() as u64 * 60 * 60 * 1000 * 1000 * 1000
                    + time.minute() as u64 * 60 * 1000 * 1000 * 1000
                    + time.second() as u64 * 1000 * 1000 * 1000
                    + time.timestamp_subsec_nanos() as u64),
        )
    }
}

/// Returns the duration to next hour (\_\_:00:00.000.000.000)
///
/// # Example
///
/// ```rust
/// use zila::duration_to_next_hour;
/// use chrono::{Local, TimeLike};
///
/// let duration = duration_to_next_hour();
/// std::thred::sleep(duration).await;
/// let time = Local::new().time();
/// println!("{}m {}s", time.minute(), time.second()); // 0m 0s
/// ```
///
/// *This function requires the following crate features to be activated: `hour`*
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
///
/// # Example
///
/// ```rust
/// use zila::duration_to_next_minute;
/// use chrono::{Local, TimeLike};
///
/// let duration = duration_to_next_minute();
/// sleep(duration).await;
/// let time = Local::new().time();
/// println!("{}s", time.second()); // 0s
/// ```
///
/// *This function requires the following crate features to be activated: `minute`*
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
///
/// # Example
///
/// ```rust
/// use zila::duration_to_next_second;
/// use chrono::{Local, TimeLike};
///
/// let duration = duration_to_next_second();
/// sleep(duration).await;
/// let time = Local::new();
/// println!("{}ms", time.timestamp_subsec_millis()); // 0ms
/// ```
///
/// *This function requires the following crate features to be activated: `second`*
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
///
/// # Example
///
/// using a closure:
///
/// ```rust
/// use zila::call_every_day;
///
/// call_every_day(|| {
///     println!("Hi");
/// });
/// ```
///
/// using a function:
///
/// ```rust
/// use zila::call_every_day;
///
/// fn callback() {
///     println!("Hi");
/// }
///
/// call_every_day(callback);
/// ```
///
/// *This function requires the following crate features to be activated: `day`*
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

/// calls the given function every day, takes a `FnMut` as the argument
///
/// # Example
///
/// ```rust
/// use zila::call_every_day_mut;
///
/// let num = Rc::new(Cell::new(0));
/// call_every_day_mut({
///     let num = num.clone();
///     move || {
///         num.set(num.get() + 1);
///     }
/// });
/// ```
///
/// *This function requires the following crate features to be activated: `day`*
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
///
/// # Example
///
/// using a closure:
///
/// ```rust
/// use zila::call_every_day_async;
///
/// // async closures are unstable
/// call_every_day_async(async || {
///     println!("Hi");
/// }).await;
/// ```
///
/// using a function:
///
/// ```rust
/// use zila::call_every_day_async;
///
/// async fn callback() {
///     println!("Hi");
/// }
///
/// call_every_day_async(callback).await;
/// ```
///
/// *This function requires the following crate features to be activated: `day`*
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

/// calls the given async function every day, takes a `FnMut` as the argument
///
/// # Example
///
/// ```rust
/// use zila::call_every_day_async_mut;
///
/// let num = Rc::new(Cell::new(0));
/// call_every_day_async_mut(move || {
///     let num = num.clone();
///     // async closures are unstable
///     async move || {
///         num.set(num.get() + 1);
///     }
/// }()).await;
/// ```
///
/// *This function requires the following crate features to be activated: `day`*
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
///
/// # Example
///
/// using a closure:
///
/// ```rust
/// use zila::call_every_hour;
///
/// call_every_hour(|| {
///     println!("Hi");
/// });
/// ```
///
/// using a function:
///
/// ```rust
/// use zila::call_every_hour;
///
/// fn callback() {
///     println!("Hi");
/// }
///
/// call_every_hour(callback);
/// ```
///
/// *This function requires the following crate features to be activated: `hour`*
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

/// calls the given function every hour, takes a `FnMut` as the argument
///
/// # Example
///
/// ```rust
/// use zila::call_every_hour_mut;
///
/// let num = Rc::new(Cell::new(0));
/// call_every_hour_mut({
///     let num = num.clone();
///     move || {
///         num.set(num.get() + 1);
///     }
/// });
/// ```
///
/// *This function requires the following crate features to be activated: `hour`*
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
///
/// # Example
///
/// using a closure:
///
/// ```rust
/// use zila::call_every_hour_async;
///
/// // async closures are unstable
/// call_every_hour_async(async || {
///     println!("Hi");
/// }).await;
/// ```
///
/// using a function:
///
/// ```rust
/// use zila::call_every_hour_async;
///
/// async fn callback() {
///     println!("Hi");
/// }
///
/// call_every_hour_async(callback).await;
/// ```
///
/// *This function requires the following crate features to be activated: `hour`*
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

/// calls the given async function every hour, takes a `FnMut` as the argument
///
/// # Example
///
/// ```rust
/// use zila::call_every_hour_async_mut;
///
/// let num = Rc::new(Cell::new(0));
/// call_every_hour_async_mut(move || {
///     let num = num.clone();
///     // async closures are unstable
///     async move || {
///         num.set(num.get() + 1);
///     }
/// }()).await;
/// ```
///
/// *This function requires the following crate features to be activated: `hour`*
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
///
/// # Example
///
/// using a closure:
///
/// ```rust
/// use zila::call_every_minute;
///
/// call_every_minute(|| {
///     println!("Hi");
/// });
/// ```
///
/// using a function:
///
/// ```rust
/// use zila::call_every_minute;
///
/// fn callback() {
///     println!("Hi");
/// }
///
/// call_every_minute(callback);
/// ```
///
/// *This function requires the following crate features to be activated: `minute`*
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

/// calls the given function every minute, takes a `FnMut` as the argument
///
/// # Example
///
/// ```rust
/// use zila::call_every_minute_mut;
///
/// let num = Rc::new(Cell::new(0));
/// call_every_minute_mut({
///     let num = num.clone();
///     move || {
///         num.set(num.get() + 1);
///     }
/// });
/// ```
///
/// *This function requires the following crate features to be activated: `minute`*
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
///
/// # Example
///
/// using a closure:
///
/// ```rust
/// use zila::call_every_minute_async;
///
/// // async closures are unstable
/// call_every_minute_async(async || {
///     println!("Hi");
/// }).await;
/// ```
///
/// using a function:
///
/// ```rust
/// use zila::call_every_minute_async;
///
/// async fn callback() {
///     println!("Hi");
/// }
///
/// call_every_minute_async(callback).await;
/// ```
///
/// *This function requires the following crate features to be activated: `minute`*
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

/// calls the given async function every minute, takes a `FnMut` as the argument
///
/// # Example
///
/// ```rust
/// use zila::call_every_minute_async_mut;
///
/// let num = Rc::new(Cell::new(0));
/// call_every_minute_async_mut(move || {
///     let num = num.clone();
///     // async closures are unstable
///     async move || {
///         num.set(num.get() + 1);
///     }
/// }()).await;
/// ```
///
/// *This function requires the following crate features to be activated: `minute`*
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
///
/// # Example
///
/// using a closure:
///
/// ```rust
/// use zila::call_every_second;
///
/// call_every_second(|| {
///     println!("Hi");
/// });
/// ```
///
/// using a function:
///
/// ```rust
/// use zila::call_every_second;
///
/// fn callback() {
///     println!("Hi");
/// }
///
/// call_every_second(callback);
/// ```
///
/// *This function requires the following crate features to be activated: `second`*
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

/// calls the given function every second, takes a `FnMut` as the argument
///
/// # Example
///
/// ```rust
/// use zila::call_every_second_mut;
///
/// let num = Rc::new(Cell::new(0));
/// call_every_second_mut({
///     let num = num.clone();
///     move || {
///         num.set(num.get() + 1);
///     }
/// });
/// ```
///
/// *This function requires the following crate features to be activated: `second`*
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
///
/// # Example
///
/// using a closure:
///
/// ```rust
/// use zila::call_every_second_async;
///
/// // async closures are unstable
/// call_every_second_async(async || {
///     println!("Hi");
/// }).await;
/// ```
///
/// using a function:
///
/// ```rust
/// use zila::call_every_second_async;
///
/// async fn callback() {
///     println!("Hi");
/// }
///
/// call_every_second_async(callback).await;
/// ```
///
/// *This function requires the following crate features to be activated: `second`*
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

/// calls the given async function every second, takes a `FnMut` as the argument
///
/// # Example
///
/// ```rust
/// use zila::call_every_second_async_mut;
///
/// let num = Rc::new(Cell::new(0));
/// call_every_second_async_mut(move || {
///     let num = num.clone();
///     // async closures are unstable
///     async move || {
///         num.set(num.get() + 1);
///     }
/// }()).await;
/// ```
///
/// *This function requires the following crate features to be activated: `second`*
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

/// calls the function after the specified duration
///
/// # Example
///
/// using a closure:
///
/// ```rust
/// use zila::set_timeout;
///
/// set_timeout(|| {
///     println!("Hi");
/// }, Duration::from_secs(1));
/// ```
///
/// using a function:
///
/// ```rust
/// use zila::set_timeout;
///
/// fn callback() {
///     println!("Hi");
/// }
///
/// set_timeout(callback, Duration::from_secs(1));
/// ```
///
/// *This function requires the following crate features to be activated: `timeout`*
#[cfg(feature = "timeout")]
pub fn set_timeout<F>(callback: F, duration: Duration)
where
    F: Fn(),
{
    std::thread::sleep(duration);
    callback();
}

/// calls the function after the specified duration, takes `FnMut` as the first argument
///
/// # Example
///
/// ```rust
/// use zila::set_timeout_mut;
///
/// let num = Rc::new(Cell::new(0));
/// set_timeout_mut({
///     let num = num.clone();
///     move || {
///         num.set(num.get() + 1);
///     }
/// }, Duration::from_secs(1));
/// ```
///
/// *This function requires the following crate features to be activated: `timeout`*
#[cfg(feature = "timeout")]
pub fn set_timeout_mut<F>(mut callback: F, duration: Duration)
where
    F: FnMut(),
{
    std::thread::sleep(duration);
    callback();
}

/// calls the async function after the specified duration
///
/// # Example
///
/// using a closure:
///
/// ```rust
/// use zila::set_timeout_async;
///
/// // async closures are unstable
/// set_timeout_async(async || {
///     println!("Hi");
/// }, Duration::from_secs(1)).await;
/// ```
///
/// using a function:
///
/// ```rust
/// use zila::set_timeout_async;
///
/// async fn callback() {
///     println!("Hi");
/// }
///
/// set_timeout_async(callback, Duration::from_secs(1)).await;
/// ```
///
/// *This function requires the following crate features to be activated: `timeout`*
#[cfg(feature = "timeout")]
pub async fn set_timeout_async<F, Fut>(callback: F, duration: Duration)
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    sleep(duration).await;
    callback().await;
}

/// calls the async function after the specified duration, takes `FnMut` as the first argument
///
/// # Example
///
/// ```rust
/// use zila::set_timeout_async_mut;
///
/// let num = Rc::new(Cell::new(0));
/// set_timeout_async_mut(move || {
///     let num = num.clone();
///     // async closures are unstable
///     async move || {
///         num.set(num.get() + 1);
///     }
/// }(), Duration::from_secs(1)).await;
/// ```
///
/// *This function requires the following crate features to be activated: `timeout`*
#[cfg(feature = "timeout")]
pub async fn set_timeout_async_mut<F, Fut>(mut callback: F, duration: Duration)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = ()>,
{
    sleep(duration).await;
    callback().await;
}

/// calls the function in the specified intervals
///
/// # Example
///
/// using a closure:
///
/// ```rust
/// use zila::set_interval;
///
/// set_interval(|| {
///     println!("Hi");
/// }, Duration::from_secs(1));
/// ```
///
/// using a function:
///
/// ```rust
/// use zila::set_interval;
///
/// fn callback() {
///     println!("Hi");
/// }
///
/// set_interval(callback, Duration::from_secs(1));
/// ```
/// *This function requires the following crate features to be activated: `interval`*
#[cfg(feature = "interval")]
pub fn set_interval<F>(callback: F, duration: Duration)
where
    F: Fn(),
{
    loop {
        std::thread::sleep(duration);
        callback();
    }
}

/// calls the function in the specified intervals, takes `FnMut` as the first argument
///
/// # Example
///
/// ```rust
/// use zila::set_interval_mut;
///
/// let num = Rc::new(Cell::new(0));
/// set_interval_mut({
///     let num = num.clone();
///     move || {
///         num.set(num.get() + 1);
///     }
/// }, Duration::from_secs(1));
/// ```
///
/// *This function requires the following crate features to be activated: `interval`*
#[cfg(feature = "timeout")]
pub fn set_interval_mut<F>(mut callback: F, duration: Duration)
where
    F: FnMut(),
{
    loop {
        std::thread::sleep(duration);
        callback();
    }
}

/// calls the async function in the specified intervals
///
/// # Example
///
/// using a closure:
///
/// ```rust
/// use zila::set_interval_async;
///
/// // async closures are unstable
/// set_interval_async(async || {
///     println!("Hi");
/// }, Duration::from_secs(1)).await;
/// ```
///
/// using a function:
///
/// ```rust
/// use zila::set_interval_async;
///
/// async fn callback() {
///     println!("Hi");
/// }
///
/// set_interval_async(callback, Duration::from_secs(1)).await;
/// ```
/// *This function requires the following crate features to be activated: `interval`*
#[cfg(feature = "interval")]
pub async fn set_interval_async<F, Fut>(callback: F, duration: Duration)
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    loop {
        sleep(duration).await;
        callback().await;
    }
}

/// calls the async function in the specified intervals, takes `FnMut` as the first argument
///
/// # Example
///
/// ```rust
/// use zila::set_interval_async_mut;
///
/// let num = Rc::new(Cell::new(0));
/// set_interval_async_mut(move || {
///     let num = num.clone();
///     // async closures are unstable
///     async move || {
///         num.set(num.get() + 1);
///     }
/// }(), Duration::from_secs(1)).await;
/// ```
///
/// *This function requires the following crate features to be activated: `interval`*
#[cfg(feature = "timeout")]
pub async fn set_interval_async_mut<F, Fut>(mut callback: F, duration: Duration)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = ()>,
{
    loop {
        sleep(duration).await;
        callback().await;
    }
}
