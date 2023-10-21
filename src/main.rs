use std::error::Error;

use jerboa::parser::Parser;

/*
To run with auto-reload:
cargo watch -w src -x run -c
*/
fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let a = add(first: a + b + c * d / f + g);
    "#;

    // let mut lexer = Lexer::new(&input);

    // loop {
    //     let token = lexer.next_token();
    //     println!("{:?}", token);

    //     if token.kind == TokenKind::Eof {
    //         break;
    //     }
    // }

    let mut parser = Parser::new(&input);
    let res = parser.parse_program()?;
    println!("{:#?}", res.to_string());

    Ok(())
}
