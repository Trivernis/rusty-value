use crate::*;
use serde_json::Number;
use std::string::ToString;

/// Options for how to represent certain rust types
/// as JSON
#[derive(Clone, Debug, RustyValue, Default)]
pub struct IntoJsonOptions {
    pub enum_repr: EnumRepr,
}

/// Controls how enums should be represented
/// This works similarly to serde except that internal tagging isn't supported
/// except that internal tagging isn't supported
#[derive(Clone, Debug, RustyValue)]
pub enum EnumRepr {
    Untagged,
    ExternallyTagged,
    AdjacentlyTagged {
        type_field: String,
        value_field: String,
    },
}

impl Default for EnumRepr {
    fn default() -> Self {
        Self::ExternallyTagged
    }
}

/// Trait to convert a value into a json value
pub trait IntoJson {
    /// Converts the value into a json value with default options
    fn into_json(self) -> serde_json::Value;
    /// Converts the value into a json value with the given options
    fn into_json_with_options(self, options: &IntoJsonOptions) -> serde_json::Value;
}

trait RustyIntoJson {
    /// Converts the value into a json value with default options
    fn into_json(self) -> serde_json::Value;
    /// Converts the value into a json value with the given options
    fn into_json_with_options(self, options: &IntoJsonOptions) -> serde_json::Value;
}

impl RustyIntoJson for crate::Value {
    #[inline]
    fn into_json(self) -> serde_json::Value {
        self.into_json_with_options(&IntoJsonOptions::default())
    }

    fn into_json_with_options(self, opt: &IntoJsonOptions) -> serde_json::Value {
        match self {
            crate::Value::Primitive(p) => p.into_json_with_options(opt),
            crate::Value::Struct(s) => s.into_json_with_options(opt),
            crate::Value::Enum(e) => e.into_json_with_options(opt),
            crate::Value::Map(m) => serde_json::Value::Object(
                m.into_iter()
                    .map(|(k, v)| (hashable_to_string(k), v.into_json_with_options(opt)))
                    .collect(),
            ),
            crate::Value::List(l) => serde_json::Value::Array(
                l.into_iter()
                    .map(|v| v.into_json_with_options(opt))
                    .collect(),
            ),
            crate::Value::None => serde_json::Value::Null,
        }
    }
}

impl IntoJson for Primitive {
    fn into_json(self) -> serde_json::Value {
        match self {
            Primitive::Integer(i) => match i {
                crate::Integer::USize(n) => serde_json::Value::Number(n.into()),
                crate::Integer::ISize(n) => serde_json::Value::Number(n.into()),
                crate::Integer::U8(n) => serde_json::Value::Number(n.into()),
                crate::Integer::I8(n) => serde_json::Value::Number(n.into()),
                crate::Integer::U16(n) => serde_json::Value::Number(n.into()),
                crate::Integer::I16(n) => serde_json::Value::Number(n.into()),
                crate::Integer::U32(n) => serde_json::Value::Number(n.into()),
                crate::Integer::I32(n) => serde_json::Value::Number(n.into()),
                crate::Integer::U64(n) => serde_json::Value::Number(n.into()),
                crate::Integer::I64(n) => serde_json::Value::Number(n.into()),
                crate::Integer::U128(n) => serde_json::Value::Array(vec![
                    ((n >> 64) as u64).into(),
                    ((n & 0xFFFFFFFFFFFFFFFF) as u64).into(),
                ]),
                crate::Integer::I128(n) => serde_json::Value::Array(vec![
                    ((n >> 64) as i64).into(),
                    ((n & 0xFFFFFFFFFFFFFFFF) as u64).into(),
                ]),
            },
            Primitive::Float(f) => match f {
                crate::Float::F32(f) => Number::from_f64(f as f64)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null),
                crate::Float::F64(f) => Number::from_f64(f)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null),
            },
            Primitive::String(s) => serde_json::Value::String(s),
            Primitive::OsString(o) => serde_json::Value::String(o.to_string_lossy().into_owned()),
            Primitive::Char(c) => serde_json::Value::String(c.to_string()),
            Primitive::Bool(b) => serde_json::Value::Bool(b),
        }
    }

    #[inline]
    fn into_json_with_options(self, _options: &IntoJsonOptions) -> serde_json::Value {
        self.into_json()
    }
}

impl IntoJson for Enum {
    #[inline]
    fn into_json(self) -> serde_json::Value {
        self.into_json_with_options(&IntoJsonOptions::default())
    }
    fn into_json_with_options(self, opt: &IntoJsonOptions) -> serde_json::Value {
        let value = match self.fields {
            crate::Fields::Named(n) => serde_json::Value::Object(
                n.into_iter()
                    .map(|(k, v)| (k, v.into_json_with_options(opt)))
                    .collect(),
            ),
            crate::Fields::Unnamed(mut u) => {
                if u.len() == 1 {
                    u.remove(0).into_json_with_options(opt)
                } else {
                    serde_json::Value::Array(
                        u.into_iter()
                            .map(|v| v.into_json_with_options(opt))
                            .collect(),
                    )
                }
            }
            crate::Fields::Unit => serde_json::Value::String(self.variant.clone()),
        };
        match &opt.enum_repr {
            EnumRepr::Untagged => value,
            EnumRepr::ExternallyTagged => {
                serde_json::Value::Object([(self.variant, value)].into_iter().collect())
            }
            EnumRepr::AdjacentlyTagged {
                type_field,
                value_field,
            } => serde_json::Value::Object(
                [
                    (
                        type_field.to_owned(),
                        serde_json::Value::String(self.variant),
                    ),
                    (value_field.to_owned(), value),
                ]
                .into_iter()
                .collect(),
            ),
        }
    }
}

impl IntoJson for Struct {
    #[inline]
    fn into_json(self) -> serde_json::Value {
        self.into_json_with_options(&IntoJsonOptions::default())
    }
    fn into_json_with_options(self, opt: &IntoJsonOptions) -> serde_json::Value {
        match self.fields {
            crate::Fields::Named(n) => serde_json::Value::Object(
                n.into_iter()
                    .map(|(k, v)| (k, v.into_json_with_options(opt)))
                    .collect(),
            ),
            crate::Fields::Unnamed(mut u) => {
                if u.len() == 1 {
                    u.remove(0).into_json_with_options(opt)
                } else {
                    serde_json::Value::Array(
                        u.into_iter()
                            .map(|v| v.into_json_with_options(opt))
                            .collect(),
                    )
                }
            }
            crate::Fields::Unit => serde_json::Value::String(self.name),
        }
    }
}

impl<R: RustyValue> IntoJson for R {
    #[inline]
    fn into_json(self) -> serde_json::Value {
        self.into_json_with_options(&IntoJsonOptions::default())
    }

    #[inline]
    fn into_json_with_options(self, opt: &IntoJsonOptions) -> serde_json::Value {
        self.into_rusty_value().into_json_with_options(opt)
    }
}

fn hashable_to_string(hashable: HashableValue) -> String {
    match hashable {
        HashableValue::Primitive(p) => p.to_string(),
        HashableValue::List(l) => l
            .into_iter()
            .map(hashable_to_string)
            .collect::<Vec<_>>()
            .join(","),
        HashableValue::None => String::new(),
    }
}

#[cfg(test)]
mod test {
    #![allow(unused)]
    use serde_json::json;

    use crate as rusty_value;
    use crate::into_json::IntoJsonOptions;
    use crate::*;

    use super::IntoJson;

    #[test]
    fn it_serializes_primitives() {
        assert_eq!(u8::MAX.into_json(), json!(u8::MAX));
        assert_eq!(i8::MIN.into_json(), json!(i8::MIN));
        assert_eq!(u16::MAX.into_json(), json!(u16::MAX));
        assert_eq!(i16::MIN.into_json(), json!(i16::MIN));
        assert_eq!(u32::MAX.into_json(), json!(u32::MAX));
        assert_eq!(i32::MIN.into_json(), json!(i32::MIN));
        assert_eq!(u64::MAX.into_json(), json!(u64::MAX));
        assert_eq!(i64::MIN.into_json(), json!(i64::MIN));
        assert_eq!(u128::MAX.into_json(), json!([u64::MAX, u64::MAX]));
        assert_eq!(i128::MIN.into_json(), json!([i64::MIN, 0]));
    }

    #[derive(Default, RustyValue)]
    struct TestStruct {
        foo: String,
        bar: u8,
    }

    #[test]
    fn it_serializes_structs() {
        let val = TestStruct::default();
        let value = val.into_json();

        assert!(value.is_object());
    }

    #[derive(RustyValue)]
    enum TestEnum {
        Foo,
        Bar(String),
    }

    #[test]
    fn it_serializes_unit_enums_untagged() {
        let val = TestEnum::Foo;
        let value = val.into_json_with_options(&IntoJsonOptions {
            enum_repr: rusty_value::into_json::EnumRepr::Untagged,
        });

        assert!(value.is_string());
        assert_eq!(value.as_str(), Some("Foo"))
    }

    #[test]
    fn it_serializes_struct_enums_adjacently_tagged() {
        let val = TestEnum::Bar(String::new());
        let value = val.into_json_with_options(&IntoJsonOptions {
            enum_repr: rusty_value::into_json::EnumRepr::AdjacentlyTagged {
                type_field: "type".into(),
                value_field: "value".into(),
            },
        });
        println!("{}", value.to_string());

        assert!(value.is_object());
        assert_eq!(value.get("type").unwrap().as_str(), Some("Bar"));
        assert!(value.get("value").unwrap().is_string());
    }

    #[test]
    fn it_serializes_struct_enums_untagged() {
        let val = TestEnum::Bar(String::new());
        let value = val.into_json_with_options(&IntoJsonOptions {
            enum_repr: rusty_value::into_json::EnumRepr::Untagged,
        });

        assert!(value.is_string());
        assert_eq!(value.as_str(), Some(""));
    }

    #[test]
    fn it_serializes_struct_enums_externally_tagged() {
        let val = TestEnum::Bar(String::new());
        let value = val.into_json_with_options(&IntoJsonOptions {
            enum_repr: rusty_value::into_json::EnumRepr::ExternallyTagged,
        });

        assert!(value.is_object());
        assert!(value.get("Bar").unwrap().is_string());
    }
}
