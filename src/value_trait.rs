use std::collections::HashMap;

use crate::{Float, HashablePrimitive, HashableValue, Primitive, Value};

pub trait RustyValue {
    fn into_rusty_value(self) -> Value;
}

pub trait HashableRustyValue {
    fn into_hashable_rusty_value(self) -> HashableValue;
}

impl HashableRustyValue for usize {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::Integer(crate::Integer::USize(self)))
    }
}

impl HashableRustyValue for isize {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::Integer(crate::Integer::ISize(self)))
    }
}

impl HashableRustyValue for u8 {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::Integer(crate::Integer::U8(self)))
    }
}

impl HashableRustyValue for i8 {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::Integer(crate::Integer::I8(self)))
    }
}

impl HashableRustyValue for u16 {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::Integer(crate::Integer::U16(self)))
    }
}

impl HashableRustyValue for i16 {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::Integer(crate::Integer::I16(self)))
    }
}

impl HashableRustyValue for u32 {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::Integer(crate::Integer::U32(self)))
    }
}

impl HashableRustyValue for i32 {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::Integer(crate::Integer::I32(self)))
    }
}

impl HashableRustyValue for u64 {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::Integer(crate::Integer::U64(self)))
    }
}

impl HashableRustyValue for i64 {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::Integer(crate::Integer::I64(self)))
    }
}

impl HashableRustyValue for u128 {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::Integer(crate::Integer::U128(self)))
    }
}

impl HashableRustyValue for i128 {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::Integer(crate::Integer::I128(self)))
    }
}

impl HashableRustyValue for String {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::String(self))
    }
}

impl HashableRustyValue for bool {
    fn into_hashable_rusty_value(self) -> HashableValue {
        HashableValue::Primitive(HashablePrimitive::Bool(self))
    }
}

impl RustyValue for HashableValue {
    fn into_rusty_value(self) -> Value {
        match self {
            HashableValue::Primitive(p) => match p {
                HashablePrimitive::Integer(i) => Value::Primitive(Primitive::Integer(i)),
                HashablePrimitive::String(s) => Value::Primitive(Primitive::String(s)),
                HashablePrimitive::Char(c) => Value::Primitive(Primitive::Char(c)),
                HashablePrimitive::Bool(b) => Value::Primitive(Primitive::Bool(b)),
            },
            HashableValue::List(l) => {
                Value::List(l.into_iter().map(|v| v.into_rusty_value()).collect())
            }
        }
    }
}

impl<H: HashableRustyValue> RustyValue for H {
    fn into_rusty_value(self) -> Value {
        self.into_hashable_rusty_value().into_rusty_value()
    }
}

impl RustyValue for f32 {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::Float(Float::F32(self)))
    }
}

impl RustyValue for f64 {
    fn into_rusty_value(self) -> Value {
        Value::Primitive(Primitive::Float(Float::F64(self)))
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

impl<R: RustyValue, H: HashableRustyValue> RustyValue for HashMap<H, R> {
    fn into_rusty_value(self) -> Value {
        let map = self
            .into_iter()
            .map(|(k, v)| (k.into_hashable_rusty_value(), v.into_rusty_value()))
            .collect::<HashMap<_, _>>();

        Value::Map(map)
    }
}
