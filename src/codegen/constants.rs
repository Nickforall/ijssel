use crate::parser::parser::NumberLiteralExpression;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub fn compile_int64_constant(number: &NumberLiteralExpression) -> LLVMValueRef {
    unsafe { LLVMConstInt(LLVMInt64Type(), number.number as u64, 0) }
}
