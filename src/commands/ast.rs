use std::fs::read_to_string;

use crate::args::AstArgs;
use crate::pretty::ast_pretty_string;


pub fn ast(args: AstArgs) {
    let buffer = read_to_string(args.path).expect("Error reading file");

    // let ast = parse_code(&buffer);

    // println!("ast has {} nodes", ast.len());

    // for node in &ast {
    //     println!("{}", ast_pretty_string(&node));
    // }
}