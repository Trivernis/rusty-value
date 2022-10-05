use rusty_value::{Fields, RustyValue, Value};

#[allow(dead_code)]
#[derive(RustyValue)]
enum TestEnumNamed {
    Named { foo: String },
    Named2 { foo: String, bar: u64 },
}

#[test]
fn it_handles_enums_with_named_fields() {
    let enum_value = TestEnumNamed::Named2 {
        foo: String::from("hello"),
        bar: 12,
    };
    let value = enum_value.into_rusty_value();
    dbg!(&value);

    if let Value::Enum(e) = value {
        assert_eq!(&e.name, "TestEnumNamed");
        assert_eq!(&e.variant, "Named2");

        if let Fields::Named(n) = e.fields {
            assert_eq!(n.len(), 2);
        } else {
            panic!("Enum variant doesn't have named fields")
        }
    } else {
        panic!("Value is not an enum")
    }
}
#[allow(dead_code)]
#[derive(RustyValue)]
enum TestEnumUnnamed {
    Unnamed1(String, u8),
    Unnamed2(u8),
}

#[test]
fn it_handles_enums_with_unamed_fields() {
    let enum_value = TestEnumUnnamed::Unnamed1(String::from("hello"), 12);
    let value = enum_value.into_rusty_value();
    dbg!(&value);

    if let Value::Enum(e) = value {
        assert_eq!(&e.name, "TestEnumUnnamed");
        assert_eq!(&e.variant, "Unnamed1");

        if let Fields::Unnamed(n) = e.fields {
            assert_eq!(n.len(), 2);
        } else {
            panic!("Enum variant doesn't have unnamed fields")
        }
    } else {
        panic!("Value is not an enum")
    }
}

#[allow(dead_code)]
#[derive(RustyValue)]
enum TestEnumUnit {
    Unit1,
    Unit2,
}

#[test]
fn it_handles_unit_enums() {
    let enum_val = TestEnumUnit::Unit1;
    let value = enum_val.into_rusty_value();
    dbg!(&value);

    if let Value::Enum(e) = value {
        assert_eq!(&e.name, "TestEnumUnit");
        assert_eq!(&e.variant, "Unit1");

        if let Fields::Unit = e.fields {
            assert!(true)
        } else {
            panic!("Enum is variant is not a unit")
        }
    } else {
        panic!("Value is not an enum")
    }
}

#[derive(RustyValue)]
enum TestGeneric<R: Clone> {
    CloneVar(R),
}

#[test]
fn it_handles_generic_enums() {
    let enum_val = TestGeneric::CloneVar(String::from("test"));
    let value = enum_val.into_rusty_value();
    dbg!(&value);

    if let Value::Enum(e) = value {
        assert_eq!(&e.name, "TestGeneric");
        assert_eq!(&e.variant, "CloneVar");

        if let Fields::Unnamed(u) = e.fields {
            assert_eq!(u.len(), 1)
        } else {
            panic!("Enum is variant is not an unnamed enum")
        }
    } else {
        panic!("Value is not an enum")
    }
}

#[allow(dead_code)]
#[derive(RustyValue)]
enum TestMixed<R: Clone> {
    CloneVar(R),
    Unit,
    Named { val: R, val2: u8 },
}

#[test]
fn it_handles_mixed_enums() {
    let enum_val = TestMixed::<String>::Unit;
    let value = enum_val.into_rusty_value();
    dbg!(&value);

    if let Value::Enum(e) = value {
        assert_eq!(&e.name, "TestMixed");
        assert_eq!(&e.variant, "Unit");

        if let Fields::Unit = e.fields {
            assert!(true)
        } else {
            panic!("Enum is variant is not a unit")
        }
    } else {
        panic!("Value is not an enum")
    }
}
