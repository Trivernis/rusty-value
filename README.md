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

use rusty_value::{RustyValue, Value};

#[derive(RustyValue)]
struct MyStruct {
  foo: String,
  bar: u8,
}

fn main() {
  let value = MyStruct {
    foo: "Hello World".to_string(),
    bar: 12,
  }.into_rusty_value();

  match value {
      Value::Primitive(p) => match p {
          rusty_value::Primitive::Integer(_) => println!("is an integer"),
          rusty_value::Primitive::Float(_) => println!("is a float"),
          rusty_value::Primitive::String(_) => println!("is a string"),
          rusty_value::Primitive::OsString(_) => println!("is a os string"),
          rusty_value::Primitive::Char(_) => println!("is a char"),
          rusty_value::Primitive::Bool(_) => println!("is a boolean"),
      },
      Value::Struct(s) => println!("is a struct with name {}", s.name),
      Value::Enum(e) => println!("is an enum with name {} of variant {}", e.name, e.variant),
      Value::Map(_) => println!("is a map"),
      Value::List(_) => println!("is a list"),
      Value::None => println!("is none"),
  }
}
```

Converting a type into a rusty value cannot fail as `rusty_value::RustyValue` is
able to represent any safe rust data type. The trait `RustyValue` is already implemented for
most std types and can therefore be easily derived.