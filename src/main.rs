use std::error::Error;

use qalo::{evaluator::Evaluator, lexer::Lexer, object::Object, parser::Parser, token::TokenKind};

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let map = fn(arr, f) {
            let iter = fn(arr, accumulated) {
                if (len(arr) == 0) {
                    accumulated
                } else {
                    iter(rest(arr), append(accumulated, f(arr[0])));
                }
            };

            iter(arr, []);
        };

        let arr = [1, 2, 3, 4];
        let double = fn(x) { x * 2 };
        map(arr, double);
    "#;

    // let mut lexer = Lexer::new(input);
    // loop {
    //     let token = lexer.next_token();
    //     println!("{token:?}");

    //     if token.kind == TokenKind::Eof {
    //         break;
    //     }
    // }

    // let mut parser = Parser::new(input);
    // let program = parser.parse_program()?;
    // println!("{program}");

    let mut evaluator = Evaluator::new(input);
    for obj in evaluator.eval_program()? {
        println!("{obj:?}");
    }

    Ok(())
}
