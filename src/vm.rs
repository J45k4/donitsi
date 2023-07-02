use std::collections::HashMap;
use std::path::Path;

use crate::component::Component;
use crate::parser::ASTNode;
use crate::parser::parse_code;

struct StructField {
    name: String,
    value: Value,
}

struct StructDef {
    name: String,
    fields: Vec<StructField>,
}

#[derive(Debug)]
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
    LoadFloatLit(f32),
    MakeArray(usize),
    MakeFn(usize),
    Call(usize),
}

#[derive(Debug)]
enum Value {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    Array(Vec<Value>),
    Null,
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
    bytecode: Vec<ByteCode>,
    pc: usize,
}

#[derive(Debug)]
pub struct Vm {
    file_path_map: HashMap<String, usize>,
    files: Vec<CodeFile>,
    ident_map: HashMap<String, usize>,
    current_file: usize,
    scope: Scope,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            file_path_map: HashMap::new(),
            files: Vec::new(),
            current_file: 0,
            scope: Scope::new(),
            ident_map: HashMap::new(),
        }
    }

    pub fn run_file<P: AsRef<Path>>(mut self, path: P) -> Self {
        let code = std::fs::read_to_string(path).unwrap();

        let ast = parse_code(&code);

        self.run_ast(ast)
    }

    pub fn run_ast(mut self, ast: Vec<ASTNode>) -> Self {
        for node in ast {
            // match 
        }

        self
    }

    fn get_ident(&mut self, ident: &str) -> usize {
        match self.ident_map.get(ident) {
            Some(id) => *id,
            None => {
                let id = self.ident_map.len();

                self.ident_map.insert(ident.to_string(), id);

                id
            }
        }
    }

    pub fn compile_file<P: AsRef<Path>>(&mut self, path: P) {
        let path = path.as_ref();
        
        let code = std::fs::read_to_string(&path).unwrap();

        let ast = parse_code(&code);

        let code_file = self.gen_codefile(ast);

        let path = path.display().to_string();

        match self.file_path_map.get(&path) {
            Some(id) => {
                self.files[*id] = code_file;
            }
            None => {
                self.files.push(code_file);

                let id = self.files.len() - 1;

                self.file_path_map.insert(path, id);
            }
        };
    }

    pub fn compile_code(&mut self, code: &str) {
        let ast = parse_code(code);

        let code_file = self.gen_codefile(ast);

        self.files.push(code_file);
    }

    fn gen_codefile(&mut self, ast: Vec<ASTNode>) -> CodeFile {
        let mut code_file = CodeFile {
            bytecode: Vec::new(),
            pc: 0,
        };

        for node in ast {
            self.compile_node(&mut code_file.bytecode, &node);
        }

        code_file
    }

    fn compile_node(&mut self, bytecode: &mut Vec<ByteCode>, node: &ASTNode) {
        match node {
            ASTNode::Ident(ident) => bytecode.push(ByteCode::Load(self.get_ident(ident))),
            ASTNode::Lambda(_) => todo!(),
            ASTNode::Assignment(_) => todo!(),
            ASTNode::ObjectDef(obj) => {
                for field in &obj.properties {
                    self.compile_node(bytecode, &field.value);
                    bytecode.push(ByteCode::StoreField(self.get_ident(&field.name)));
                }

                bytecode.push(ByteCode::Load(self.get_ident(&obj.name)));
            },
            ASTNode::ForLoop(_) => todo!(),
            ASTNode::Array(a) => {
                for item in &a.items {
                    self.compile_node(bytecode, item);
                }

                bytecode.push(ByteCode::MakeArray(a.items.len()));
            },
            ASTNode::Call(call) => {
                self.compile_node(bytecode, &call.callee);
                for a in &call.arguments {
                    self.compile_node(bytecode, a);
                }                
                bytecode.push(ByteCode::Call(call.arguments.len()))

            },
            ASTNode::VarDef(_, _) => todo!(),
            ASTNode::TypeDef(_, _) => todo!(),
            ASTNode::Property(_, _) => todo!(),
            ASTNode::Object(_, _) => todo!(),
            ASTNode::LiteralString(lit) => bytecode.push(ByteCode::LoadStrLit(self.get_ident(&lit))),
            ASTNode::LiteralInt(lit) => bytecode.push(ByteCode::LoadIntLit(*lit as usize)),
            ASTNode::LiteralDecimal(_) => todo!(),
            ASTNode::LiteralPercent(_) => todo!(),
            ASTNode::FnDef(def) => {
                for p in &def.params {
                    self.compile_node(bytecode, p);
                }

                for item in &def.body {
                    self.compile_node(bytecode, item);
                }

                bytecode.push(ByteCode::MakeFn(def.params.len()));
            },
            ASTNode::StructDef(def) => {
                let ident_id = self.get_ident(&def.name);
                bytecode.push(ByteCode::CreateStruct(ident_id));
                
                for field in &def.fields {
                    let ident_id = self.get_ident(&field.name);
                    bytecode.push(ByteCode::AddField(ident_id));
                }
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vm = Vm::new();

        vm.compile_code(r#"
        Main {
            children: Window {
                title: "Testi Ikkuna"
                children: [
                    Box {
                        onClick: () => {
                            info("Hello world")
                        }
                        children: [
                            Text {
                                title: "qwerty"
                            }
                        ]
                    }
                ]
            }
        }
        "#);

        println!("{:#?}", vm);
    }
}