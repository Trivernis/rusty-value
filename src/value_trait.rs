use std::collections::HashMap;

use crate::{HashablePrimitive, Primitive, Value};

pub trait RustyValue {
    fn into_rusty_value(self) -> Value;
}

impl RustyValue for u8 {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::Integer(crate::Integer::U8(self)))
    }
}

impl RustyValue for i8 {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::Integer(crate::Integer::I8(self)))
    }
}

impl RustyValue for u16 {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::Integer(crate::Integer::U16(self)))
    }
}

impl RustyValue for i16 {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::Integer(crate::Integer::I16(self)))
    }
}

impl RustyValue for u32 {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::Integer(crate::Integer::U32(self)))
    }
}

impl RustyValue for i32 {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::Integer(crate::Integer::I32(self)))
    }
}

impl RustyValue for u64 {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::Integer(crate::Integer::U64(self)))
    }
}

impl RustyValue for i64 {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::Integer(crate::Integer::I64(self)))
    }
}

impl RustyValue for u128 {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::Integer(crate::Integer::U128(self)))
    }
}

impl RustyValue for i128 {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::Integer(crate::Integer::I128(self)))
    }
}

impl RustyValue for String {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::String(self))
    }
}

impl RustyValue for bool {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::Bool(self))
    }
}

impl<R: RustyValue> RustyValue for Vec<R> {
    fn into_rusty_value(self) -> Value {
        let value_vec = self
            .into_iter()
            .map(|v| v.into_rusty_value())
            .collect::<Vec<_>>();

        Value::List(value_vec)
    }
}

impl<R: RustyValue> RustyValue for HashMap<String, R> {
    fn into_rusty_value(self) -> Value {
        let map = self
            .into_iter()
            .map(|(k, v)| (HashablePrimitive::String(k), v.into_rusty_value()))
            .collect::<HashMap<_, _>>();

        Value::Map(map)
    }
}
