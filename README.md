[![License: CC0](https://img.shields.io/github/license/ringostarr80/rust-length.svg)](https://creativecommons.org/publicdomain/zero/1.0/legalcode)
[![Crates.io](https://img.shields.io/crates/v/length.svg)](https://crates.io/crates/length)
[![Documentation](https://docs.rs/color_processing/badge.svg)](https://docs.rs/length/)
![Rust](https://github.com/ringostarr80/rust-length/workflows/Rust/badge.svg)
[![codecov](https://codecov.io/gh/ringostarr80/rust-length/graph/badge.svg?token=SB4B08D01O)](https://codecov.io/gh/ringostarr80/rust-length)

# Purpose

This rust library is intended to do some processing of length values.  
It can parse strings with different units (m, km, mi, ft, ...) and converting them into other units.

## Usage

To use `length`, first add this to your `Cargo.toml`:

```toml
[dependencies]
length = "0.2"
```

Next, add this to your crate:

```rust
extern crate length;

use length::{Length, Unit, MetricUnit::*};

fn main() {
    let five_meter = Length::new_string("5m").unwrap();
    assert_eq!("5m", five_meter.to_original_string());
    assert_eq!(5.0, five_meter.value);
    assert_eq!(Unit::Metric(Meter), five_meter.unit);

    let fivehundred_centimeter = five_meter.to(Unit::Metric(Centimeter));
    assert_eq!(500.0, fivehundred_centimer.value);
    assert_eq!(Unit::Metric(Centimeter), fivehundred_centimer.unit);

    // ...
}
```

# Documentation

For the latest documentation and examples, please go to [https://docs.rs/length](https://docs.rs/length).

# Miscellaneous

If you have suggestions or found an error, feel free to open an [issue](https://github.com/ringostarr80/rust-length/issues) or create a [pull request](https://github.com/ringostarr80/rust-length/pulls) on github.
