use crate::parser::parser::BlockExpression;
use crate::raw_cstr;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

use std::collections::hash_map::HashMap;

pub type BindingsHashMap = HashMap<String, LLVMValueRef>;

pub struct CodegenBlock {
    pub bindings: BindingsHashMap,
    pub module: LLVMModuleRef,
    pub inner: LLVMBasicBlockRef,
    pub builder: LLVMBuilderRef,
}

impl CodegenBlock {
    pub fn new(
        module: LLVMModuleRef,
        inner: LLVMBasicBlockRef,
        bindings: BindingsHashMap,
    ) -> CodegenBlock {
        let builder = unsafe { LLVMCreateBuilder() };
        unsafe {
            LLVMPositionBuilderAtEnd(builder, inner);
        }

        CodegenBlock {
            bindings,
            module,
            builder,
            inner,
        }
    }
}

pub fn compile_block(
    module: &LLVMModuleRef,
    ast_block: &BlockExpression,
    inherited_bindings: BindingsHashMap,
) -> LLVMBasicBlockRef {
    let context = unsafe { LLVMGetModuleContext(*module) };
    let basic_block = unsafe { LLVMCreateBasicBlockInContext(context, raw_cstr("entry")) };
    let block = CodegenBlock::new(*module, basic_block, inherited_bindings.clone());

    let mut last_value_ref = None;
    for expression in &ast_block.expressions {
        last_value_ref = Some(super::compile_expression_to_instruction(&block, expression));
    }

    // always return last expression result
    if let Some(last_value_ref) = last_value_ref {
        unsafe {
            LLVMBuildRet(block.builder, last_value_ref);
        }
    } else {
        unsafe {
            LLVMBuildRetVoid(block.builder);
        }
    }

    return basic_block;
}
