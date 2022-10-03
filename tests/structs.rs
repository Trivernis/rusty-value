use rusty_value::{RustyValue, StructFields, Value};
use rusty_value_derive::*;

#[derive(RustyValue)]
struct TestStructNamed {
    foo: String,
    bar: u64,
}

#[test]
fn it_handles_named_fields() {
    let test_struct = TestStructNamed {
        foo: String::from("Hello World"),
        bar: 12,
    };
    let value = test_struct.into_rusty_value();
    dbg!(&value);

    if let Value::Struct(s) = value {
        assert_eq!(&s.name, "TestStructNamed");

        if let StructFields::Named(fields) = s.fields {
            assert_eq!(fields.len(), 2);
        } else {
            panic!("Struct wasn't serialized as named struct")
        }
    } else {
        panic!("Struct wasn't serialized as struct");
    }
}

#[derive(RustyValue)]
struct TestStructUnnamed(String, u64);

#[test]
fn it_handles_unnamed_fields() {
    let test_struct = TestStructUnnamed(String::from("Hello World"), 12);
    let value = test_struct.into_rusty_value();
    dbg!(&value);

    if let Value::Struct(s) = value {
        assert_eq!(&s.name, "TestStructUnnamed");

        if let StructFields::Unnamed(fields) = s.fields {
            assert_eq!(fields.len(), 2);
        } else {
            panic!("Struct wasn't serialized as unnamed struct")
        }
    } else {
        panic!("Struct wasn't serialized as struct");
    }
}

#[derive(RustyValue)]
struct TestStructUnit;

#[test]
fn it_handles_unit_structs() {
    let test_struct = TestStructUnit;
    let value = test_struct.into_rusty_value();
    dbg!(&value);

    if let Value::Unit(s) = value {
        assert_eq!(&s, "TestStructUnit");
    } else {
        panic!("Struct wasn't serialized as struct");
    }
}

#[derive(RustyValue)]
struct GenericStruct<T: Clone> {
    field: T,
}

#[test]
fn it_handles_generics() {
    let test_struct = GenericStruct::<u8> { field: 12 };
    let value = test_struct.into_rusty_value();
    dbg!(&value);

    if let Value::Struct(s) = value {
        assert_eq!(&s.name, "GenericStruct");

        if let StructFields::Named(fields) = s.fields {
            assert_eq!(fields.len(), 1);
        } else {
            panic!("Struct wasn't serialized as named struct")
        }
    } else {
        panic!("Struct wasn't serialized as struct");
    }
}
