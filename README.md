# Rusty Value
[![](https://img.shields.io/crates/v/rusty-value?style=for-the-badge)](https://crates.io/crates/rusty-value)
[![](https://img.shields.io/docsrs/rusty-value/latest?style=for-the-badge)](https://docs.rs/rusty-value/)

This crate adds a `RustyValue` trait that can be derived for all types (except unions)
to create a generic value that represents a rust value.
This can be used to implement serialization of types without having to rely on serde.

## Usage

The trait `RustyValue` allows one to create a `rusty_value::Value` for any
type that implements it. This trait can be derived if the `derive` **feature** is enabled.

```rust

use rusty_value::RustyValue;

#[derive(RustyValue)]
struct MyStruct {
  foo: String,
  bar: u8,
}

fn main() {
  MyStruct {
    foo: "Hello World".to_string(),
    bar: 12,
  }.into_rusty_value();
}
```

Converting a type into a rusty value cannot fail as `rusty_value::RustyValue` is
able to represent any safe rust data type. The trait `RustyValue` is already implemented for
most std types and can therefore be easily derived.