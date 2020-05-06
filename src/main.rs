extern crate clap;
extern crate libc;
extern crate llvm_sys;

use clap::{App, Arg};
use std::ffi::CString;
use std::fs;
use std::os::raw::c_char;
use std::path;
use std::time::Instant;

mod codegen;
mod machine;
mod modules;
mod parser;
mod type_system;

use self::parser::parser::Parser;
use self::parser::tokenizer::Tokenizer;

fn main() {
    let matches = App::new("ijssel Compiler")
        .version("0.1.0")
        .author("Nick Vernij. <hello@nickforall.nl>")
        .about("Compiler for the ijssel language")
        .arg(
            Arg::with_name("source")
                .required(true)
                .help("An ijssel source file")
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("output")
                .required(false)
                .short("o")
                .long("output")
                .help("Output to write object or assembly to. If omitted this will default to the input file name with an extension corresponding the `type`.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file-type")
                .required(false)
                .long("type")
                .takes_value(true)
                .possible_values(&["object", "asm", "tokens", "ast", "bc", "ll"])
                .help("Output format. Tokens, AST and LL are printed to stdout.")
                .default_value("object"),
        )
        .arg(Arg::with_name("debug").long("debug").help("Whether to perform optimisations"))
        .get_matches();

    let now = Instant::now();

    let file = matches.value_of("source").expect("File is required");
    let debug = matches.is_present("debug");

    if let Ok(string) = fs::read_to_string(file) {
        let mut tokenizer = Tokenizer::new(&string);
        tokenizer.parse();

        if matches.value_of("file-type").unwrap_or("object") == "tokens" {
            println!("{:#?}", tokenizer.tokens);
            return;
        }

        let mut parser = Parser::new(&tokenizer.tokens);
        parser.parse_module();

        if matches.value_of("file-type").unwrap_or("object") == "ast" {
            println!("{:#?}", parser.module);
            return;
        }

        let module = modules::Module::from_ast_module(parser.module);

        let extension = match matches.value_of("file-type").unwrap_or("object") {
            "asm" => "s",
            "bc" => "bc",
            "object" | _ => "o",
        };

        let default_output_path = path::Path::new(file).with_extension(extension);
        let output = matches.value_of("output").unwrap_or(
            default_output_path
                .to_str()
                .expect("Invalid default output"),
        );

        let llvm_module = codegen::compile_application(module);

        if matches.value_of("file-type").unwrap_or("object") == "ll" {
            unsafe { llvm_sys::core::LLVMDumpModule(llvm_module) };
            return;
        }

        if matches.value_of("file-type").unwrap_or("object") == "bc" {
            unsafe {
                llvm_sys::bit_writer::LLVMWriteBitcodeToFile(llvm_module, raw_cstr(output));
            }
            return;
        }

        let file_type = match matches.value_of("file-type").unwrap_or("object") {
            "asm" => llvm_sys::target_machine::LLVMCodeGenFileType::LLVMAssemblyFile,
            "object" | _ => llvm_sys::target_machine::LLVMCodeGenFileType::LLVMObjectFile,
        };

        machine::compile_to_current_target(&llvm_module, output, file_type, debug);

        let benchmark = now.elapsed().as_millis();
        println!("Compiled {} in {}ms!", output, benchmark)
    } else {
        println!("File {} not found", file)
    }
}

fn raw_cstr(string: &str) -> *const c_char {
    let string = CString::new(string).expect("Cstring::new failed");
    string.into_raw()
}
