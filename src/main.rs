use std::error::Error;

use qalo::{evaluator::Evaluator, lexer::Lexer, object::Object, parser::Parser, token::TokenKind};

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        {"temp": 1+1, "foo": true, };
    "#;

    // let mut lexer = Lexer::new(input);
    // loop {
    //     let token = lexer.next_token();
    //     println!("{token:?}");

    //     if token.kind == TokenKind::Eof {
    //         break;
    //     }
    // }

    let mut parser = Parser::new(input);
    let program = parser.parse_program()?;
    println!("{program:#?}");

    // let mut evaluator = Evaluator::new(input);
    // for obj in evaluator.eval_program()? {
    //     println!("{obj:?}");
    // }

    Ok(())
}
