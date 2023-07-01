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
	#[token(": ")]
	Colon,
	#[token("=")]
	Equal,
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
	Identifier(String),
	#[error]
	Error,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
	pub params: Vec<Node>,
	pub body: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
	pub left: Box<Node>,
	pub right: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
	pub name: String,
	pub value: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectDef {
	pub name: String,
	pub properties: Vec<Property>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForLoop {
	pub iterator: Box<Node>,
	pub body: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Array {
	pub items: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
	pub callee: Box<Node>,
	pub arguments: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VarType {
	Int,
	Float,
	String,
	Var(String),
	StrLit(String),
	FnDef(FnDef),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructField {
	pub name: String,
	pub typ: VarType
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructDef {
	pub name: String,
	pub fields: Vec<StructField>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FnDef {
	pub params: Vec<Node>,
	pub body: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
	Ident(String),
	Lambda(Lambda),
	Assignment(Assignment),
	ObjectDef(ObjectDef),
	ForLoop(ForLoop),
	Array(Array),
	Call(Call),
	VarDef(String, Box<Node>),
	TypeDef(String, Vec<Node>),
	Property(String, Box<Node>),
	Object(String, Vec<Node>),
	LiteralString(String),
	LiteralInt(i64),
	LiteralDecimal(f64),
	LiteralPercent(f64),
	FnDef(FnDef),
	StructDef(StructDef),
}

fn expect_token(tokens: &mut Vec<Token>, expected: Token) {
	let token = tokens.pop();
	assert_eq!(token, Some(expected.clone()), "Expected {:?}, got {:?}", expected, token);
}

fn parse_object_def_properties(tokens: &mut Vec<Token>) -> Vec<Property> {
	println!("parse_object_def_properties");

	let mut properties = Vec::new();

	while let Some(token) = tokens.pop() {
		println!("object token {:?}", token);

		match token {
			Token::CloseBrace => {
				break;
			},
			Token::Identifier(ident) => {
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

fn parse_list(tokens: &mut Vec<Token>) -> Vec<Node> {
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

fn parse_identifier(mut tokens: &mut Vec<Token>, ident: &str) -> Node {
	println!("Identifier: {:?}", ident);

	let next_token = tokens.pop().unwrap();

	match next_token {
		Token::OpenBrace => {
			let props: Vec<Property> = parse_object_def_properties(&mut tokens);
		
			Node::ObjectDef(
				ObjectDef {
					name: ident.to_string(),
					properties: props,
				}
			)
		},
		Token::Equal => {
			tokens.pop();

			let left = Node::Ident(ident.to_string());
			let right = parse_node(&mut tokens).unwrap();

			let node = Node::Assignment(
				Assignment {
					left: Box::new(left),
					right: Box::new(right),
				}
			);

			node
		},
		Token::OpenParen => {
			let mut call = Call {
				callee: Box::new(Node::Ident(ident.to_string())),
				arguments: Vec::new(),
			};

			while let Some(next) = tokens.pop() {
				match next {
					Token::CloseParen => {
						break;
					},
					Token::String(str) => {
						println!("String: {}", str);
						call.arguments.push(Node::LiteralString(str));
					},
					_ => {
						todo!();
					}
				}
			};

			Node::Call(call)
		},
		// Token::OpenBracket => {
		// 	parse_list(tokens)
		// },
		_ => {
			panic!("Unexpected token: {:?}", next_token);
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
			_ => {
				todo!();
			}
		};
	};

	expect_token(tokens, Token::Arrow);

	let next = tokens.pop().unwrap();

	println!("next: {:?}", next);

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

				if braced {
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

fn parse_node(tokens: &mut Vec<Token>) -> Option<Node> {
	println!("parse_node");

	match tokens.pop().unwrap() {
		Token::Identifier(ident) => {
			Some(parse_identifier(tokens, &ident))
		},
		Token::String(str) => {
			println!("LiteralString: {:?}", str);
			Some(Node::LiteralString(str))
		},
		Token::Int(i) => {
			Some(Node::LiteralInt(i))
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

			Some(Node::Array(array))
		},
		Token::OpenParen => {
			Some(Node::FnDef(parse_func_def(tokens)))
		},
		Token::Struct => {
			println!("Struct");

			let token = tokens.pop().unwrap();

			let name = match token {
				Token::Identifier(ident) => {
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
					Token::Identifier(ident) => {
						println!("Identifier: {}", ident);

						expect_token(tokens, Token::Colon);

						let typ = match tokens.pop().unwrap() {
							Token::IntDef => VarType::Int,
							Token::FloatDef => VarType::Float,
							Token::StringDef => VarType::String,
							Token::Identifier(ident) => VarType::Var(ident),
							Token::String(str) => VarType::StrLit(str),
							Token::OpenParen => VarType::FnDef(parse_func_def(tokens)),
							_ => {
								todo!();
							}
						};

						let field = StructField {
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

			Some(Node::StructDef(struct_def))
		},
		_ => {
			todo!();
		}
	}
}

pub fn parse_code(input: &str) -> Vec<Node> {
	let lexer = Token::lexer(input);
	let mut tokens: Vec<Token> = lexer.collect();
	tokens.reverse();

	let mut ast = Vec::new();

	while let Some(_) = tokens.last() {
		let node = match parse_node(&mut tokens) {
			Some(n) => n,
			None => break,
		};

		ast.push(node);
	}

	ast
}