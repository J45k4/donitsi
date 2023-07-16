
// #[cfg(test)]
// mod tests {
//     use std::vec;

//     use crate::parser::*;

//     #[test]
//     fn parse_simple_typedef() {
//         let code = r#"Main {
//         }"#;
 
//         let ast = parse_code(code);

//         assert_eq!(ast.len(), 1);
//         assert_eq!(ast, vec![
//             ASTNode::StructIns(
//                 StructIns { 
//                     name: "Main".to_string(), 
//                     properties: vec![]
//                 }
//             ),
//         ]);            
//     }

//     #[test]
//     fn parse_simple_typedef_with_properties() {
//         let code = r#"Main {
//             name: "Teppo"
//             age: 24
//         }"#;

//         let ast = parse_code(code);

//         assert_eq!(ast.len(), 1);
//         assert_eq!(ast, vec![
//             ASTNode::StructIns(
//                 StructIns { 
//                     name: "Main".to_string(), 
//                     properties: vec![
//                         Property {
//                             name: "name".to_string(),
//                             value: Box::new(
//                                 ASTNode::LiteralString("Teppo".to_string())
//                             ),
//                         },
//                         Property {
//                             name: "age".to_string(),
//                             value: Box::new(
//                                 ASTNode::LiteralInt(24)
//                             ),
//                         },
//                     ]
//                 }
//             ),
//         ]);            
//     }

//     #[test]
//     fn parse_typedef_with_more_complicated_properties() {
//         let code = r#"Main {
//             children: Player {
//                 name: "Teppo"
//                 age: 24
//                 children: [
//                     RigidBody {
//                         shape: Box {
//                             width: 100
//                             height: 100
//                             depth: 100
//                         }
//                     }
//                 ]
//             }
//         }"#;

//         let ast = parse_code(code);

//         assert_eq!(ast, vec![
//             ASTNode::StructIns(
//                 StructIns { 
//                     name: "Main".to_string(), 
//                     properties: vec![
//                         Property {
//                             name: "children".to_string(),
//                             value: Box::new(
//                                 ASTNode::StructIns(
//                                     StructIns { 
//                                         name: "Player".to_string(), 
//                                         properties: vec![
//                                             Property {
//                                                 name: "name".to_string(),
//                                                 value: Box::new(
//                                                     ASTNode::LiteralString("Teppo".to_string())
//                                                 ),
//                                             },
//                                             Property {
//                                                 name: "age".to_string(),
//                                                 value: Box::new(
//                                                     ASTNode::LiteralInt(24)
//                                                 ),
//                                             },
//                                             Property {
//                                                 name: "children".to_string(),
//                                                 value: Box::new(
//                                                     ASTNode::Array(
//                                                         Array { 
//                                                             items: vec![
//                                                                 ASTNode::StructIns(
//                                                                     StructIns { 
//                                                                         name: "RigidBody".to_string(), 
//                                                                         properties: vec![
//                                                                             Property {
//                                                                                 name: "shape".to_string(),
//                                                                                 value: Box::new(
//                                                                                     ASTNode::StructIns(
//                                                                                         StructIns { 
//                                                                                             name: "Box".to_string(), 
//                                                                                             properties: vec![
//                                                                                                 Property {
//                                                                                                     name: "width".to_string(),
//                                                                                                     value: Box::new(
//                                                                                                         ASTNode::LiteralInt(100)
//                                                                                                     ),
//                                                                                                 },
//                                                                                                 Property {
//                                                                                                     name: "height".to_string(),
//                                                                                                     value: Box::new(
//                                                                                                         ASTNode::LiteralInt(100)
//                                                                                                     ),
//                                                                                                 },
//                                                                                                 Property {
//                                                                                                     name: "depth".to_string(),
//                                                                                                     value: Box::new(
//                                                                                                         ASTNode::LiteralInt(100)
//                                                                                                     ),
//                                                                                                 },
//                                                                                             ]
//                                                                                         }
//                                                                                     )
//                                                                                 ),
//                                                                             },
//                                                                         ]
//                                                                     }
//                                                                 )
//                                                             ]
//                                                         }
//                                                     )
//                                                 ),
//                                             },
//                                         ]
//                                     }
//                                 )
//                             )
//                         }
//                     ]
//                 }
//             )
//         ]);
//     }

//     #[test]
//     fn parse_arrow_function() {
//         let code = r#"foo = () => info("hello")"#;

//         let ast = parse_code(code);

//         assert_eq!(ast, vec![
//             ASTNode::Assign(
//                 Assign {
//                     left: Box::new(
//                         ASTNode::Ident("foo".to_string())
//                     ),
//                     right: Box::new(
//                         ASTNode::Lambda(
//                             Lambda { 
//                                 params: vec![], 
//                                 body: vec![
//                                     ASTNode::Call(
//                                         Call { 
//                                             callee: Box::new(
//                                                 ASTNode::Ident("info".to_string())
//                                             ), 
//                                             arguments: vec![
//                                                 ASTNode::LiteralString("hello".to_string())
//                                             ] 
//                                         }
//                                     )
//                                 ]
//                             }
//                         )
//                     )
//                 }
//             )
//         ]);
//     }
// }