use crate::parser::Type;
use llvm_sys::core::LLVMInt32Type;
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
            _ => unimplemented!(),
        }
    }
}
