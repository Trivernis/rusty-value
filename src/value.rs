use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Primitive(Primitive),
    Struct(Struct),
    Enum(Enum),
    Map(HashMap<HashablePrimitive, Value>),
    List(Vec<Value>),
    Unit(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enum {
    pub name: String,
    pub variant: String,
    pub value: Box<Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Struct {
    pub name: String,
    pub fields: StructFields,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StructFields {
    Named(HashMap<String, Value>),
    Unnamed(Vec<Value>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Primitive {
    Integer(Integer),
    Float(Float),
    String(String),
    Char(char),
    Bool(bool),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Integer {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    U128(u128),
    I128(i128),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Float {
    F32(f32),
    F64(f64),
}

pub enum HashableValue {
    Primitive(HashablePrimitive),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum HashablePrimitive {
    Integer(Integer),
    String(String),
    Char(char),
    Bool(bool),
}
