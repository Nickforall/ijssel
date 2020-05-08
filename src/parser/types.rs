use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// A type that still needs to be inferred by the type-checker. The codegen compiler will crash when encountering an unknown type.
    Unknown,

    /// A signed 32bit integer type, the default type in Ijssel.
    Int32,

    /// A user defined type. This can be a typealias or a struct type.
    User(String),

    ListOf(Box<Type>, u32),

    /// Return type used by extern functions
    Void,
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        use Type::*;

        match value {
            "i32" => Int32,
            identfier => User(String::from(identfier)),
        }
    }
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        Type::from(value.as_str())
    }
}
