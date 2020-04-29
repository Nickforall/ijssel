extern crate libc;
extern crate llvm_sys;

use std::ffi::CString;
use std::os::raw::c_char;

mod codegen;
mod parser;

use self::parser::parser::Parser;
use self::parser::tokenizer::Tokenizer;

fn main() {
    println!("Hello, world!");

    let value = &String::from(include_str!("../test.ij"));
    let mut tokenizer = Tokenizer::new(value);
    tokenizer.parse();
    println!("tokens {:?}", tokenizer.tokens);

    let mut parser = Parser::new(&tokenizer.tokens);
    parser.parse_module();
    println!("main module {:?}", parser.module);

    codegen::compile_application(parser.module);
}

fn raw_cstr(string: &str) -> *const c_char {
    let string = CString::new(string).expect("Cstring::new failed");
    string.into_raw()
}
