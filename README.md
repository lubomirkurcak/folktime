# folktime

[![Build status](https://github.com/lubomirkurcak/folktime/workflows/Test/badge.svg)](https://github.com/lubomirkurcak/folktime/actions)
[![Crates.io](https://img.shields.io/crates/v/folktime.svg)](https://crates.io/crates/folktime)

Tiny library for approximate formatting of [std::time::Duration](https://doc.rust-lang.org/stable/core/time/struct.Duration.html) in a human-friendly way.

If you are looking for a full precision human readable format, take a look at [humantime](https://crates.io/crates/humantime).

### Usage

```rust
use std::time::Duration;
use folktime::Folktime;

let a = Folktime::duration(Duration::from_secs(5));
assert_eq!(format!("{}", a), "5.00s");
```

### Precision

Formatting only shows the most significant digits:

```rust
use std::time::Duration;
use folktime::Folktime;

let a = Folktime::duration(Duration::new(0, 123_456_789));
let b = Folktime::duration(Duration::new(1, 123_456_789));
let c = Folktime::duration(Duration::new(12, 123_456_789));
let d = Folktime::duration(Duration::new(123, 123_456_789));

assert_eq!(format!("{}", a), "123ms");
assert_eq!(format!("{}", b), "1.12s");
assert_eq!(format!("{}", c), "12.1s");
assert_eq!(format!("{}", d), "2.05m");
```

### Formatting styles

There are several styles for formatting:

```rust
use std::time::Duration;
use folktime::Folktime;
use folktime::duration::Style;

let a = Folktime::duration(Duration::new(0, 12_056_999));
let b = Folktime::duration(Duration::new(0, 12_056_999)).with_style(Style::OneUnitWhole);
let c = Folktime::duration(Duration::new(0, 12_056_999)).with_style(Style::TwoUnitsWhole);

assert_eq!(format!("{}", a), "12.0ms");
assert_eq!(format!("{}", b), "12ms");
assert_eq!(format!("{}", c), "12ms 56us");
```

Here's a comparison of styles:

| Duration   | [`Style::OneUnitFrac`](https://docs.rs/folktime/latest/folktime/duration/enum.Style.html#variant.OneUnitFrac) | [`Style::OneUnitWhole`](https://docs.rs/folktime/latest/folktime/duration/enum.Style.html#variant.OneUnitWhole) | [`Style::TwoUnitsWhole`](https://docs.rs/folktime/latest/folktime/duration/enum.Style.html#variant.TwoUnitsWhole) |
|-----------:|---------------------:|----------------------:|-----------------------:|
| 0s         | `0.00s`              | `0s`                  | `0s 0ms`               |
| 0.123456s  | `123ms`              | `123ms`               | `123ms 456us`          |
| 1.123456s  | `1.12s`              | `1s`                  | `1s 123ms`             |
| 12.12345s  | `12.1s`              | `12s`                 | `12s 123ms`            |
| 123.1234s  | `2.05m`              | `2m`                  | `2m 3s`                |
| 86400s     | `1.00d`              | `1d`                  | `1d 0h`                |
| 12345678s  | `2.04w`              | `2w`                  | `2w 0d`                |
| 123456789s | `4.69mo`             | `4mo`                 | `4mo 21d`              |
| max        | `584Gy`              | `584Gy`               | `584Gy 4mo`            |

