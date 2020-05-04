use crate::parser::VariableExpression;
use llvm_sys::prelude::*;

pub fn compile_variable_expression(
    block: &super::CodegenBlock,
    expression: &VariableExpression,
) -> LLVMValueRef {
    if let Some(value) = block.bindings.get(&expression.binding) {
        *value
    } else {
        panic!("Unknown variable {}", expression.binding);
    }
}
