use std::collections::HashMap;

use crate::component::Object;
use crate::parser::ASTNode;
use crate::parser::Call;
use crate::pretty::ast_pretty_string;
use crate::types::Action;
use crate::types::Const;
use crate::types::Value;

struct StructField {
    name: String,
    value: Value,
}

struct StructDef {
    name: String,
    fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub enum ByteCode {
    Load(usize),
    Store,
    CreateStruct(usize),
    AddField(usize),
    LoadStruct(usize),
    StoreField(usize),
    InstanceStruct(usize),
    LoadStrLit(usize),
    LoadIntLit(usize),
    LoadDecLit(f64),
    MakeArray(usize),
    MakeFn(usize),
    Call(usize),
}

#[derive(Debug)]
struct Scope {
    scopes: Vec<HashMap<usize, Value>>
}

impl Scope {
    fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn insert(&mut self, id: usize, val: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(id, val);
        }
    }

    fn get(&self, var: &usize) -> Option<&Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(var) {
                return Some(val);
            }
        }
        None
    }
}

struct IdentMap {
    idents: HashMap<String, usize>,
}

impl IdentMap {
    pub fn new() -> IdentMap {
        IdentMap {
            idents: HashMap::new(),
        }
    }

    pub fn get(&self, ident: &str) -> Option<usize> {
        self.idents.get(ident).copied()
    }

    pub fn insert(&mut self, ident: String, id: usize) {
        self.idents.insert(ident, id);
    }
}

#[derive(Debug)]
struct CodeFile {
    path: String,
    bytecode: Vec<ByteCode>,
    pc: usize,
    ast: Vec<ASTNode>,
}

impl CodeFile {
    pub fn ast_to_pretty_string(&self) -> String {
        let mut s = String::new();

        for node in self.ast.iter() {
           s += &ast_pretty_string(&node);
        }
        s
    }

    pub fn to_pretty_string(&self) -> String {
        let mut s = String::new();

        s += format!("pc: {}\n", self.pc).as_str();

        if !self.path.is_empty() {
            s.push_str(&format!("File: {}\n", self.path));
        }

        for (i, bc) in self.bytecode.iter().enumerate() {
            s.push_str(&format!("{:04} {:?}\n", i, bc));
        }

        s
    }
}

#[derive(Debug)]
struct CallItem {
    blk: usize,
    pc: usize,
}

pub struct Vm {
    code_blocks: Vec<Vec<ByteCode>>,
    call_stack: Vec<CallItem>,
    str_to_id: HashMap<String, usize>,
    id_to_str: HashMap<usize, String>,
    float_map: HashMap<f64, usize>,
    scope: Scope,
    stack: Vec<Value>,
    actions: Vec<Action>,
    consts: HashMap<usize, Value>
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            code_blocks: Vec::new(),
            call_stack: Vec::new(),
            scope: Scope::new(),
            str_to_id: HashMap::new(),
            id_to_str: HashMap::new(),
            float_map: HashMap::new(),
            stack: Vec::new(),
            actions: Vec::new(),
            consts: HashMap::new(),
        }
    }

    pub fn work(&mut self) -> &Vec<Action> {
        let mut item = match self.call_stack.last_mut() {
            Some(item) => item,
            None => {
                let item = CallItem {
                    blk: 0,
                    pc: 0,
                };

                self.call_stack.push(item);

                self.call_stack.last_mut().unwrap()
            }
        };

        println!("item: {:?}", item);

        let bytecode = &self.code_blocks[item.blk];

        while item.pc < bytecode.len() {
            let bc = &bytecode[item.pc];
            item.pc += 1;

            println!("pc: {:04} {:?}", item.pc, bc);

            match bc {
                ByteCode::Load(id) => {
                    // let val = self.scope.get(id).unwrap().clone();

                    // self.scope.insert(*id, val);
                }
                ByteCode::Store => {}
                ByteCode::CreateStruct(id) => {
                    // let struct_def = self.objects.get(id).unwrap().clone();

                    // self.scope.insert(*id, struct_def);
                }
                ByteCode::AddField(id) => {
                    // let struct_def = self.scope.get(id).unwrap().clone();

                    // self.scope.insert(*id, struct_def);
                }
                ByteCode::LoadStruct(id) => {
                    // let struct_def = self.scope.get(id).unwrap().clone();

                    // self.scope.insert(*id, struct_def);
                }
                ByteCode::StoreField(id) => {
                    // let struct_def = self.scope.get(id).unwrap().clone();

                    // self.scope.insert(*id, struct_def);
                }
                ByteCode::InstanceStruct(id) => {
                    // let struct_def = self.scope.get(id).unwrap().clone();

                    // self.scope.insert(*id, struct_def);
                }
                ByteCode::LoadStrLit(id) => {
                    let s: String = self.id_to_str.get(id).unwrap().clone();
                    self.stack.push(Value::String(s));
                }
                ByteCode::LoadIntLit(id) => {
                    let i: i64 = *id as i64;
                    self.stack.push(Value::Int(i));
                }
                ByteCode::LoadDecLit(id) => {
                    // let struct_def = self.scope.get(id).unwrap().clone();

                    // self.scope.insert(*id, struct_def);
                }
                ByteCode::MakeArray(id) => {
                    // let struct_def = self.scope.get(id).unwrap().clone();

                    // self.scope.insert(*id, struct_def);
                }
                ByteCode::MakeFn(id) => {
                    // let struct_def = self.scope.get(id).unwrap().clone();

                    // self.scope.insert(*id, struct_def);
                }
                ByteCode::Call(id) => {
                    // let struct_def = self.scope.get(id).unwrap().clone();

                    // self.scope.insert(*id, struct_def);
                }
            }
        }

        self.call_stack.pop();

        if self.call_stack.len() == 0 {
            self.actions.push(Action::Quit);
        }

        &self.actions
    }

    pub fn clear_actions(&mut self) {
        self.actions.clear();
    }

    pub fn store_const(&mut self, c: Const) {
        self.consts.insert(c.id, c.value);
    }

    pub fn create_code_block(&mut self, code: &[ByteCode]) -> usize {
        let id = self.code_blocks.len();

        self.code_blocks.push(code.to_vec());

        id
    }

    // pub fn run_file<P: AsRef<Path>>(mut self, path: P) -> Self {
    //     let code = std::fs::read_to_string(path).unwrap();

    //     let ast = parse_code(&code);

    //     self.run_ast(ast)
    // }

    // pub fn run_ast(mut self, ast: Vec<ASTNode>) -> Self {
    //     for node in ast {
    //         // match 
    //     }

    //     self
    // }

    fn get_str(&mut self, ident: &str) -> usize {
        match self.str_to_id.get(ident) {
            Some(id) => *id,
            None => {
                let id = self.str_to_id.len();

                self.str_to_id.insert(ident.to_string(), id);
                self.id_to_str.insert(id, ident.to_string());

                id
            }
        }
    }

    // pub fn compile_file<P: AsRef<Path>>(&mut self, path: P) {
    //     let path = path.as_ref();
        
    //     let code = std::fs::read_to_string(&path).unwrap();

    //     let ast = parse_code(&code);

    //     let code_file = self.gen_codefile(&ast);

    //     let path = path.display().to_string();

    //     match self.file_path_map.get(&path) {
    //         Some(id) => {
    //             self.files[*id] = code_file;
    //         }
    //         None => {
    //             self.files.push(code_file);

    //             let id = self.files.len() - 1;

    //             self.file_path_map.insert(path, id);
    //         }
    //     };
    // }

    // pub fn compile_code(&mut self, code: &str) {
    //     let ast = parse_code(code);

    //     self.compile_ast(&ast);
    // }

    // pub fn compile_ast(&mut self, ast: &[ASTNode]) {
    //     let code_file = self.gen_codefile(ast);

    //     self.files.push(code_file);
    // }

    // fn gen_codefile(&mut self, ast: &[ASTNode]) -> CodeFile {
    //     let mut code_file = CodeFile {
    //         path: String::new(),
    //         bytecode: Vec::new(),
    //         pc: 0,
    //         ast: ast.to_vec(),
    //     };

    //     for node in ast {
    //         self.compile_node(&mut code_file.bytecode, &node);
    //     }

    //     code_file
    // }

    // fn compile_node(&mut self, bytecode: &mut Vec<ByteCode>, node: &ASTNode) {
    //     match node {
    //         ASTNode::Ident(ident) => bytecode.push(ByteCode::Load(self.get_str(ident))),
    //         ASTNode::Lambda(_) => todo!(),
    //         ASTNode::Assign(asg) => {
    //             self.compile_node(bytecode, &asg.left);
    //             self.compile_node(bytecode, &asg.right);
    //             bytecode.push(ByteCode::Store);
    //         },
    //         ASTNode::StructIns(obj) => {

    //             for field in &obj.properties {
    //                 self.compile_node(bytecode, &field.value);
    //                 bytecode.push(ByteCode::StoreField(self.get_str(&field.name)));
    //             }

    //             bytecode.push(ByteCode::Load(self.get_str(&obj.name)));
    //         },
    //         ASTNode::ForLoop(_) => todo!(),
    //         ASTNode::Array(a) => {
    //             for item in &a.items {
    //                 self.compile_node(bytecode, item);
    //             }

    //             bytecode.push(ByteCode::MakeArray(a.items.len()));
    //         },
    //         ASTNode::Call(call) => {
    //             self.compile_node(bytecode, &call.callee);
    //             for a in &call.arguments {
    //                 self.compile_node(bytecode, a);
    //             }                
    //             bytecode.push(ByteCode::Call(call.arguments.len()))

    //         },
    //         ASTNode::TypeDef(_) => { /* We are going to ignore types in compiler for now */},
    //         ASTNode::Property(_, _) => todo!(),
    //         ASTNode::StructIns(obj) => todo!(),
    //         ASTNode::LiteralString(lit) => bytecode.push(ByteCode::LoadStrLit(self.get_str(&lit))),
    //         ASTNode::LiteralInt(lit) => bytecode.push(ByteCode::LoadIntLit(*lit as usize)),
    //         ASTNode::LiteralDecimal(lit) => bytecode.push(ByteCode::LoadDecLit(*lit)),
    //         ASTNode::LiteralPercent(_) => todo!(),
    //         ASTNode::FnDef(def) => {
    //             for p in &def.params {
    //                 self.compile_node(bytecode, p);
    //             }

    //             for item in &def.body {
    //                 self.compile_node(bytecode, item);
    //             }

    //             bytecode.push(ByteCode::MakeFn(def.params.len()));
    //         },
    //         ASTNode::StructDef(def) => {
    //             let ident_id = self.get_str(&def.name);
    //             bytecode.push(ByteCode::CreateStruct(ident_id));
                
    //             for field in &def.fields {
    //                 let ident_id = self.get_str(&field.name);
    //                 bytecode.push(ByteCode::AddField(ident_id));
    //             }
    //         },
    //         ASTNode::Var(def) => {

    //         },
    //         ASTNode::ProbAccess(prob) => {

    //         }
    //         ASTNode::Obj(obj) => todo!("Object literals are not supported yet"),
    //         ASTNode::Ret(ret) => {
    //             // self.compile_node(bytecode, &ret.value);
    //             // bytecode.push(ByteCode::Return);
    //         },
    //     }
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let mut vm = Vm::new();

    //     vm.compile_code(r#"
    //     Main {
    //         children: Window {
    //             title: "Testi Ikkuna"
    //             children: [
    //                 Box {
    //                     onClick: () => {
    //                         info("Hello world")
    //                     }
    //                     children: [
    //                         Text {
    //                             title: "qwerty"
    //                         }
    //                     ]
    //                 }
    //             ]
    //         }
    //     }
    //     "#);
    // }

    // #[test]
    // fn compile_todo_app() {
    //     let mut vm = Vm::new();

    //     vm.compile_code(r#"
    //     type Todo {
    //         id: int
    //         title: string
    //         done: bool
    //     }

    //     Todo todo = [] 

    //     Window {
    //         title: "Todo app"
    //         children: [
    //             Div {
    //                 children: [
    //                     Div {
    //                         children: todos.map((p) => Div {
    //                             flex_direction: FlexDirection.Row
    //                             children: [
    //                                 Text {
    //                                     text: p.title
    //                                 }
    //                                 Checkbox {
    //                                     checked: p.done
    //                                 }
    //                             ]
    //                         })
    //                     }
    //                     Div {
    //                         children: [
    //                             TextInput {
    //                                 placeholder: "Search word"
    //                             }
    //                         ] 
    //                     }
    //                 ]
    //             }
    //         ]
    //     }"#);

    //     let files = vm.get_code_files();

    //     for file in files {
    //         println!("{}", file.to_pretty_string());
    //     }
    // }
}