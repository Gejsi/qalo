use std::error::Error;

use jerboa::parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        return 1;

        { let a = 2; }

        { 2 + 2; }

        let b = a;
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
    println!("{:#?}", res);

    Ok(())
}
