use crate::parser::parser::BlockExpression;
use crate::raw_cstr;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub fn compile_block(module: &LLVMModuleRef, block: &BlockExpression) -> LLVMBasicBlockRef {
    let context = unsafe { LLVMGetModuleContext(*module) };
    let basic_block = unsafe { LLVMCreateBasicBlockInContext(context, raw_cstr("entry")) };
    let builder = unsafe { LLVMCreateBuilder() };

    unsafe {
        LLVMPositionBuilderAtEnd(builder, basic_block);
    }

    let mut last_value_ref = None;
    for expression in &block.expressions {
        last_value_ref = Some(super::compile_expression_to_instruction(
            &builder, expression,
        ));
    }

    // always return last expression result
    if let Some(last_value_ref) = last_value_ref {
        unsafe {
            LLVMBuildRet(builder, last_value_ref);
        }
    } else {
        unsafe {
            LLVMBuildRetVoid(builder);
        }
    }

    return basic_block;
}
