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
    #[regex(r"[A-Za-z]+", |t| t.slice().to_string())]
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
    FunctionDef(String, Vec<Node>),
}

// fn parse_function_definition(tokens: &mut Vec<Token>) -> Node {
//     let name = expect_identifier(tokens);
//     expect_token(tokens, Token::Equal);
//     expect_token(tokens, Token::OpenParen);
//     expect_token(tokens, Token::CloseParen);
//     expect_token(tokens, Token::Arrow);
//     let body = parse_value(tokens);
//     Node::FunctionDef(name, vec![body])
// }

// fn parse_for_loop(tokens: &mut Vec<Token>) -> Node {
//     expect_token(tokens, Token::OpenBrace);
//     let body = parse_properties(tokens);
//     expect_token(tokens, Token::CloseBrace);
//     Node::ForLoop(Box::new(Node::Object(String::new(), Vec::new())), Box::new(Node::Array(body)))
// }

// fn parse_type_definitions(tokens: &mut Vec<Token>) -> Vec<Node> {
//     let mut nodes = Vec::new();
//     while !tokens.is_empty() {
//         nodes.push(parse_type_definition(tokens));
//     }
//     nodes
// }

// fn parse_type_definition(tokens: &mut Vec<Token>) -> Node {
//     let name = expect_identifier(tokens);
//     expect_token(tokens, Token::OpenBrace);
//     let properties = parse_properties(tokens);
//     expect_token(tokens, Token::CloseBrace);
//     Node::TypeDef(name, properties)
// }

// fn parse_properties(tokens: &mut Vec<Token>) -> Vec<Node> {
//     let mut nodes = Vec::new();
//     while tokens.last() != Some(&Token::CloseBrace) {
//         nodes.push(parse_property(tokens));
//     }
//     nodes
// }

// fn parse_property(tokens: &mut Vec<Token>) -> Node {
//     let name = expect_identifier(tokens);
//     expect_token(tokens, Token::Colon);
//     let value = parse_value(tokens);
//     Node::Property(name, Box::new(value))
// }


// fn parse_value(tokens: &mut Vec<Token>) -> Node {
//     match tokens.pop() {
//         Some(Token::Identifier(ident)) if tokens.last() == Some(&Token::OpenParen) => {
//             tokens.push(Token::Identifier(ident));
//             parse_function_definition(tokens)
//         },
//         Some(Token::For) => parse_for_loop(tokens),
//         Some(Token::Identifier(ident)) => {
//             if tokens.last() == Some(&Token::OpenBrace) {
//                 tokens.pop();
//                 let properties = parse_properties(tokens);
//                 expect_token(tokens, Token::CloseBrace);
//                 Node::Object(ident, properties)
//             } else {
//                 Node::Object(ident, Vec::new())
//             }
//         }
//         Some(Token::OpenBracket) => {
//             let mut items = Vec::new();
//             while tokens.last() != Some(&Token::CloseBracket) {
//                 items.push(parse_value(tokens));
//             }
//             expect_token(tokens, Token::CloseBracket);
//             Node::Array(items)
//         }
//         Some(Token::String(str)) => {
//             Node::LiteralString(str)
//         }
//         Some(Token::Int(i)) => {
//             Node::LiteralInt(i)
//         }
//         _ => panic!("Unexpected token while parsing value"),
//     }
// }

fn expect_token(tokens: &mut Vec<Token>, expected: Token) {
    let token = tokens.pop();
    assert_eq!(token, Some(expected.clone()), "Expected {:?}, got {:?}", expected, token);
}

// fn expect_identifier(tokens: &mut Vec<Token>) -> String {
//     let token = tokens.pop();
//     if let Some(Token::Identifier(ident)) = token {
//         ident
//     } else {
//         panic!("Expected identifier, got {:?}", token);
//     }
// }

fn parse_object_def_properties(tokens: &mut Vec<Token>) -> Vec<Property> {
    expect_token(tokens, Token::OpenBrace);

    let mut properties = Vec::new();

    while let Some(token) = tokens.pop() {
        match token {
            Token::CloseBrace => {
                break;
            },
            Token::Identifier(ident) => {
                expect_token(tokens, Token::Colon);
                let value = parse_node(tokens);
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

fn parse_node(tokens: &mut Vec<Token>) -> Node {

}

pub fn parse_code(input: &str) -> Vec<Node> {
    let lexer = Token::lexer(input);
    let mut tokens: Vec<Token> = lexer.collect();
    tokens.reverse();

    let mut ast = Vec::new();

    while let Some(token) = tokens.last() {
        match token {
            Token::Identifier(ident) => {
                tokens.pop();

                let next_token = match tokens.last() {
                    Some(token) => token,
                    None => panic!("Unexpected end of file"),
                };

                match next_token {
                    Token::OpenBrace => {
                        let props = parse_object_def_properties(&mut tokens);

                        ast.push(
                            Node::ObjectDef(
                                ObjectDef {
                                    name: ident.clone(),
                                    properties: props,
                                }
                            )
                        );
                    },
                    Token::Equal => {

                    },
                    _ => {
                        panic!("Unexpected token: {:?}", next_token);
                    }
                }
            }
            _ => {
                panic!("Unexpected token: {:?}", token);
            }
        }
    }

    ast
}