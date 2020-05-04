pub mod binary;
pub mod block;
pub mod call;
pub mod constants;
pub mod function;
pub mod module;
pub mod variable;

pub use block::CodegenBlock;

use crate::parser::Expression;
use crate::parser::Module;
use llvm_sys::prelude::*;

pub fn compile_application(module: Module) -> LLVMModuleRef {
    self::module::compile_module(&module)
}

pub fn compile_expression_to_instruction(
    containing_block: &CodegenBlock,
    expression: &Expression,
) -> LLVMValueRef {
    use crate::parser::Expression::*;

    match expression {
        Binary(expr) => binary::compile_binary_expression_to_instruction(containing_block, expr),
        NumberLiteral(literal) => constants::compile_int64_constant(literal),
        Variable(expr) => variable::compile_variable_expression(containing_block, expr),
        Call(expr) => call::compile_call(containing_block, expr),
        _ => unimplemented!(),
    }
}
