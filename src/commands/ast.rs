use std::fs::read_to_string;

use crate::args::AstArgs;
use crate::parser::parse_code;
use crate::pretty::ast_pretty_string;
use crate::vm::Vm;


pub fn ast(args: AstArgs) {
    let buffer = read_to_string(args.path).expect("Error reading file");

    let ast = parse_code(&buffer);

    println!("ast has {} nodes", ast.len());

    for node in &ast {
        println!("{}", ast_pretty_string(&node));
    }

    let mut vm = Vm::new();
    vm.compile_ast(&ast);

    println!("{}", vm.to_pretty_string());
}