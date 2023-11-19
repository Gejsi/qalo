use std::error::Error;

use qalo::{evaluator::Evaluator, lexer::Lexer, object::Object, parser::Parser, token::TokenKind};

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let a = [100, 200, 300, 400];
        1 + !-a[10 + 20 * 30];
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
    let program = parser.parse_program().unwrap();
    println!("{program:#?}");

    // let mut evaluator = Evaluator::new(input);
    // for obj in evaluator.eval_program()? {
    //     println!("{obj}");
    // }

    Ok(())
}
