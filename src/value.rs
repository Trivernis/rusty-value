use std::collections::HashMap;

/// Represents a generic rust value
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Primitive(Primitive),
    Struct(Struct),
    Enum(Enum),
    Map(HashMap<HashableValue, Value>),
    List(Vec<Value>),
    None,
}

/// Represents an enum with a given variant
/// And fields depending on that variant
#[derive(Clone, Debug, PartialEq)]
pub struct Enum {
    pub name: String,
    pub variant: String,
    pub fields: Fields,
}

/// Represents a struct with fields
#[derive(Clone, Debug, PartialEq)]
pub struct Struct {
    pub name: String,
    pub fields: Fields,
}

/// Fields of a struct or an enum that are either named, unnamed or not defined (Unit enums/structs)
#[derive(Clone, Debug, PartialEq)]
pub enum Fields {
    Named(HashMap<String, Value>),
    Unnamed(Vec<Value>),
    Unit,
}

/// A rust primitive value
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Primitive {
    Integer(Integer),
    Float(Float),
    String(String),
    Char(char),
    Bool(bool),
}

/// A primitive integer value
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Integer {
    USize(usize),
    ISize(isize),
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

/// A primitive float value
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Float {
    F32(f32),
    F64(f64),
}

/// A value that can be used as a key inside a hash map
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum HashableValue {
    Primitive(HashablePrimitive),
    List(Vec<HashableValue>),
    None,
}

/// A primitive that can be used as a hash map key
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum HashablePrimitive {
    Integer(Integer),
    String(String),
    Char(char),
    Bool(bool),
}

impl ToString for HashablePrimitive {
    fn to_string(&self) -> String {
        match self {
            HashablePrimitive::Integer(i) => i.to_string(),
            HashablePrimitive::String(s) => s.to_owned(),
            HashablePrimitive::Char(c) => c.to_string(),
            HashablePrimitive::Bool(b) => b.to_string(),
        }
    }
}

impl ToString for Integer {
    fn to_string(&self) -> String {
        match self {
            Integer::USize(n) => n.to_string(),
            Integer::ISize(n) => n.to_string(),
            Integer::U8(n) => n.to_string(),
            Integer::I8(n) => n.to_string(),
            Integer::U16(n) => n.to_string(),
            Integer::I16(n) => n.to_string(),
            Integer::U32(n) => n.to_string(),
            Integer::I32(n) => n.to_string(),
            Integer::U64(n) => n.to_string(),
            Integer::I64(n) => n.to_string(),
            Integer::U128(n) => n.to_string(),
            Integer::I128(n) => n.to_string(),
        }
    }
}

impl ToString for Float {
    fn to_string(&self) -> String {
        match self {
            Float::F32(f) => f.to_string(),
            Float::F64(f) => f.to_string(),
        }
    }
}
