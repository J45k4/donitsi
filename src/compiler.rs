use std::collections::HashMap;
use std::collections::HashSet;

use crate::parser::ASTNode;
use crate::types::Const;
use crate::types::Value;
use crate::vm::ByteCode;

#[derive(Debug, Clone)]
pub struct CompileRes {
    pub bytecode: Vec<ByteCode>,
    pub consts: Vec<Const>,
}

pub struct Compiler {
    pub consts: Vec<Value>,
    pub idents: HashMap<String, usize>
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler{
            consts: Vec::new(),
            idents: HashMap::new(),
        }
    }

    fn store_const(&mut self, v: Value) -> usize {
        let id = self.consts.len();

        self.consts.push(v);

        id
    }

    fn store_ident(&mut self, ident: &str) -> usize {
        match self.idents.get(ident) {
            Some(id) => *id,
            None => {
                let id = self.idents.len();

                self.idents.insert(ident.to_string(), id);

                id
            }
        }
    }

    // fn get_str(&mut self, ident: &str) -> usize {
    //     match self.str_to_id.get(ident) {
    //         Some(id) => *id,
    //         None => {
    //             let id = self.str_to_id.len();

    //             self.str_to_id.insert(ident.to_string(), id);
    //             // self.id_to_str.insert(id, ident.to_string());

    //             id
    //         }
    //     }
    // }

    fn compile_node(&mut self, res: &mut CompileRes, node: &ASTNode) {
        match node {
            ASTNode::Ident(ident) => res.bytecode.push(ByteCode::Load(self.store_ident(ident))),
            ASTNode::Assign(asg) => {
                self.compile_node(res, &asg.left);
                self.compile_node(res, &asg.right);
                res.bytecode.push(ByteCode::Store);
            },
            ASTNode::StructIns(obj) => {

                for field in &obj.probs {
                    self.compile_node(res, &field.value);
                    res.bytecode.push(ByteCode::StoreField(self.store_ident(&field.name)));
                }

                res.bytecode.push(ByteCode::Load(self.store_ident(&obj.name)));
            },
            ASTNode::ForLoop(_) => todo!(),
            ASTNode::Array(a) => {
                for item in &a.items {
                    self.compile_node(res, item);
                }

                res.bytecode.push(ByteCode::MakeArray(a.items.len()));
            },
            ASTNode::Call(call) => {
                self.compile_node(res, &call.callee);
                for a in &call.args {
                    self.compile_node(res, a);
                }                
                res.bytecode.push(ByteCode::Call(call.args.len()))

            },
            ASTNode::TypeDef(_) => { /* We are going to ignore types in compiler for now */},
            ASTNode::Property(_, _) => todo!(),
            ASTNode::StructIns(obj) => todo!(),
            ASTNode::Lit(lit) => res.bytecode.push(ByteCode::LoadConst(self.store_const(lit.clone()))),
            ASTNode::LiteralPercent(_) => todo!(),
            ASTNode::Fun(def) => {
                for p in &def.params {
                    self.compile_node(res, p);
                }

                for item in &def.body {
                    self.compile_node(res, item);
                }

                res.bytecode.push(ByteCode::MakeFn(def.params.len()));
            },
            ASTNode::StructDef(def) => {
                for field in &def.fields {
                    res.bytecode.push(ByteCode::AddField(self.store_ident(&field.name)));
                }

                res.bytecode.push(ByteCode::CreateStruct(self.store_ident(&def.name)));
            },
            ASTNode::Var(def) => {

            },
            ASTNode::ProbAccess(prob) => {

            }
            ASTNode::Obj(obj) => todo!("Object literals are not supported yet"),
            ASTNode::Ret(ret) => {
                // self.compile_node(bytecode, &ret.value);
                // bytecode.push(ByteCode::Return);
            },
            ASTNode::BinOp(_) => todo!(),
            
        }
    }

    pub fn compile(&mut self, ast: &[ASTNode]) -> CompileRes {
        let mut res = CompileRes {
            bytecode: Vec::new(),
            consts: Vec::new(),
        };
        
        for node in ast {
            self.compile_node(&mut res, node);
        }

        res
    }
}


#[cfg(test)]
mod tests {
    use crate::types::Value;

    use super::*;

    #[test]
    fn test_assign_num_lit() {
        let mut compiler = super::Compiler::new();

        let ast = vec![
            crate::parser::ASTNode::Assign(crate::parser::Assign{
                left: Box::new(crate::parser::ASTNode::Ident("x".to_string())),
                right: Box::new(crate::parser::ASTNode::Lit(Value::Int(10)))
            })
        ];

        let res = compiler.compile(&ast);

        println!("{:?}", res);

        assert_eq!(res.consts, vec![]);
        assert_eq!(res.bytecode, vec![
            ByteCode::Load(0),
            ByteCode::LoadConst(0),
            ByteCode::Store,
        ]);
    }

    #[test]
    fn test_assign_str_lit() {
        let mut compiler = super::Compiler::new();

        let ast = vec![
            crate::parser::ASTNode::Assign(crate::parser::Assign{
                left: Box::new(crate::parser::ASTNode::Ident("x".to_string())),
                right: Box::new(crate::parser::ASTNode::Lit(Value::Str("Hello".to_string()))),
            })
        ];

        let res = compiler.compile(&ast);

        println!("{:?}", res);

        assert_eq!(res.consts, vec![
            Const {
                id: 1,
                value: Value::Str("Hello".to_string())
            }
        ]);
        assert_eq!(res.bytecode, vec![
            ByteCode::Load(0),
            ByteCode::LoadConst(0),
            ByteCode::Store,
        ]);
    }
}