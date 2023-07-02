use crate::args::RunArgs;
use crate::vm::Vm;
use crate::parser::ASTNode;
use crate::parser::parse_code;
use crate::window::run_window;


pub fn run(args: RunArgs) {
    // let code = std::fs::read_to_string(args.path).unwrap();

    // let ast = parse_code(&code);

    let mut vm = Vm::new();

    vm.run_file(&args.path);

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