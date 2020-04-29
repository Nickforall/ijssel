use crate::parser::parser::BinaryExpression;
use crate::parser::tokenizer::BinaryOperator;
use crate::raw_cstr;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub fn compile_binary_expression_to_instruction(
    builder: &LLVMBuilderRef,
    expression: &BinaryExpression,
) -> LLVMValueRef {
    let right = super::compile_expression_to_instruction(builder, &expression.right);
    let left = super::compile_expression_to_instruction(builder, &expression.left);

    match expression.operator {
        BinaryOperator::Add => unsafe {
            LLVMBuildAdd(*builder, left, right, raw_cstr("__ijssel_tmp"))
        },
        BinaryOperator::Mul => unsafe {
            LLVMBuildMul(*builder, left, right, raw_cstr("__ijssel_tmp"))
        },
        BinaryOperator::Sub => unsafe {
            LLVMBuildSub(*builder, left, right, raw_cstr("__ijssel_tmp"))
        },
    }
}
