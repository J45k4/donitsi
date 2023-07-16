use logos::{Logos, skip, Span};

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
	Assign,
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
	#[token("return")]
	Return,
	#[token("+")]
	Plus,
	#[token("-")]
	Minus,
	#[token("*")]
	Multiply,
	#[token("/")]
	Divide,
	#[regex(r"[A-Za-z_]+", |t| t.slice().to_string())]
	Ident(String),
	#[error]
	Error,
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
pub struct StructIns {
	pub name: String,
	pub probs: Vec<Property>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Obj {
	pub probs: Vec<Property>,
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
	pub args: Vec<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VarType {
	Int,
	Float,
	String,
	Var(String),
	StrLit(String),
	FnDef(Fun),
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
pub struct Fun {
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
	pub property: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
	Plus,
	Minus,
	Multiply,
	Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinOp {
	pub left: Box<ASTNode>,
	pub op: Op,
	pub right: Box<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
	Ident(String),
	Assign(Assign),
	StructIns(StructIns),
	ForLoop(ForLoop),
	Array(Array),
	Call(Call),
	Property(String, Box<ASTNode>),
	LiteralString(String),
	LiteralInt(i64),
	LiteralDecimal(f64),
	LiteralPercent(f64),
	Fun(Fun),
	StructDef(StructDef),
	TypeDef(TypeDef),
	Var(Var),
	ProbAccess(ProbAccess),
	Obj(Obj),
	Ret(Box<ASTNode>),
	BinOp(BinOp)
}

fn expect_token(tokens: &mut Vec<Token>, expected: Token) {
	println!("expect_token: {:?}", expected);
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

// fn parse_obj_probs(tokens: &mut Vec<Token>) -> Vec<Property> {
// 	println!("parse obj probs start");

// 	let mut properties = Vec::new();

// 	while let Some(token) = tokens.pop() {
// 		println!("object token {:?}", token);

// 		match token {
// 			Token::CloseBrace => {
// 				break;
// 			},
// 			Token::Ident(ident) => {
// 				println!("object property ident: {}", ident);

// 				expect_token(tokens, Token::Colon);
// 				let value = parse_node(tokens).unwrap();
// 				let prop = Property { 
// 					name: ident,
// 					value: Box::new(value) 
// 				};
// 				properties.push(prop);
// 			},
// 			Token::Comma => {
// 				continue;
// 			},
// 			_ => {
// 				panic!("Unexpected token while parsing object definition properties: {:?}", token)
// 			}
// 		}
// 	}

// 	println!("parse obj probs end");

// 	properties
// }

// fn parse_list(tokens: &mut Vec<Token>) -> Vec<ASTNode> {
// 	println!("parse_list");

// 	let mut list = Vec::new();

// 	while let Some(token) = tokens.pop() {
// 		println!("list token {:?}", token);

// 		match token {
// 			Token::CloseBracket => {
// 				break;
// 			},
// 			_ => {
// 				let node = parse_node(tokens).unwrap();
// 				list.push(node);
// 			}
// 		}
// 	}

// 	list
// }

// fn parse_ident(mut tokens: &mut Vec<Token>, ident: &str) -> Option<ASTNode> {
// 	println!("parse_ident: {:?}", ident);

// 	let next = tokens.last().unwrap().clone();

// 	match next {
// 		Token::OpenBrace => {
// 			tokens.pop();

// 			let props: Vec<Property> = parse_obj_probs(&mut tokens);
		
// 			Some(ASTNode::StructIns(
// 				StructIns {
// 					name: ident.to_string(),
// 					properties: props,
// 				}
// 			))
// 		},
// 		Token::Assign => {
// 			tokens.pop();

// 			println!("parsing assignment: {}", ident);

// 			let left = ASTNode::Ident(ident.to_string());
// 			let right = parse_node(&mut tokens).unwrap();

// 			let node: ASTNode = ASTNode::Assign(
// 				Assign {
// 					left: Box::new(left),
// 					right: Box::new(right),
// 				}
// 			);

// 			Some(node)
// 		},
// 		Token::OpenParen => {
// 			tokens.pop();

// 			println!("parsing call: {}", ident);

// 			let mut call = Call {
// 				callee: Box::new(ASTNode::Ident(ident.to_string())),
// 				arguments: Vec::new(),
// 			};

// 			while let Some(next) = tokens.last() {
// 				match next {
// 					Token::CloseParen => {
// 						tokens.pop();
// 						break;
// 					},
// 					Token::Comma => {
// 						tokens.pop();
// 					},
// 					_ => {
// 						match parse_node(tokens) {
// 							Some(n) => call.arguments.push(n),
// 							None => {
// 								println!("next parsed node is none");
// 								println!("next token: {:?}", tokens.last());
// 								break;
// 							},
// 						}
// 					}
// 				}
// 			};

// 			println!("parsing call end");

// 			Some(ASTNode::Call(call))
// 		},
// 		Token::Dot => {
// 			tokens.pop();

// 			println!("prob access");

// 			let prob = ProbAccess {
// 				object: Box::new(ASTNode::Ident(ident.to_string())),
// 				property: match tokens.pop().unwrap() {
// 					Token::Ident(idt) => {
// 						Box::new(parse_ident(tokens, &idt).unwrap())
// 					},
// 					_ => {
// 						todo!();
// 					}
// 				},
// 			};

// 			Some(ASTNode::ProbAccess(prob))
// 		},
// 		Token::Arrow => {
// 			println!("arrow");

// 			tokens.pop();

// 			let mut body = Vec::new();

// 			match tokens.last().unwrap() {
// 				Token::OpenBrace => {
// 					body.extend(parse_block(tokens));
// 				},
// 				_ => {
// 					body.push(parse_node(tokens).unwrap());
// 				}
// 			}

// 			expect_token(tokens, Token::CloseParen);
			
// 			Some(ASTNode::FnDef(
// 				FnDef { 
// 					params: Vec::new(), 
// 					body: body
// 				}
// 			))
// 		}
// 		_ => {
// 			Some(ASTNode::Ident(ident.to_string()))
// 		}
// 	}
// }

// fn parse_func_def(tokens: &mut Vec<Token>) -> FnDef {
// 	println!("parse_func_def");

// 	let mut fn_def = FnDef {
// 		params: Vec::new(),
// 		body: Vec::new(),
// 	};

// 	while let Some(token) = tokens.pop() {
// 		println!("token: {:?}", token);

// 		match token {
// 			Token::CloseParen => {
// 				break;
// 			},
// 			Token::Ident(ident) => {
// 				parse_ident(tokens, &ident);
// 			},
// 			_ => {
// 				todo!()
// 			}
// 		};
// 	};

// 	expect_token(tokens, Token::Arrow);

// 	// let next = tokens.pop().unwrap();

// 	// println!("next: {:?}", next);

// 	let mut braced = false;
// 	while let Some(next) = tokens.last() {
// 		match next {
// 			Token::OpenBrace => {
// 				tokens.pop();
// 				braced = true;
// 			},
// 			Token::CloseBrace => {
// 				tokens.pop();
// 				break;
// 			},
// 			_ => {
// 				let n = match parse_body_node(tokens) {
// 					Some(n) => n,
// 					None => break,
// 				};

// 				fn_def.body.push(n);

// 				if !braced {
// 					println!("breaking since not braced");

// 					break;
// 				}
// 			}
// 		};
// 	};

// 	println!("parsing fn def finished");

// 	fn_def
	
// 	// match next {
// 	// 	Token::Arrow => {
// 	// 		let next = tokens.pop().unwrap();

// 	// 		match next {
// 	// 			Token::OpenBrace => {
// 	// 				//let props: Vec<Property> = parse_object_def_properties(&mut tokens);

// 	// 				let mut fdef = FunctionDef {
// 	// 					params: Vec::new(),
// 	// 					body: Vec::new(),
// 	// 				};

// 	// 				while let Some(token) = tokens.pop() {
// 	// 					let node = match token {
// 	// 						Token::Identifier(ident) => {
// 	// 							parse_identifier(tokens, &ident)
// 	// 						},
// 	// 						Token::CloseBrace => {
// 	// 							break;
// 	// 						},
// 	// 						_ => {
// 	// 							todo!("Unexpected token: {:?}", token);
// 	// 						}
// 	// 					};

// 	// 					fdef.body.push(node);
// 	// 				}

// 	// 				Node::FunctionDef(fdef)
// 	// 			},
// 	// 			_ => {
// 	// 				todo!();
// 	// 			}
// 	// 		}
// 	// 	},
// 	// 	_ => {
// 	// 		todo!();
// 	// 	}
// 	// }
// }

// fn parse_struct(tokens: &mut Vec<Token>) -> StructDef {
// 	println!("Struct");

// 	let token = tokens.pop().unwrap();

// 	let name = match token {
// 		Token::Ident(ident) => {
// 			ident
// 		},
// 		_ => {
// 			todo!();
// 		}
// 	};

// 	expect_token(tokens, Token::OpenBrace);

// 	let mut struct_def = StructDef {
// 		name: name,
// 		fields: Vec::new(),
// 	};

// 	while let Some(token) = tokens.pop() {
// 		match token {
// 			Token::CloseBrace => {
// 				break;
// 			},
// 			Token::Ident(ident) => {
// 				println!("Identifier: {}", ident);

// 				expect_token(tokens, Token::Colon);

// 				let typ = match tokens.pop().unwrap() {
// 					Token::IntDef => VarType::Int,
// 					Token::FloatDef => VarType::Float,
// 					Token::StringDef => VarType::String,
// 					Token::Ident(ident) => VarType::Var(ident),
// 					Token::String(str) => VarType::StrLit(str),
// 					Token::OpenParen => VarType::FnDef(parse_func_def(tokens)),
// 					_ => {
// 						todo!();
// 					}
// 				};

// 				let field = TypeField {
// 					name: ident,
// 					typ: typ,
// 				};

// 				struct_def.fields.push(field);
// 			},
// 			_ => {
// 				todo!();
// 			}
// 		}
// 	}

// 	struct_def
// }

pub struct Parser {
	tokens: Vec<(Token, Span)>,
	i: usize,
	loglevel: usize,
	callstack: Vec<String>,
}

impl Parser {
	pub fn new(input: &str) -> Parser {
		let lexer = Token::lexer(input);

		Parser {
			i: 0,
			loglevel: 0,
			callstack: Vec::new(),
			tokens: lexer.spanned().map(|(token, span)| (token, span.into())).collect()
		}
	}

	// pub fn parse(&mut self) -> Vec<ASTNode> {
	// 	let lexer = Token::lexer(input);
	// 	let mut tokens: Vec<Token> = lexer.collect();
	// 	let tokens_with_spans: Vec<(Token, Span)> = lexer.spanned().map(|(token, span)| (token, span.into())).collect();
	// 	tokens.reverse();
	
	// 	println!("tokens: {:?}", tokens);
	
	// 	self.parse_block(&mut tokens)
	// }

	pub fn set_loglevel(mut self, level: usize) -> Self {
		self.loglevel = level;

		self
	}

	pub fn parse(&mut self) -> Vec<ASTNode> {
		self.parse_block()
	}

	fn peek(&self, i: usize) -> Option<Token> {
		if self.loglevel > 0 {
			self.log(&format!("peek: {} {:?}", i, self.tokens.get(self.i + i)));
		}

		match self.tokens.get(self.i + i) {
			Some((token, _)) => Some(token.clone()),
			None => None,
		}
	}

	fn eat(&mut self) -> Option<Token> {
		if self.loglevel > 0 {
			self.log(&format!("eat: {} val: {:?}", self.i, self.tokens.get(self.i)));
		}

		let token = match self.tokens.get(self.i) {
			Some((token, _)) => token.clone(),
			None => return None,
		};

		self.i += 1;

		Some(token.clone())
	}

	fn expect_eat(&mut self, token: Token) {
		if self.loglevel > 0 {
			self.log(&format!("expect_eat: {:?}", token));
		}

		let next = self.eat().unwrap();

		if next != token {
			panic!("Expected {:?} but got {:?}", token, next);
		}
	}

	fn skip(&mut self, n: usize) {
		if self.loglevel > 0 {
			self.log(&format!("skip: {} {:?} to {:?}", n, self.tokens.get(self.i), self.tokens.get(self.i + n)));
		}

		self.i += n;
	}

	fn expect_ident(&mut self) -> String {
		if self.loglevel > 0 {
			self.log(&format!("expect_ident"));
		}

		let token = self.eat().unwrap();

		match token {
			Token::Ident(ident) => ident,
			_ => panic!("Expected ident but got {:?}", token),
		}
	}

	fn log(&self, msg: &str) {
		println!("{} {}", self.callstack.join(":"), msg);
	}

	fn parse_block(&mut self) -> Vec<ASTNode> {
		if self.loglevel > 0 {
			self.callstack.push("parse_block".to_string());
		}
	
		let mut nodes = Vec::new();

		loop {
			match self.parse_item() {
				Some(n) => nodes.push(n),
				None => break,
			};
		}

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		nodes
	}

	fn parse_item(&mut self) -> Option<ASTNode> {
		if self.loglevel > 0 {
			self.callstack.push("parse_item".to_string());
		}

		let token = match self.peek(0) {
			Some(token) => token.clone(),
			None => return None,
		};

		let ret = match token {
			Token::Ident(ident) => {
				if self.loglevel > 0 {
					self.log(&format!("ident: {}", ident));
				}
				// let next = match self.peek(2) {
				// 	Some(token) => token,
				// 	None => return Some(ASTNode::Ident(ident)),
				// };

				match self.peek(1) {
					Some(Token::Assign) => {
						self.skip(2);

						let right = match self.peek(0) {
							Some(token) => match token {
								Token::OpenParen => {
									self.skip(1);

									let mut params = Vec::new();

									while let Some(token) = self.peek(0) {
										match token {
											Token::CloseParen => {
												self.skip(1);
												break;
											},
											Token::Comma => {
												self.skip(1);
											},
											Token::Ident(name) => {
												self.skip(1);
												params.push(ASTNode::Ident(name));
											},
											_ => panic!("Expected ident or ) but got {:?}", self.peek(0)),
										}
									}

									self.expect_eat(Token::Arrow);
									self.expect_eat(Token::OpenBrace);

									let mut body = Vec::new();

									while let Some(token) = self.peek(0) {
										match token {
											Token::CloseBrace => {
												self.skip(1);
												break;
											},
											_ => body.push(self.parse_item().unwrap()),
										}
									}

									let f = Fun {
										params: params,
										body: body,
									};

									ASTNode::Fun(f)
								},
								_ => self.parse_item().unwrap(),
							},
							None => panic!("Expected expression after assignment")
						};

						let a = Assign { 
							left: Box::new(ASTNode::Ident(ident.clone())), 
							right: Box::new(right)
						};


						Some(ASTNode::Assign(a))
					},
					Some(Token::Ident(name)) => {
						self.skip(2);
						Some(
							ASTNode::Var(
								Var {
									name: name.to_string(),
									typ: ident.to_string(),
								}
							)
						)
					},
					Some(Token::OpenBrace) => {
						Some(self.parse_obj_ins())
					},
					_ => {
						Some(self.parse_expr())
					}
				}
			}
			Token::OpenBracket => {
				self.skip(1);
				let mut items = Vec::new();

				while let Some(token) = self.peek(0) {
					match token {
						Token::CloseBracket => {
							self.skip(1);
							break;
						},
						_ => {
							items.push(self.parse_expr());
						}
					}
				}

				Some(ASTNode::Array(Array { items }))
			}
			Token::OpenParen => {
				let mut i = 1;

				while let Some(token) = self.peek(i) {
					i += 1;

					match token {
						Token::CloseParen => break,
						_ => {}
					}
				};
		
				Some(match self.peek(i) {
					Some(Token::Arrow) => {
						self.parse_fun()
					}
					_ => self.parse_expr()
				})
			}
			_ => Some(self.parse_expr())
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ret
	}

	fn parse_fun(&mut self) -> ASTNode {
		if self.loglevel > 0 {
			self.callstack.push("parse_fun".to_string());
		}

		self.expect_eat(Token::OpenParen);

		let mut params = Vec::new();

		while let Some(token) = self.peek(0) {
			match token {
				Token::CloseParen => {
					self.skip(1);
					break;
				},
				Token::Comma => {
					self.skip(1);
				},
				Token::Ident(name) => {
					self.skip(1);
					params.push(ASTNode::Ident(name));
				},
				_ => panic!("Expected ident or ) but got {:?}", self.peek(0)),
			}
		}

		self.expect_eat(Token::Arrow);
		self.expect_eat(Token::OpenBrace);

		let mut body = Vec::new();

		while let Some(token) = self.peek(0) {
			match token {
				Token::CloseBrace => {
					self.skip(1);
					break;
				},
				_ => body.push(self.parse_item().unwrap()),
			}
		}

		let f = Fun {
			params: params,
			body: body,
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ASTNode::Fun(f)
	}

	fn parse_obj_ins(&mut self) -> ASTNode {
		let name = self.expect_ident();

		if self.loglevel > 0 {
			self.callstack.push("parse_obj_ins".to_string());
			self.log(&format!("name: {}", name));
		}

		self.expect_eat(Token::OpenBrace);

		let mut props = Vec::new();

		loop {
			match self.peek(0) {
				Some(Token::CloseBrace) => {
					self.skip(1);
					break;
				}
				Some(Token::Comma) => {
					self.skip(1);
				}
				_ => {
					let prob_name = self.expect_ident();
					self.expect_eat(Token::Colon);

					let prob = Property {
						name: prob_name,
						value: Box::new(self.parse_item().unwrap())
					};

					props.push(prob);
				}
			}
		}

		let b = StructIns {
			name: name.to_string(),
			probs: props,
		};

		ASTNode::StructIns(b)
	}

	fn parse_expr(&mut self) -> ASTNode {
		if self.loglevel > 0 {
			self.callstack.push("parse_expr".to_string());
		}

		let left = self.parse_term();

		let next = match self.peek(0) {
			Some(t) => t,
			None => {
				if self.loglevel > 0 {
					self.callstack.pop();
				}
				return left;
			}
		};

		let ret = match next {
			Token::Plus => {
				self.log("Plus");
				self.skip(1);
				ASTNode::BinOp(
					BinOp { 
						left: Box::new(left), 
						op: Op::Plus,
						right: Box::new(self.parse_expr()) 
					}
				)
			},
			Token::Minus => {
				self.log("Minus");
				self.skip(1);
				ASTNode::BinOp(
					BinOp { 
						left: Box::new(left), 
						op: Op::Minus,
						right: Box::new(self.parse_expr()) 
					}
				)
			},
			Token::OpenParen => {
				self.log("parsing call");
				self.parse_call(left)
			},
			Token::Dot => {
				self.log("parsing dot");
				self.parse_prob_access(left)
			},
			_ => {
				left
			}
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ret
	}

	fn parse_call(&mut self, caller: ASTNode) -> ASTNode {
		self.skip(1);

		let mut args = Vec::new();

		while let Some(token) = self.peek(0) {
			match token {
				Token::CloseParen => {
					self.skip(1);
					break;
				},
				Token::Comma => {
					self.skip(1);
				},
				_ => {
					args.push(self.parse_expr());
				}
			}
		}

		self.log("call done");

		let call = ASTNode::Call(
			Call {
				callee: Box::new(caller),
				args: args,
			}
		);

		match self.peek(0) {
			Some(t) => match t {
				Token::OpenParen => {
					self.parse_call(call)
				}
				_ => call,
			},
			None => call,
		}
	}

	fn parse_prob_access(&mut self, left: ASTNode) -> ASTNode {
		self.skip(1);

		let ident = self.expect_ident();
		
		let prob_access = ASTNode::ProbAccess(
			ProbAccess {
				object: Box::new(left),
				property: ident,
			}
		);

		match self.peek(0) {
			Some(t) => match t {
				Token::OpenParen => {
					self.parse_call(prob_access)
				},
				_ => prob_access,
			},
			None => prob_access,
		}
	}

	fn parse_term(&mut self) -> ASTNode {
		if self.loglevel > 0 {
			self.callstack.push("parse_term".to_string());
		}

		let left = self.parse_factor();

		let next = match self.peek(0) {
			Some(t) => t,
			None => {
				if self.loglevel > 0 {
					self.callstack.pop();
				}
				return left;
			}
		};

		let ret = match next {
			Token::Multiply => {
				self.log("Multiply");
				self.skip(1);
				ASTNode::BinOp(
					BinOp { 
						left: Box::new(left), 
						op: Op::Multiply,
						right: Box::new(self.parse_factor()) 
					}
				)
			},
			Token::Divide => {
				self.log("Divide");
				self.skip(1);
				ASTNode::BinOp(
					BinOp { 
						left: Box::new(left), 
						op: Op::Divide,
						right: Box::new(self.parse_factor()) 
					}
				)
			},
			_ => {
				left
			}
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ret
	}

	fn parse_factor(&mut self) -> ASTNode {
		if self.loglevel > 0 {
			self.callstack.push("parse_factor".to_string());
		}

		let next = match self.eat() {
			Some(t) => t,
			None => {
				panic!("Unexpected end of tokens");
			}
		};

		let ret = match next {
			Token::Ident(ident) => {
				if self.loglevel > 0 {
					self.log(&format!("Ident: {}", ident));
				}

				ASTNode::Ident(ident.to_string())

				// let token = match self.peek(0) {
				// 	Some(t) => t,
				// 	None => {
				// 		return ASTNode::Ident(ident.to_string());
				// 	}
				// };

				// match token {
				// 	Token::OpenParen => {
				// 		Call {
				// 			callee: ident.to_string(),
				// 		}
				// 	},
				// 	_ => {
				// 		return ASTNode::Ident(ident.to_string());
				// 	}
				// }
			}
			Token::String(s) => ASTNode::LiteralString(s),
			Token::Int(num) => ASTNode::LiteralInt(num),
			Token::Decimal(num) => ASTNode::LiteralDecimal(num),
			Token::OpenParen => {
				let node = self.parse_expr();
				
				self.expect_eat(Token::CloseParen);
				return node;
			},

			_ => {
				panic!("Unexpected token {:?}", next);
			}
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ret
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let code = r#"Main {
    //         children: Window {
    //             title: "Testi Ikkuna"
    //             children: [
    //                 Box {
    //                     onClick: () => {
    //                         info("Hello world")
    //                         info("Hello world")
    //                         info("Hello world")
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
    //     }"#;

    //     parse_code(code);
    // }

    // #[test]
    // fn test_parse_struct() {
    //     let code = r#"
    //         struct Person {
    //             name: "Testi"
    //             age: Int

    //             say_hello: () => {
    //                 info("Hello world")
    //             }
    //         }
    //     "#;

    //     parse_code(code);
    // }

	// #[test]
	// fn test_parse_import() {

	// }

	// #[test]
	// fn parse_two_list_assigments() {
	// 	let code = r#"
	// 		a = [1, 2, 3]
	// 		b = [3, 2, 1]
	// 	"#;

	// 	let ast = parse_code(code);

	// 	println!("ast: {:?}", ast);

	// 	for node in ast {
	// 		println!("{}", ast_pretty_string(&node));
	// 	}
	// }

	// #[test]
	// fn parse_object_assigment() {
	// 	let code = r#"
	// 		a = {
	// 			name: "Testi"
	// 			age: 123
	// 		}
	// 	"#;

	// 	let ast = parse_code(code);

	// 	println!("ast: {:?}", ast);

	// 	for node in ast {
	// 		println!("{}", ast_pretty_string(&node));
	// 	}
	// }

	// #[test]
	// fn parse_var() {
	// 	let code = r#"TodoItem todo_item = {}"#;

	// 	let ast = parse_code(code);

	// 	println!("ast: {:?}", ast);

	// 	for node in ast {
	// 		println!("{}", ast_pretty_string(&node));
	// 	}
	// }
	
	// #[test]
	// fn parse_nested_instances() {
	// 	let code = r#"
	// 		Window {
	// 			title: "test"
	// 			children: [
	// 				Box {
	// 					onClick: () => {
	// 						info("Hello world")
	// 					}
	// 					children: [
	// 						Text {
	// 							title: "qwerty"
	// 						}
	// 					]
	// 				}
	// 			]
	// 		}
	// 	"#;

	// 	let ast = parse_code(code);

	// 	println!("ast: {:?}", ast);

	// 	for node in ast {
	// 		println!("{}", ast_pretty_string(&node));
	// 	}
	// }

	// #[test]
	// fn test_parse_fn_without_params() {
	// 	let code = r#"
	// 		foo = () => {
	// 			return "Hello world"
	// 		}
	// 	"#;

	// 	let ast = parse_code(code);

	// 	println!("ast: {:?}", ast);

	// 	for node in ast {
	// 		println!("{}", ast_pretty_string(&node));
	// 	}
	// }

	// #[test]
	// fn test_a1() {
	// 	let code = r#"
	// 		Div {
	// 			children: todos.map(todo => {})
	// 		}
	// 	"#;

	// 	let ast = parse_code(code);

	// 	for node in ast {
	// 		println!("{}", ast_pretty_string(&node));
	// 	}
	// }

	// #[test]
	// fn test_a2() {
	// 	let code = r#"
	// 		a = [
	// 			Div {
	// 				children: todos.map(todo => {})
	// 			}
	// 		]
	// 	"#;

	// 	let ast = parse_code(code);
	// 	println!("ast: {:?}", ast);

	// 	for node in ast {
	// 		println!("{}", ast_pretty_string(&node));
	// 	}
	// }

	#[test]
	fn test_simple_plus_expr() {
		let code = r#"
			a = 1 + 2
		"#;

		let ast = Parser::new(code)
			.parse();

		println!("ast: {:?}", ast);
	}

	#[test]
	fn test_simple_minus_expr() {
		let code = r#"
			a = 1 - 2
		"#;

		let ast = Parser::new(code)
			.parse();

		println!("ast: {:?}", ast);
	}

	#[test]
	fn test_simple_mul_expr() {
		let code = r#"
			a = 1 * 2
		"#;

		let ast = Parser::new(code)
			.set_loglevel(1)
			.parse();

		println!("ast: {:?}", ast);
	}

	#[test]
	fn test_simple_div_expr() {
		let code = r#"
			a = 1 / 2
		"#;

		let ast = Parser::new(code)
			.set_loglevel(1)
			.parse();

		println!("ast: {:?}", ast);
	}

	#[test]
	fn test_paren_expr() {
		let code = r#"
			a = (1 + 2) * 3
		"#;

		let ast = Parser::new(code)
			.parse();

		println!("ast: {:?}", ast);
	}

	#[test]
	fn test_expr_ordering() {
		let code = r#"
			a = 1 + 2 * 3
		"#;

		let ast = Parser::new(code)
			.parse();

		println!("ast: {:?}", ast);
	}

	#[test]
	fn test_call_without_args() {
		let code = r#"
			a = foo()
		"#;

		let ast = Parser::new(code)
			.parse();

		println!("ast: {:?}", ast);
	}

	#[test]
	fn test_call_with_num_arg() {
		let code = r#"
			a = foo(1)
		"#;

		let ast = Parser::new(code)
			.parse();

		println!("ast: {:?}", ast);
	}

	#[test]
	fn test_double_call() {
		let code = r#"
			a = foo(1)(2)
		"#;

		let ast = Parser::new(code)
			.set_loglevel(1)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::Call(
							Call {
								callee: Box::new(
									ASTNode::Call(
										Call {
											callee: Box::new(ASTNode::Ident("foo".to_string())),
											args: vec![
												ASTNode::LiteralInt(1),
											],
										}
									)
								),
								args: vec![
									ASTNode::LiteralInt(2),
								],
							}

						)
					),
				},
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_prob_access() {
		let code = r#"
			foo.bar
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::ProbAccess(
				ProbAccess {
					object: Box::new(ASTNode::Ident("foo".to_string())),
					property: "bar".to_string(),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_method_call() {
		let code = r#"
			foo.bar(1)
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Call(
				Call {
					callee: Box::new(
						ASTNode::ProbAccess(
							ProbAccess {
								object: Box::new(ASTNode::Ident("foo".to_string())),
								property: "bar".to_string(),
							}
						)
					),
					args: vec![
						ASTNode::LiteralInt(1),
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_empty_array() {
		let code = r#"
			l = []
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("l".to_string())),
					right: Box::new(
						ASTNode::Array(
							Array {
								items: vec![],
							}
						)
					),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_empty_named_instance() {
		let code = r#"
			Ball {}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::StructIns(
				StructIns {
					name: "Ball".to_string(),
					probs: vec![],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_named_instance_fields() {
		let code = r#"
			Ball {
				x: 1,
				y: 2,
				name: "nakki"
			}
		"#;

		let ast = Parser::new(code)
			.set_loglevel(1)
			.parse();

		let expected = vec![
			ASTNode::StructIns(
				StructIns {
					name: "Ball".to_string(),
					probs: vec![
						Property {
							name: "x".to_string(),
							value: Box::new(ASTNode::LiteralInt(1)),
						},
						Property {
							name: "y".to_string(),
							value: Box::new(ASTNode::LiteralInt(2)),
						},
						Property {
							name: "name".to_string(),
							value: Box::new(ASTNode::LiteralString("nakki".to_string())),
						},
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_fun_empty_param_and_body() {
		let code = r#"
			foo = () => {}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("foo".to_string())),
					right: Box::new(
						ASTNode::Fun(
							Fun {
								params: vec![],
								body: vec![],
							}
						)
					),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_simple_fun() {
		let code = r#"
			foo = (a, b) => {
				a + b
			}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("foo".to_string())),
					right: Box::new(
						ASTNode::Fun(
							Fun {
								params: vec![
									ASTNode::Ident("a".to_string()),
									ASTNode::Ident("b".to_string()),
								],
								body: vec![
									ASTNode::BinOp(
										BinOp {
											op: Op::Plus,
											left: Box::new(ASTNode::Ident("a".to_string())),
											right: Box::new(ASTNode::Ident("b".to_string())),
										}
									)
								],
							}
						)
					),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_obj_field_fun() {
		let code = r#"
			Div {
				on_click: () => {}
			}
		"#;

		let ast = Parser::new(code)
			.set_loglevel(1)
			.parse();

		let expected = vec![
			ASTNode::StructIns(
				StructIns {
					name: "Div".to_string(),
					probs: vec![
						Property {
							name: "on_click".to_string(),
							value: Box::new(
								ASTNode::Fun(
									Fun {
										params: vec![],
										body: vec![],
									}
								)
							),
						},
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}
}





	// fn parse_body(&mut self, tokens: &mut Vec<Token>) -> Vec<ASTNode> {
	// 	println!("parse body");
	
	// 	let mut nodes = Vec::new();
	
	// 	while let Some(token) = tokens.last() {
	// 		match token {
	// 			Token::CloseBrace => {
	// 				println!("brace closed stop parsing body");
	
	// 				tokens.pop();
	// 				break;
	// 			},
	// 			_ => {
	// 				println!("parse body node");
	
	// 				match self.parse_body_node(tokens) {
	// 					Some(n) => nodes.push(n),
	// 					None => break,
	// 				}
	// 			}
	// 		}
	// 	}
	
	// 	println!("parsing body done");
	
	// 	nodes
	// }

	// fn parse_body_node(&mut self, tokens: &mut Vec<Token>) -> Option<ASTNode> {
	// 	println!("parse_body_node");
	
	// 	let next = tokens.pop().unwrap();
	
	// 	match next {
	// 		Token::Ident(ident) => {
	// 			println!("ident: {}", ident);
	
	// 			let next = match tokens.pop() {
	// 				Some(n) => n,
	// 				None => {
	// 					return Some(ASTNode::Ident(ident.to_string()));
	// 				}
	// 			};
	
	// 			match next {
	// 				Token::Ident(var_name) => {
	// 					println!("var name: {}", var_name);
	
	// 					let var = Var {
	// 						name: var_name,
	// 						typ: ident,
	// 					};
	// 					let next = tokens.last().unwrap();
	
	// 					match next {
	// 						Token::Assign => {
	// 							tokens.pop();
	// 							Some(ASTNode::Assign(Assign {
	// 								left: Box::new(ASTNode::Var(var)),
	// 								right: Box::new(parse_node(tokens).unwrap()),
	// 							}))
	// 						},
	// 						_ => None
	// 					}
	// 				},
	// 				Token::OpenBrace => {
	// 					let props: Vec<Property> = parse_obj_probs(tokens);
			
	// 					Some(ASTNode::StructIns(
	// 						StructIns {
	// 							name: ident.to_string(),
	// 							properties: props,
	// 						}
	// 					))
	// 				},
	// 				Token::Assign => {
	// 					println!("assign");
	
	// 					Some(ASTNode::Assign(Assign {
	// 						left: Box::new(ASTNode::Ident(ident.to_string())),
	// 						right: Box::new(parse_node(tokens).unwrap()),
	// 					}))
	// 				},
	// 				Token::OpenParen => {
	// 					let mut call = Call {
	// 						callee: Box::new(ASTNode::Ident(ident.to_string())),
	// 						arguments: Vec::new(),
	// 					};
			
	// 					while let Some(next) = tokens.last() {
	// 						match next {
	// 							Token::CloseParen => {
	// 								tokens.pop();
	// 								break;
	// 							},
	// 							Token::Comma => {
	// 								tokens.pop();
	// 							},
	// 							_ => {
	// 								call.arguments.push(parse_node(tokens).unwrap());
	// 							}
	// 						}
	// 					};
			
	// 					Some(ASTNode::Call(call))
	// 				},
	// 				Token::Dot => {
	// 					Some(ASTNode::ProbAccess(ProbAccess {
	// 						object: Box::new(ASTNode::Ident(ident.to_string())),
	// 						property: Box::new(parse_node(tokens).unwrap()),
	// 					}))
	// 				},
	// 				_ => {
	// 					todo!("Unexpected token: {:?}", next);
	// 				}
	// 			}
	// 		},
	// 		Token::Type => {
	// 			println!("type");
	
	// 			let ident = expect_identifier(tokens);
	
	// 			expect_token(tokens, Token::OpenBrace);
	
	// 			let mut type_def = TypeDef {
	// 				name: ident,
	// 				fields: Vec::new(),
	// 			};
				
	// 			while let Some(token) = tokens.pop() {
	// 				match token {
	// 					Token::CloseBrace => {
	// 						break;
	// 					},
	// 					Token::Ident(ident) => {
	// 						println!("Identifier: {}", ident);
	
	// 						expect_token(tokens, Token::Colon);
	
	// 						let typ = match tokens.pop().unwrap() {
	// 							Token::IntDef => VarType::Int,
	// 							Token::FloatDef => VarType::Float,
	// 							Token::StringDef => VarType::String,
	// 							Token::Ident(ident) => VarType::Var(ident),
	// 							Token::String(str) => VarType::StrLit(str),
	// 							Token::OpenParen => VarType::FnDef(parse_func_def(tokens)),
	// 							_ => {
	// 								todo!();
	// 							}
	// 						};
	
	// 						let field = TypeField {
	// 							name: ident,
	// 							typ: typ,
	// 						};
	
	// 						type_def.fields.push(field);
	// 					},
	// 					_ => {
	// 						todo!();
	// 					}
	// 				}
	// 			}
	
	// 			Some(ASTNode::TypeDef(type_def))
	// 		},
	// 		Token::Return => {
	// 			let node = parse_node(tokens).unwrap();
	
	// 			Some(ASTNode::Ret(Box::new(node)))
	// 		},
	// 		Token::Struct => Some(ASTNode::StructDef(parse_struct(tokens))),
	// 		_ => {
	// 			todo!("Unexpected token: {:?}", next);
	// 		}
	// 	}
	// }

	// fn parse_node(&mut self, tokens: &mut Vec<Token>) -> Option<ASTNode> {
	// 	let next = tokens.last().unwrap().clone();
	
	// 	println!("parse_node {:?}", next);
	
	// 	match next {
	// 		Token::Ident(ident) => {
	// 			tokens.pop();
	// 			println!("node is identifier: {:?}", ident);
	// 			parse_ident(tokens, &ident)
	// 		},
	// 		Token::String(str) => {
	// 			tokens.pop();
	// 			println!("LiteralString: {:?}", str);
	// 			Some(ASTNode::LiteralString(str.to_string()))
	// 		},
	// 		Token::Int(i) => {
	// 			tokens.pop();
	// 			println!("LiteralInt: {:?}", i);
	// 			Some(ASTNode::LiteralInt(i))
	// 		},
	// 		Token::Decimal(d) => {
	// 			tokens.pop();
	// 			Some(ASTNode::LiteralDecimal(d))
	// 		},
	// 		Token::CloseBrace => {
	// 			None
	// 		},
	// 		Token::OpenBracket => {
	// 			println!("parse array");
	// 			tokens.pop();
	
	// 			let mut items = Vec::new();
	
	// 			while let Some(token) = tokens.last() {
	// 				match token {
	// 					Token::CloseBracket => {
	// 						println!("CloseBracket");
	
	// 						tokens.pop();
	// 						break;
	// 					},
	// 					Token::Comma => {
	// 						println!("Comma");
	
	// 						tokens.pop();
	// 					},
	// 					_ => {
	// 						println!("parse array item");
	// 						let node = parse_node(tokens).unwrap();
	// 						items.push(node);
	// 					}
	// 				}
	// 			}
	
	// 			let array = Array {
	// 				items: items,
	// 			};
	
	// 			println!("parse array done");
	
	// 			Some(ASTNode::Array(array))
	// 		},
	// 		Token::OpenParen => {
	// 			tokens.pop();
	// 			Some(ASTNode::FnDef(parse_func_def(tokens)))
	// 		},
	// 		Token::Struct => {
	// 			tokens.pop();
	// 			Some(ASTNode::StructDef(parse_struct(tokens)))
	// 		},
	// 		Token::CloseParen => None,
	// 		Token::OpenBrace => {
	// 			tokens.pop();
	// 			Some(ASTNode::Obj(Obj{
	// 				properties: parse_obj_probs(tokens),
	// 			}))
	// 		},
	// 		_ => {
	// 			todo!("Unexpected token: {:?}", next);
	// 		}
	// 	}
	// }
