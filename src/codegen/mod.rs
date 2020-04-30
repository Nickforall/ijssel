pub mod binary;
pub mod block;
pub mod constants;
pub mod function;
pub mod module;

use crate::parser::parser::Expression;
use crate::parser::parser::Module;
use llvm_sys::prelude::*;

pub fn compile_application(module: Module) -> LLVMModuleRef {
    self::module::compile_module(&module)
}

pub fn compile_expression_to_instruction(
    builder: &LLVMBuilderRef,
    expression: &Expression,
) -> LLVMValueRef {
    use crate::parser::parser::Expression::*;

    match expression {
        Binary(expr) => binary::compile_binary_expression_to_instruction(builder, expr),
        NumberLiteral(literal) => constants::compile_int64_constant(literal),
        _ => unimplemented!(),
    }
}
