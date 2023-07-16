use std::collections::HashMap;

use crate::parser::ASTNode;
use crate::types::Const;
use crate::vm::ByteCode;

pub struct CompileRes {
    pub bytecode: Vec<ByteCode>,
    pub consts: Vec<Const>,
}

pub struct Compiler {
    str_to_id: std::collections::HashMap<String, usize>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler{
            str_to_id: HashMap::new(),
        }
    }

    fn get_str(&mut self, ident: &str) -> usize {
        match self.str_to_id.get(ident) {
            Some(id) => *id,
            None => {
                let id = self.str_to_id.len();

                self.str_to_id.insert(ident.to_string(), id);
                // self.id_to_str.insert(id, ident.to_string());

                id
            }
        }
    }

    fn compile_node(&mut self, res: &mut CompileRes, node: &ASTNode) {
        match node {
            ASTNode::Ident(ident) => res.bytecode.push(ByteCode::Load(self.get_str(ident))),
            ASTNode::Assign(asg) => {
                self.compile_node(res, &asg.left);
                self.compile_node(res, &asg.right);
                res.bytecode.push(ByteCode::Store);
            },
            ASTNode::StructIns(obj) => {

                for field in &obj.probs {
                    self.compile_node(res, &field.value);
                    res.bytecode.push(ByteCode::StoreField(self.get_str(&field.name)));
                }

                res.bytecode.push(ByteCode::Load(self.get_str(&obj.name)));
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
            ASTNode::LiteralString(lit) => res.bytecode.push(ByteCode::LoadStrLit(self.get_str(&lit))),
            ASTNode::LiteralInt(lit) => res.bytecode.push(ByteCode::LoadIntLit(*lit as usize)),
            ASTNode::LiteralDecimal(lit) => res.bytecode.push(ByteCode::LoadDecLit(*lit)),
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
                let ident_id = self.get_str(&def.name);
                res.bytecode.push(ByteCode::CreateStruct(ident_id));
                
                for field in &def.fields {
                    let ident_id = self.get_str(&field.name);
                    res.bytecode.push(ByteCode::AddField(ident_id));
                }
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