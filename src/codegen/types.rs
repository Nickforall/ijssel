use crate::parser::Type;
use llvm_sys::core::{LLVMArrayType, LLVMInt32Type};
use llvm_sys::prelude::LLVMTypeRef;

use std::convert::From;

impl From<Type> for LLVMTypeRef {
    fn from(value: Type) -> LLVMTypeRef {
        Self::from(&value)
    }
}

impl From<&Type> for LLVMTypeRef {
    fn from(value: &Type) -> LLVMTypeRef {
        match value {
            Type::Int32 => unsafe { LLVMInt32Type() },
            Type::Unknown => panic!("Could not infer type, got Unknown."),
            Type::ListOf(t, length) => {
                let inner_type = LLVMTypeRef::from(t.as_ref());
                unsafe { LLVMArrayType(inner_type, *length) }
            }
            _ => unimplemented!(),
        }
    }
}
