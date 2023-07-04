use logos::{Logos, skip};

#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
	#[token(" ", skip)]
	#[token("\t", skip)]
	#[token("\n", skip)]
	#[token("\r", skip)]
	Whitespace,
	#[token("for")]
	For,
	#[token("type")]
	Type,
	#[token("=>")]
	Arrow,
	#[token("{")]
	OpenBrace,
	#[token("}")]
	CloseBrace,
	#[token("(")]
	OpenParen,
	#[token(")")]
	CloseParen,
	#[token("[")]
	OpenBracket,
	#[token("]")]
	CloseBracket,
	#[token("::")]
	DoubleColon,
	#[token(":")]
	Colon,
	#[token(",")]
	Comma,
	#[token(".")]
	Dot,
	#[token("=")]
	Assigment,
	#[regex(r#""[^"]*""#, |t| t.slice()[1..t.slice().len()-1].to_string())]
	String(String),
	#[regex(r"-?[0-9]+", |t| t.slice().parse::<i64>())]
	Int(i64),
	#[regex(r"-?[0-9]*\.[0-9]+", |t| t.slice().parse::<f64>())]
	Decimal(f64),
	#[token("struct")]
	Struct,
	#[token("Int")]
	IntDef,
	#[token("Float")]
	FloatDef,
	#[token("String")]
	StringDef,
	#[regex(r"[A-Za-z_]+", |t| t.slice().to_string())]
	Ident(String),
	#[error]
	Error,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
	pub params: Vec<ASTNode>,
	pub body: Vec<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assign {
	pub left: Box<ASTNode>,
	pub right: Box<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
	pub name: String,
	pub value: Box<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
	pub name: String,
	pub properties: Vec<Property>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForLoop {
	pub iterator: Box<ASTNode>,
	pub body: Box<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Array {
	pub items: Vec<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
	pub callee: Box<ASTNode>,
	pub arguments: Vec<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VarType {
	Int,
	Float,
	String,
	Var(String),
	StrLit(String),
	FnDef(FnDef),
	Ident(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeField {
	pub name: String,
	pub typ: VarType
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructDef {
	pub name: String,
	pub fields: Vec<TypeField>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FnDef {
	pub params: Vec<ASTNode>,
	pub body: Vec<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDef {
	pub name: String,
	pub fields: Vec<TypeField>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Var {
	pub name: String,
	pub typ: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProbAccess {
	pub object: Box<ASTNode>,
	pub property: Box<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
	Ident(String),
	Lambda(Lambda),
	Assign(Assign),
	Object(Object),
	ForLoop(ForLoop),
	Array(Array),
	Call(Call),
	Property(String, Box<ASTNode>),
	LiteralString(String),
	LiteralInt(i64),
	LiteralDecimal(f64),
	LiteralPercent(f64),
	FnDef(FnDef),
	StructDef(StructDef),
	TypeDef(TypeDef),
	Var(Var),
	ProbAccess(ProbAccess),
}

fn expect_token(tokens: &mut Vec<Token>, expected: Token) {
	let token = tokens.pop();
	assert_eq!(token, Some(expected.clone()), "Expected {:?}, got {:?}", expected, token);
}

fn expect_identifier(tokens: &mut Vec<Token>) -> String {
	let token = tokens.pop();
	match token {
		Some(Token::Ident(ident)) => ident,
		_ => panic!("Expected identifier, got {:?}", token)
	}
}

fn eat_if(tokens: &mut Vec<Token>, expected: Token) -> bool {
	match tokens.last() {
		Some(t) => {
			if t == &expected {
				tokens.pop();
				true
			} else {
				false
			}
		},
		None => false
	}
}

fn parse_obj_probs(tokens: &mut Vec<Token>) -> Vec<Property> {
	println!("parse_object_def_properties");

	let mut properties = Vec::new();

	while let Some(token) = tokens.pop() {
		println!("object token {:?}", token);

		match token {
			Token::CloseBrace => {
				break;
			},
			Token::Ident(ident) => {
				println!("object property ident: {}", ident);

				expect_token(tokens, Token::Colon);
				let value = parse_node(tokens).unwrap();
				let prop = Property { 
					name: ident,
					value: Box::new(value) 
				};
				properties.push(prop);
			},
			_ => {
				panic!("Unexpected token while parsing object definition properties: {:?}", token)
			}
		}
	}

	properties
}

fn parse_list(tokens: &mut Vec<Token>) -> Vec<ASTNode> {
	println!("parse_list");

	let mut list = Vec::new();

	while let Some(token) = tokens.pop() {
		println!("list token {:?}", token);

		match token {
			Token::CloseBracket => {
				break;
			},
			_ => {
				let node = parse_node(tokens).unwrap();
				list.push(node);
			}
		}
	}

	list
}

fn parse_identifier(mut tokens: &mut Vec<Token>, ident: &str) -> Option<ASTNode> {
	println!("parse_identifier: {:?}", ident);

	let next = tokens.last().unwrap().clone();

	match next {
		Token::OpenBrace => {
			tokens.pop();

			let props: Vec<Property> = parse_obj_probs(&mut tokens);
		
			Some(ASTNode::Object(
				Object {
					name: ident.to_string(),
					properties: props,
				}
			))
		},
		Token::Assigment => {
			tokens.pop();

			println!("parsing assignment: {}", ident);

			let left = ASTNode::Ident(ident.to_string());
			let right = parse_node(&mut tokens).unwrap();

			let node: ASTNode = ASTNode::Assign(
				Assign {
					left: Box::new(left),
					right: Box::new(right),
				}
			);

			Some(node)
		},
		Token::OpenParen => {
			tokens.pop();

			println!("parsing call: {}", ident);

			let mut call = Call {
				callee: Box::new(ASTNode::Ident(ident.to_string())),
				arguments: Vec::new(),
			};

			while let Some(next) = tokens.last() {
				match next {
					Token::CloseParen => {
						tokens.pop();
						break;
					},
					Token::Comma => {
						tokens.pop();
					},
					_ => {
						call.arguments.push(parse_node(tokens).unwrap());
					}
				}
			};

			Some(ASTNode::Call(call))
		},
		Token::Dot => {
			tokens.pop();

			println!("prob access");

			let prob = ProbAccess {
				object: Box::new(ASTNode::Ident(ident.to_string())),
				property: match tokens.pop().unwrap() {
					Token::Ident(idt) => {
						Box::new(parse_identifier(tokens, &idt).unwrap())
					},
					_ => {
						todo!();
					}
				},
			};

			Some(ASTNode::ProbAccess(prob))
		},
		_ => {
			Some(ASTNode::Ident(ident.to_string()))
		}
	}
}

fn parse_func_def(tokens: &mut Vec<Token>) -> FnDef {
	println!("parse_func_def");

	let mut fn_def = FnDef {
		params: Vec::new(),
		body: Vec::new(),
	};

	while let Some(token) = tokens.pop() {
		println!("token: {:?}", token);

		match token {
			Token::CloseParen => {
				break;
			},
			Token::Ident(ident) => {
				parse_identifier(tokens, &ident);
			},
			_ => {
				todo!()
			}
		};
	};

	expect_token(tokens, Token::Arrow);

	// let next = tokens.pop().unwrap();

	// println!("next: {:?}", next);

	let mut braced = false;
	while let Some(next) = tokens.last() {
		match next {
			Token::OpenBrace => {
				tokens.pop();
				braced = true;
			},
			Token::CloseBrace => {
				tokens.pop();
				break;
			},
			_ => {

				let n = match parse_node(tokens) {
					Some(n) => n,
					None => break,
				};

				fn_def.body.push(n);

				if !braced {
					println!("breaking since not braced");

					break;
				}
			}
		};
	};

	println!("parsing fn def finished");

	fn_def
	
	// match next {
	// 	Token::Arrow => {
	// 		let next = tokens.pop().unwrap();

	// 		match next {
	// 			Token::OpenBrace => {
	// 				//let props: Vec<Property> = parse_object_def_properties(&mut tokens);

	// 				let mut fdef = FunctionDef {
	// 					params: Vec::new(),
	// 					body: Vec::new(),
	// 				};

	// 				while let Some(token) = tokens.pop() {
	// 					let node = match token {
	// 						Token::Identifier(ident) => {
	// 							parse_identifier(tokens, &ident)
	// 						},
	// 						Token::CloseBrace => {
	// 							break;
	// 						},
	// 						_ => {
	// 							todo!("Unexpected token: {:?}", token);
	// 						}
	// 					};

	// 					fdef.body.push(node);
	// 				}

	// 				Node::FunctionDef(fdef)
	// 			},
	// 			_ => {
	// 				todo!();
	// 			}
	// 		}
	// 	},
	// 	_ => {
	// 		todo!();
	// 	}
	// }
}

fn parse_body_node(tokens: &mut Vec<Token>) -> Option<ASTNode> {
	println!("parse_body_node");

	let next = tokens.pop().unwrap();

	match next {
		Token::Ident(ident) => {
			println!("ident: {}", ident);

			match tokens.pop().unwrap() {
				Token::Ident(var_name) => {
					println!("var name: {}", var_name);

					let var = Var {
						name: var_name,
						typ: ident,
					};
					let next = tokens.last().unwrap();

					match next {
						Token::Assigment => {
							tokens.pop();
							Some(ASTNode::Assign(Assign {
								left: Box::new(ASTNode::Var(var)),
								right: Box::new(parse_node(tokens).unwrap()),
							}))
						},
						_ => None
					}
				},
				Token::OpenBrace => {
					let props: Vec<Property> = parse_obj_probs(tokens);
		
					Some(ASTNode::Object(
						Object {
							name: ident.to_string(),
							properties: props,
						}
					))
				},	
				_ => {
					todo!();
				}
			}
		},
		Token::Type => {
			println!("type");

			let ident = expect_identifier(tokens);

			expect_token(tokens, Token::OpenBrace);

			let mut type_def = TypeDef {
				name: ident,
				fields: Vec::new(),
			};
			
			while let Some(token) = tokens.pop() {
				match token {
					Token::CloseBrace => {
						break;
					},
					Token::Ident(ident) => {
						println!("Identifier: {}", ident);

						expect_token(tokens, Token::Colon);

						let typ = match tokens.pop().unwrap() {
							Token::IntDef => VarType::Int,
							Token::FloatDef => VarType::Float,
							Token::StringDef => VarType::String,
							Token::Ident(ident) => VarType::Var(ident),
							Token::String(str) => VarType::StrLit(str),
							Token::OpenParen => VarType::FnDef(parse_func_def(tokens)),
							_ => {
								todo!();
							}
						};

						let field = TypeField {
							name: ident,
							typ: typ,
						};

						type_def.fields.push(field);
					},
					_ => {
						todo!();
					}
				}
			}

			Some(ASTNode::TypeDef(type_def))
		},
		_ => {
			todo!("Unexpected token: {:?}", next);
		}
	}
}

fn parse_node(tokens: &mut Vec<Token>) -> Option<ASTNode> {
	println!("parse_node");

	let next = tokens.pop().unwrap();

	match next {
		Token::Ident(ident) => {
			println!("node is identifier: {:?}", ident);

			parse_identifier(tokens, &ident)
		},
		Token::String(str) => {
			println!("LiteralString: {:?}", str);
			Some(ASTNode::LiteralString(str))
		},
		Token::Int(i) => {
			Some(ASTNode::LiteralInt(i))
		},
		Token::CloseBrace => {
			None
		},
		Token::OpenBracket => {
			println!("OpenBracket");

			let mut items = Vec::new();

			while let Some(token) = tokens.last() {
				match token {
					Token::CloseBracket => {
						println!("CloseBracket");

						tokens.pop();
						break;
					},
					_ => {
						let node = parse_node(tokens).unwrap();
						items.push(node);
					}
				}
			}

			let array = Array {
				items: items,
			};

			Some(ASTNode::Array(array))
		},
		Token::OpenParen => {
			Some(ASTNode::FnDef(parse_func_def(tokens)))
		},
		Token::Struct => {
			println!("Struct");

			let token = tokens.pop().unwrap();

			let name = match token {
				Token::Ident(ident) => {
					ident
				},
				_ => {
					todo!();
				}
			};

			expect_token(tokens, Token::OpenBrace);

			let mut struct_def = StructDef {
				name: name,
				fields: Vec::new(),
			};

			while let Some(token) = tokens.pop() {
				match token {
					Token::CloseBrace => {
						break;
					},
					Token::Ident(ident) => {
						println!("Identifier: {}", ident);

						expect_token(tokens, Token::Colon);

						let typ = match tokens.pop().unwrap() {
							Token::IntDef => VarType::Int,
							Token::FloatDef => VarType::Float,
							Token::StringDef => VarType::String,
							Token::Ident(ident) => VarType::Var(ident),
							Token::String(str) => VarType::StrLit(str),
							Token::OpenParen => VarType::FnDef(parse_func_def(tokens)),
							_ => {
								todo!();
							}
						};

						let field = TypeField {
							name: ident,
							typ: typ,
						};

						struct_def.fields.push(field);
					},
					_ => {
						todo!();
					}
				}
			}

			Some(ASTNode::StructDef(struct_def))
		},
		Token::CloseParen => None,
		// Token::DoubleColon => {
		// 	println!("double colon");


		// },
		_ => {
			todo!("Unexpected token: {:?}", next);
		}
	}
}

pub fn parse_code(input: &str) -> Vec<ASTNode> {
	let lexer = Token::lexer(input);
	let mut tokens: Vec<Token> = lexer.collect();
	tokens.reverse();

	println!("tokens: {:?}", tokens);

	let mut ast = Vec::new();

	while let Some(_) = tokens.last() {
		let node = match parse_body_node(&mut tokens) {
			Some(n) => n,
			None => break,
		};

		ast.push(node);
	}

	ast
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_code;

    #[test]
    fn it_works() {
        let code = r#"Main {
            children: Window {
                title: "Testi Ikkuna"
                children: [
                    Box {
                        onClick: () => {
                            info("Hello world")
                            info("Hello world")
                            info("Hello world")
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
        }"#;

        parse_code(code);
    }

    #[test]
    fn test_parse_struct() {
        let code = r#"
            struct Person {
                name: "Testi"
                age: Int

                say_hello: () => {
                    info("Hello world")
                }
            }
        "#;

        parse_code(code);
    }

	#[test]
	fn test_parse_import() {

	}
}