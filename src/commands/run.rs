use std::fs::read_to_string;
use std::path::Path;

use crate::args::RunArgs;
use crate::compiler::Compiler;
use crate::parser::Parser;
use crate::pretty::bytecode_to_str;
use crate::types::Action;
use crate::vm::Vm;

pub fn run(args: RunArgs) {
    // let code = std::fs::read_to_string(args.path).unwrap();

    // let ast = parse_code(&code);

    let path = Path::new(&args.path);

    let mut compiler = Compiler::new();
    let mut vm = Vm::new();

    let code = match path.exists() {
        true => read_to_string(path).unwrap(),
        false => args.path.to_string(),
    };

    let ast = Parser::new(&code).parse();
    let res = compiler.compile(&ast);

    println!("consts: {:?}", res.consts);
    println!("bytecode: {}", bytecode_to_str(&res.bytecode));

    for c in res.consts {
        vm.store_const(c);
    }

    vm.create_code_block(&res.bytecode);

    loop {
        let actions = vm.work();

        for action in actions {
            match action {
                Action::Construct{ id } => {
                    log::info!("Construct {}", id);
                }
                Action::Destruct{ id } => {
                    log::info!("Destruct {}", id);
                }
                Action::LoadField{ id, field } => {
                    log::info!("LoadField {} {}", id, field);
                }
                Action::StoreField{ id, field, val } => {
                    log::info!("StoreField {} {} {:?}", id, field, val);
                }
                Action::Call{ id, args } => {
                    log::info!("Call {} {:?}", id, args);
                }
                Action::Import{ path } => {
                    log::info!("Import {}", path);
                }
                Action::Quit => {
                    log::info!("Quit");
                    return;
                }
            }
        }

        vm.clear_actions();
    };

    // for node in ast {
    //     //println!("{:?}", node);

    //     match node {
    //         ASTNode::ObjectDef(o) => {
    //             log::info!("ObjectDef: {}", o.name);

    //             if o.name == "Main" {
    //                 for property in o.properties {
    //                     log::info!("name {}", property.name);

    //                     if property.name == "children" {
    //                         match *property.value {
    //                             ASTNode::ObjectDef(o) => {
    //                                 log::info!("ObjectDef: {}", o.name);

    //                                 let mut title = "Donitsi".to_string();

    //                                 for prop in o.properties {
    //                                     if prop.name == "title" {
    //                                         match *prop.value {
    //                                             ASTNode::LiteralString(s) => {
    //                                                 title = s;
    //                                             }
    //                                             _ => {}
    //                                         }
    //                                     }
    //                                 }

    //                                 if o.name == "Window" {
    //                                     run_window(&title);
    //                                 }
    //                             }
    //                             _ => {}
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         _ => {}
    //     }
    // }
}