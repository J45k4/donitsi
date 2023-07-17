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
	Ret,
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
	Str(String),
	Int(i64),
	Float(f64),
	LiteralPercent(f64),
	Fun(Fun),
	StructDef(StructDef),
	TypeDef(TypeDef),
	Var(Var),
	ProbAccess(ProbAccess),
	Obj(Obj),
	Ret(Ret),
	BinOp(BinOp)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ret {
	pub value: Box<Option<ASTNode>>
}

pub struct Parser {
	tokens: Vec<(Token, Span)>,
	i: usize,
	loglevel: usize,
	callstack: Vec<String>,
	input: String
}

impl Parser {
	pub fn new(input: &str) -> Parser {
		let lexer = Token::lexer(input);

		Parser {
			input: input.to_string(),
			i: 0,
			loglevel: 0,
			callstack: Vec::new(),
			tokens: lexer.spanned().map(|(token, span)| (token, span.into())).collect()
		}
	}

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

	fn peek_unwrap(&self, i: usize) -> Token {
		match self.peek(i) {
			Some(token) => token,
			None => {
				println!("{}", self.curr_loc());
				panic!("Unexpected end of input");
			},
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

	// Return the current location in the source code
	// takes few lines of context from both sides
	fn curr_loc(&self) -> String {
		let mut start = self.i;
		let mut end = self.i;

		for _ in 0..3 {
			if start > 0 {
				start -= 1;
			}
		}

		for _ in 0..3 {
			if end < self.tokens.len() {
				end += 1;
			}
		}

		let min = if start > 0 {
			self.tokens.get(start).unwrap().1.start
		} else {
			0
		};
		let max = if end < self.tokens.len() {
			self.tokens.get(end).unwrap().1.end
		} else {
			self.input.len()
		};

		let text = self.input.get(min..max).unwrap();

		text.to_string()
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

				match self.peek(1) {
					Some(Token::Assign) => {
						self.skip(2);

						let a = Assign { 
							left: Box::new(ASTNode::Ident(ident.clone())), 
							right: Box::new(self.parse_item().unwrap())
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
					Some(Token::Arrow) => {
						Some(self.parse_fun())
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
						Token::Comma => {
							self.skip(1);
						},
						_ => {
							items.push(self.parse_item().unwrap());
						}
					}
				}

				Some(ASTNode::Array(Array { items }))
			}
			Token::OpenParen => {
				// In here we check if future tokens contain an close paren and an arrow
				// If so, we parse a function, otherwise we parse an expression
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
			Token::Ret => {
				self.skip(1);
				
				Some(ASTNode::Ret(Ret {
					value: Box::new(self.parse_item()),
				}))
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

		let next = match self.peek(0) {
			Some(token) => token,
			None => {
				println!("{}", self.curr_loc());
				panic!("Expected token but got None")
			},
		};

		let mut params = Vec::new();

		match next {
			Token::OpenParen => {
				self.skip(1);

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
				
			}
			Token::Ident(idt) => {
				self.skip(1);
				params.push(ASTNode::Ident(idt));
			}
			_ => {
				println!("{}", self.curr_loc());
				panic!("Expected ( or ident but got {:?}", next);
			}
		}

		self.expect_eat(Token::Arrow);

		let next = self.peek_unwrap(0);

		let mut body = Vec::new();

		match next {
			Token::OpenBrace => {
				self.skip(1);
				while let Some(token) = self.peek(0) {
					match token {
						Token::CloseBrace => {
							self.skip(1);
							break;
						},
						_ => body.push(self.parse_item().unwrap()),
					}
				}
			},
			_ => {
				body.push(self.parse_item().unwrap());
			}
		}

		// self.expect_eat(Token::OpenBrace);

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
				if self.loglevel > 0 {
					self.log("Plus");
				}
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
				if self.loglevel > 0 {
					self.log("Minus");
				}
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
				self.parse_call(left)
			},
			Token::Dot => {
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
		if self.loglevel > 0 {
			self.callstack.push("parse_call".to_string());
		}

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
					args.push(self.parse_item().unwrap());
				}
			}
		}

		if self.loglevel > 0 {
			self.log("call done");
		}

		let call = ASTNode::Call(
			Call {
				callee: Box::new(caller),
				args: args,
			}
		);

		let ret = match self.peek(0) {
			Some(t) => match t {
				Token::OpenParen => {
					self.parse_call(call)
				}
				_ => call,
			},
			None => call,
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ret
	}

	fn parse_prob_access(&mut self, left: ASTNode) -> ASTNode {
		if self.loglevel > 0 {
			self.callstack.push("parse_prob_access".to_string());
		}

		self.skip(1);

		let ident = self.expect_ident();
		
		let prob_access = ASTNode::ProbAccess(
			ProbAccess {
				object: Box::new(left),
				property: ident,
			}
		);

		let ret = match self.peek(0) {
			Some(t) => match t {
				Token::OpenParen => {
					self.parse_call(prob_access)
				},
				_ => prob_access,
			},
			None => prob_access,
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ret
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
				if self.loglevel > 0 {
					self.log("Multiply");
				}
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
				if self.loglevel > 0 {
					self.log("Divide");
				}
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
			}
			Token::String(s) => ASTNode::Str(s),
			Token::Int(num) => ASTNode::Int(num),
			Token::Decimal(num) => ASTNode::Float(num),
			Token::OpenParen => {
				let node = self.parse_expr();
				
				self.expect_eat(Token::CloseParen);
				return node;
			},
			_ => {
				println!("{}", self.curr_loc());
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

	#[test]
	fn test_simple_plus_expr() {
		let code = r#"
			a = 1 + 2
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::BinOp(
							BinOp {
								left: Box::new(ASTNode::Int(1)),
								op: Op::Plus,
								right: Box::new(ASTNode::Int(2)),
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_simple_minus_expr() {
		let code = r#"
			a = 1 - 2
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::BinOp(
							BinOp {
								left: Box::new(ASTNode::Int(1)),
								op: Op::Minus,
								right: Box::new(ASTNode::Int(2)),
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_simple_mul_expr() {
		let code = r#"
			a = 1 * 2
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::BinOp(
							BinOp {
								left: Box::new(ASTNode::Int(1)),
								op: Op::Multiply,
								right: Box::new(ASTNode::Int(2)),
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_simple_div_expr() {
		let code = r#"
			a = 1 / 2
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::BinOp(
							BinOp {
								left: Box::new(ASTNode::Int(1)),
								op: Op::Divide,
								right: Box::new(ASTNode::Int(2)),
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_paren_expr() {
		let code = r#"
			a = (1 + 2) * 3
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::BinOp(
							BinOp {
								left: Box::new(
									ASTNode::BinOp(
										BinOp {
											left: Box::new(ASTNode::Int(1)),
											op: Op::Plus,
											right: Box::new(ASTNode::Int(2)),
										}
									)
								),
								op: Op::Multiply,
								right: Box::new(ASTNode::Int(3)),
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_expr_ordering() {
		let code = r#"
			a = 1 + 2 * 3
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::BinOp(
							BinOp {
								left: Box::new(ASTNode::Int(1)),
								op: Op::Plus,
								right: Box::new(
									ASTNode::BinOp(
										BinOp {
											left: Box::new(ASTNode::Int(2)),
											op: Op::Multiply,
											right: Box::new(ASTNode::Int(3)),
										}
									)
								),
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_call_without_args() {
		let code = r#"
			a = foo()
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::Call(
							Call {
								callee: Box::new(ASTNode::Ident("foo".to_string())),
								args: vec![],
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_call_with_num_arg() {
		let code = r#"
			a = foo(1)
		"#;

		let ast = Parser::new(code)
			.parse();

		let expeted = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::Call(
							Call {
								callee: Box::new(ASTNode::Ident("foo".to_string())),
								args: vec![
									ASTNode::Int(1),
								],
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expeted);
	}

	#[test]
	fn test_double_call() {
		let code = r#"
			a = foo(1)(2)
		"#;

		let ast = Parser::new(code)
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
												ASTNode::Int(1),
											],
										}
									)
								),
								args: vec![
									ASTNode::Int(2),
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
						ASTNode::Int(1),
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_call_with_callback() {
		let code = r#"
			foo(() => 5)
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Call(
				Call {
					callee: Box::new(ASTNode::Ident("foo".to_string())),
					args: vec![
						ASTNode::Fun(
							Fun {
								params: vec![],
								body: vec![
									ASTNode::Int(5),
								],
							}
						),
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
	fn test_array_with_many_numbers() {
		let code = r#"
			l = [1, 2, 3]
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
								items: vec![
									ASTNode::Int(1),
									ASTNode::Int(2),
									ASTNode::Int(3),
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
			.parse();

		let expected = vec![
			ASTNode::StructIns(
				StructIns {
					name: "Ball".to_string(),
					probs: vec![
						Property {
							name: "x".to_string(),
							value: Box::new(ASTNode::Int(1)),
						},
						Property {
							name: "y".to_string(),
							value: Box::new(ASTNode::Int(2)),
						},
						Property {
							name: "name".to_string(),
							value: Box::new(ASTNode::Str("nakki".to_string())),
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

	#[test]
	fn test_fun_without_paren() {
		let code = r#"
			foo = a => {
				a + 1
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
								],
								body: vec![
									ASTNode::BinOp(
										BinOp {
											op: Op::Plus,
											left: Box::new(ASTNode::Ident("a".to_string())),
											right: Box::new(ASTNode::Int(1)),
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
	fn test_fun_without_block() {
		let code = r#"
			foo = a => a + 1
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
								],
								body: vec![
									ASTNode::BinOp(
										BinOp {
											op: Op::Plus,
											left: Box::new(ASTNode::Ident("a".to_string())),
											right: Box::new(ASTNode::Int(1)),
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
	fn test_return_expr() {
		let code = r#"
			return 1 + 5
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Ret(
				Ret {
					value: Box::new(
						Some(
							ASTNode::BinOp(
								BinOp {
									op: Op::Plus,
									left: Box::new(ASTNode::Int(1)),
									right: Box::new(ASTNode::Int(5)),
								}
							)
						)
					),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_empty_return() {
		let code = r#"
			return
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Ret(
				Ret {
					value: Box::new(None),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_parse_obj_instance_in_array() {
		let code = r#"
			[
				Div { }
			]
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Array(
				Array {
					items: vec![
						ASTNode::StructIns(
							StructIns {
								name: "Div".to_string(),
								probs: vec![],
							}
						)
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_parse_vertex() {
		let code = r#"
			Vertex { x: -0.6, y: 0.1, color: "black" }
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::StructIns(
				StructIns {
					name: "Vertex".to_string(),
					probs: vec![
						Property {
							name: "x".to_string(),
							value: Box::new(ASTNode::Float(-0.6)),
						},
						Property {
							name: "y".to_string(),
							value: Box::new(ASTNode::Float(0.1)),
						},
						Property {
							name: "color".to_string(),
							value: Box::new(ASTNode::Str("black".to_string())),
						},
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}
}
