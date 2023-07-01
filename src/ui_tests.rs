
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

        let ast = parse_code(code);

        println!("{:#?}", ast);
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

        let ast = parse_code(code);

        println!("{:#?}", ast);
    }
}