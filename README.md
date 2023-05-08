# Zila

A library for calling function on certain events with
the Rust programming language. It is:

* **Fast**: Zila uses tokio and Chrono under the hood thats
  make it very fast

* **Reliable**: Zila leverages Rust's ownership, type system, and
  concurrency model to reduce bugs and ensure thread safety.

* **Scalable**: Zila has a minimal footprint, and handles backpressure
  and cancellation naturally.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/zila.svg
[crates-url]: https://crates.io/crates/zila
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/a-rustacean/zila/blob/master/LICENSE
[actions-badge]: https://github.com/a-rustacean/zila/workflows/CI/badge.svg
[actions-url]: https://github.com/a-rustacean/zila/actions?query=workflow%3ACI+branch%3Amaster


## Overview

Zila is a fast and reliable library for performing tasks on
certian events. It propvides both syncronuos and asyncronuos functions
to make writing Rust code esier. At a high level, it provides a few major
functions:

* [duration_to_next_hour][duration]
* [call_every_hour][every]
* [call_every_hour_mut][every_mut]
* [call_every_hour_async][every_async]
* [call_every_hour_async_mut][every_async_mut]

[duration]: https://docs.rs/zila/0.1.2/fn.duration_to_next_hour.html
[every]: https://docs.rs/zila/0.1.2/fn.call_every_hour.html
[every_mut]: https://docs.rs/zila/0.1.2/fn.call_every_hour_mut.html
[every_async]: https://docs.rs/zila/0.1.2/fn.call_every_hour_async.html
[every_async_mut]: https://docs.rs/zila/0.1.2/fn.call_every_hour_async_mut.html

## Example

A basic logger with zila.

Make sure you activated the `second` features of the zila crate on Cargo.toml:

```toml
[dependencies]
zila = { version = "0.1.2", features = ["second"] }
```
Then, on your main.rs:

```rust,no_run
use zila::call_every_hour;

fn main() {
    call_every_hour(|| {
        println!("Hi");
    })
}
```

More examples can be found [here][examples].

[examples]: https://github.com/a-rustacean/zila/tree/master/examples

## Getting Help

First, see if the answer to your question can be found in the [API documentation].
If the answer is not there, you can raise an [issue] if you think there is a problem
with zila

[API documentation]: https://docs.rs/zila/latest/zila
[issue]: https://github.com/a-rustacean/zila/issues/new??labels=A-zila%2C+C-bug&template=bug_report.md

## Contributing

:balloon: Thanks for your help improving the project! We are so happy to have
you! We have a [contributing guide][guide] to help you get involved in the zila
project.

[guide]: https://github.com/a-rustacean/zila/blob/master/CONTRIBUTING.md

<!--
When updating this, also update:
- CONTRIBUTING.md
- README.md
-->

## Release schedule

zila doesn't follow a fixed release schedule, but we typically make one to two
new minor releases each month. We make patch releases for bugfixes as necessary.

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/a-rustacean/zila/blob/master/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in zila by you, shall be licensed as MIT, without any additional
terms or conditions.
