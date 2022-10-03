# Rusty Value

This crate adds a `RustyValue` trait that can be derived for all types (except unions)
to create a generic value that represents a rust value.
This can be used to implement serialization of types without having to rely on serde.