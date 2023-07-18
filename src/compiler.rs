use std::collections::HashMap;

use crate::parser::ASTNode;
use crate::parser::Op;
use crate::types::Const;
use crate::types::Value;
use crate::vm::ByteCode;

#[derive(Debug, Clone)]
pub struct CompileRes {
    pub bytecode: Vec<ByteCode>,
    pub consts: Vec<Const>,
}

#[derive(Debug, Clone)]
pub struct Compiler {
    pub consts: Vec<Value>,
    pub idents: HashMap<String, usize>,
    pub bytecode: Vec<ByteCode>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler{
            consts: Vec::new(),
            idents: HashMap::new(),
            bytecode: Vec::new(),
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

    fn compile_node(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Ident(ident) => {
                let id = self.store_ident(&ident);
                self.bytecode.push(ByteCode::Load(id));
            },
            ASTNode::Assign(asg) => {
                self.compile_node(&asg.left);
                self.compile_node(&asg.right);
                self.bytecode.push(ByteCode::Store);
            },
            ASTNode::StructIns(obj) => {

                for field in &obj.probs {
                    self.compile_node(&field.value);
                    let id = self.store_ident(&field.name);
                    self.bytecode.push(ByteCode::StoreField(id));
                }

                let id = self.store_ident(&obj.name);
                self.bytecode.push(ByteCode::Load(id));
            },
            ASTNode::ForLoop(_) => todo!(),
            ASTNode::Array(a) => {
                for item in &a.items {
                    self.compile_node(&item);
                }

                self.bytecode.push(ByteCode::MakeArray(a.items.len()));
            },
            ASTNode::Call(call) => {
                self.compile_node(&call.callee);
                for a in &call.args {
                    self.compile_node(&a);
                }                
                self.bytecode.push(ByteCode::Call(call.args.len()))

            },
            ASTNode::TypeDef(_) => { /* We are going to ignore types in compiler for now */},
            ASTNode::Property(_, _) => todo!(),
            ASTNode::StructIns(obj) => todo!(),
            ASTNode::Lit(lit) => {
                let id = self.store_const(lit.clone());
                self.bytecode.push(ByteCode::LoadConst(id))
            },
            ASTNode::LiteralPercent(_) => todo!(),
            ASTNode::Fun(def) => {
                for p in &def.params {
                    self.compile_node(&p);
                }

                for item in &def.body {
                    self.compile_node(&item);
                }

                self.bytecode.push(ByteCode::MakeFn(def.params.len()));
            },
            ASTNode::StructDef(def) => {
                for field in &def.fields {
                    let id = self.store_ident(&field.name);
                    self.bytecode.push(ByteCode::AddField(id));
                }

                let id = self.store_ident(&def.name);
                self.bytecode.push(ByteCode::CreateStruct(id));
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
            ASTNode::BinOp(bin_op) => {
                self.compile_node(&bin_op.left);
                self.compile_node(&bin_op.right);

                match bin_op.op {
                    Op::Plus => self.bytecode.push(ByteCode::Add),
                    Op::Minus => self.bytecode.push(ByteCode::Sub),
                    Op::Multiply => self.bytecode.push(ByteCode::Mul),
                    Op::Divide => self.bytecode.push(ByteCode::Div),
                }
            },
            
        }
    }

    pub fn compile(mut self, ast: Vec<ASTNode>) -> Self {
        for node in &ast {
            self.compile_node(node);
        }

        self
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::BinOp;
    use crate::parser::Op;
    use crate::types::Value;

    use super::*;

    #[test]
    fn test_assign_num_lit() {
        let ast = vec![
            crate::parser::ASTNode::Assign(crate::parser::Assign{
                left: Box::new(crate::parser::ASTNode::Ident("x".to_string())),
                right: Box::new(crate::parser::ASTNode::Lit(Value::Int(10)))
            })
        ];

        let compiler = Compiler::new().compile(ast);

        assert_eq!(compiler.consts, vec![Value::Int(10)]);
        assert_eq!(compiler.bytecode, vec![
            ByteCode::Load(0),
            ByteCode::LoadConst(0),
            ByteCode::Store,
        ]);
    }

    #[test]
    fn test_assign_str_lit() {
        let ast = vec![
            crate::parser::ASTNode::Assign(crate::parser::Assign{
                left: Box::new(crate::parser::ASTNode::Ident("x".to_string())),
                right: Box::new(crate::parser::ASTNode::Lit(Value::Str("Hello".to_string()))),
            })
        ];

        let compiler = Compiler::new().compile(ast);

        println!("{:?}", compiler);

        assert_eq!(compiler.consts, vec![
            Value::Str("Hello".to_string())
        ]);
        assert_eq!(compiler.bytecode, vec![
            ByteCode::Load(0),
            ByteCode::LoadConst(0),
            ByteCode::Store,
        ]);
    }

    #[test]
    fn test_simple_binop() {
        let ast = vec![
            ASTNode::BinOp(BinOp{
                left: Box::new(ASTNode::Lit(Value::Int(10))),
                right: Box::new(ASTNode::Lit(Value::Int(20))),
                op: Op::Plus,
            })
        ];

        let compiler = Compiler::new().compile(ast);

        assert_eq!(compiler.consts, vec![
            Value::Int(10),
            Value::Int(20),
        ]);
        assert_eq!(compiler.bytecode, vec![
            ByteCode::LoadConst(0),
            ByteCode::LoadConst(1),
            ByteCode::Add,
        ]);
    }

    #[test]
    fn test_more_complicated_binop() {
        let ast = vec![
            ASTNode::BinOp(BinOp{
                left: Box::new(ASTNode::BinOp(BinOp{
                    left: Box::new(ASTNode::Lit(Value::Int(10))),
                    right: Box::new(ASTNode::Lit(Value::Int(20))),
                    op: Op::Plus,
                })),
                right: Box::new(ASTNode::Lit(Value::Int(30))),
                op: Op::Plus,
            })
        ];

        let compiler = Compiler::new().compile(ast);

        assert_eq!(compiler.consts, vec![
            Value::Int(10),
            Value::Int(20),
            Value::Int(30),
        ]);
        assert_eq!(compiler.bytecode, vec![
            ByteCode::LoadConst(0),
            ByteCode::LoadConst(1),
            ByteCode::Add,
            ByteCode::LoadConst(2),
            ByteCode::Add,
        ]);
    }

    // #[test]
    // fn 
}