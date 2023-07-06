use std::collections::HashMap;
use std::path::Path;

use crate::component::Object;
use crate::parser::ASTNode;
use crate::parser::parse_code;
use crate::pretty::ast_pretty_string;
use crate::types::Value;

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

        s += &self.ast_to_pretty_string();

        s
    }
}

pub struct Vm {
    file_path_map: HashMap<String, usize>,
    files: Vec<CodeFile>,
    ident_map: HashMap<String, usize>,
    current_file: usize,
    scope: Scope,
    objects: HashMap<String, Box<dyn Object>>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            file_path_map: HashMap::new(),
            files: Vec::new(),
            current_file: 0,
            scope: Scope::new(),
            ident_map: HashMap::new(),
            objects: HashMap::new(),
        }
    }

    pub fn get_code_files(&self) -> &Vec<CodeFile> {
        &self.files
    }

    pub fn register_obj(&mut self, name: &str, obj: Box<dyn Object>) {
        self.objects.insert(name.to_string(), obj);
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
            path: String::new(),
            bytecode: Vec::new(),
            pc: 0,
            ast: ast.clone(),
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
            ASTNode::Assign(asg) => {
                self.compile_node(bytecode, &asg.left);
                self.compile_node(bytecode, &asg.right);
                bytecode.push(ByteCode::Store);
            },
            ASTNode::StructIns(obj) => {

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
            ASTNode::TypeDef(_) => { /* We are going to ignore types in compiler for now */},
            ASTNode::Property(_, _) => todo!(),
            ASTNode::StructIns(obj) => todo!(),
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
            ASTNode::Var(def) => {

            },
            ASTNode::ProbAccess(prob) => {

            }
            ASTNode::Obj(obj) => todo!("Object literals are not supported yet"),
            ASTNode::Ret(ret) => {
                // self.compile_node(bytecode, &ret.value);
                // bytecode.push(ByteCode::Return);
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
    }

    #[test]
    fn compile_todo_app() {
        let mut vm = Vm::new();

        vm.compile_code(r#"
        type Todo {
            id: int
            title: string
            done: bool
        }

        Todo todo = [] 

        Window {
            title: "Todo app"
            children: [
                Div {
                    children: [
                        Div {
                            children: todos.map((p) => Div {
                                flex_direction: FlexDirection.Row
                                children: [
                                    Text {
                                        text: p.title
                                    }
                                    Checkbox {
                                        checked: p.done
                                    }
                                ]
                            })
                        }
                        Div {
                            children: [
                                TextInput {
                                    placeholder: "Search word"
                                }
                            ] 
                        }
                    ]
                }
            ]
        }"#);

        let files = vm.get_code_files();

        for file in files {
            println!("{}", file.to_pretty_string());
        }
    }
}