use crate::parser::ASTNode;
use crate::vm::ByteCode;

pub fn ast_pretty_string(node: &ASTNode) -> String {
    let mut s = String::new();

    match node {
        ASTNode::StructDef(struct_def) => {
            s += format!("StructDef: {}\n", struct_def.name).as_str();
            for field in struct_def.fields.iter() {
                s += format!("    Field: {}\n", field.name).as_str();
                // s += format!("    value: {}\n", field.typ).as_str();
            }
        },
        ASTNode::TypeDef(type_def) => {
            s += format!("TypeDef: {}\n", type_def.name).as_str();
            for field in type_def.fields.iter() {
                s += format!("    Field: {}\n", field.name).as_str();
            }
        },
        ASTNode::Fun(fn_def) => {
            let body = if fn_def.body.len() == 0 {
                "".to_string()
            } else if fn_def.body.len() < 1 {
                format!("{}", ast_pretty_string(&fn_def.body[0]))
            } else {
                fn_def.body.iter().map(|p: &ASTNode| format!("{}", ast_pretty_string(p))).collect::<Vec<String>>().join("\n")
            };

            s += &format!("() => {{{}}}", body);

            // s += "FnDef: {}\n";

            // println!("Params:");
            // for (index, param) in fn_def.params.iter().enumerate() {
            //     s += format!("[{}] param: {}\n", index, ast_pretty_string(param)).as_str();
            // }

            // println!("Body:");
            // for (index, statement) in fn_def.body.iter().enumerate() {
            //     s += format!("[{}] item: {}\n", index, ast_pretty_string(statement)).as_str();
            // }
        },
        ASTNode::Var(var) => {
            s += format!("{} {}", var.typ, var.name).as_str();
        },
        ASTNode::Assign(assign) => {
            s += &format!("{} = {}", ast_pretty_string(&assign.left), ast_pretty_string(&assign.right));
            // s += "Assign:\n";
            // s += &format!("left: {}", ast_pretty_string(&assign.left));
            // s += &format!("right: {}", ast_pretty_string(&assign.right));
        },
        ASTNode::Ident(ident) => {
            s += ident
        },
        ASTNode::Str(lit) => {
            s += &format!(r#""{}""#, lit);
        },
        ASTNode::Int(lit) => {
            s += &lit.to_string()
        },
        ASTNode::Float(lit) => {
            s += format!("LiteralDecimal: {}\n", lit).as_str();
        },
        ASTNode::LiteralPercent(lit) => {
            s += format!("LiteralPercent: {}\n", lit).as_str();
        },
        ASTNode::StructIns(obj) => {
            s += &format!("{} {{\n", obj.name);
            for prob in &obj.probs {
                s += &format!("  {}: {}\n", prob.name, ast_pretty_string(&prob.value));
            }
            s += "}}\n";

            s += format!("Object: {}\n", obj.name).as_str();
        },
        ASTNode::Call(call) => {
            s += format!("Call: {:?}\n", call.callee).as_str();
        },
        ASTNode::Array(array) => {
            s += &format!("[{}]", array.items.iter().map(|p| format!("{}", ast_pretty_string(p))).collect::<Vec<String>>().join(", "));
        },
        ASTNode::Obj(obj) => {
            s += &format!("{{{}}}", obj.probs.iter()
                .map(|p| format!("{}: {:?}", p.name, ast_pretty_string(&p.value)))
                .collect::<Vec<String>>().join(", "))
        },
        // ASTNode::Ret(ret) => {
        //     match ret.value.clone() {
        //         Some(value) => {
        //             s += &format!("return {}", ast_pretty_string(&value));
        //         },
        //         None => {
        //             s += "return";
        //         }
        //     }
        // },
        _ => {}

        
    }

    s
}

pub fn bytecode_to_str(bytecode: &[ByteCode]) -> String {
    let mut s = String::new();

    for (i, bc) in bytecode.iter().enumerate() {
        s.push_str(&format!("{:04} {:?}\n", i, bc));
    }

    s
}