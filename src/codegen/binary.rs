use crate::parser::tokenizer::BinaryOperator;
use crate::parser::BinaryExpression;
use crate::raw_cstr;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub fn compile_binary_expression_to_instruction(
    containing_block: &super::CodegenBlock,
    expression: &BinaryExpression,
) -> LLVMValueRef {
    let right = super::compile_expression_to_instruction(containing_block, &expression.right);
    let left = super::compile_expression_to_instruction(containing_block, &expression.left);

    match expression.operator {
        BinaryOperator::Add => unsafe {
            LLVMBuildAdd(
                containing_block.builder,
                left,
                right,
                raw_cstr("__ijssel_tmp"),
            )
        },
        BinaryOperator::Mul => unsafe {
            LLVMBuildMul(
                containing_block.builder,
                left,
                right,
                raw_cstr("__ijssel_tmp"),
            )
        },
        BinaryOperator::Sub => unsafe {
            LLVMBuildSub(
                containing_block.builder,
                left,
                right,
                raw_cstr("__ijssel_tmp"),
            )
        },
    }
}
