use crate::parser::ASTNode;

pub fn ast_to_pretty_string(node: &ASTNode) -> String {
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
        ASTNode::FnDef(fn_def) => {
            println!("FnDef");

            println!("Params:");
            for (index, param) in fn_def.params.iter().enumerate() {
                s += format!("[{}] param: {}\n", index, ast_to_pretty_string(param)).as_str();
            }

            println!("Body:");
            for (index, statement) in fn_def.body.iter().enumerate() {
                s += format!("[{}] item: {}\n", index, ast_to_pretty_string(statement)).as_str();
            }
        },
        ASTNode::Var(var) => {
            s += format!("Var: {}\n", var.name).as_str();
        },
        ASTNode::Assign(assign) => {
            println!("Assign");
            s += &format!("left: {}", ast_to_pretty_string(&assign.left));
            s += &format!("right: {}", ast_to_pretty_string(&assign.right));
        },
        ASTNode::Ident(ident) => {
            s += format!("Ident: {}\n", ident).as_str();
        },
        ASTNode::LiteralString(lit) => {
            s += format!("LiteralString: {}\n", lit).as_str();
        },
        ASTNode::LiteralInt(lit) => {
            s += format!("LiteralInt: {}\n", lit).as_str();
        },
        ASTNode::LiteralDecimal(lit) => {
            s += format!("LiteralDecimal: {}\n", lit).as_str();
        },
        ASTNode::LiteralPercent(lit) => {
            s += format!("LiteralPercent: {}\n", lit).as_str();
        },
        ASTNode::Object(obj) => {
            s += format!("Object: {}\n", obj.name).as_str();
        },
        ASTNode::Call(call) => {
            s += format!("Call: {:?}\n", call.callee).as_str();
        },
        _ => {}

        
    }

    s
}