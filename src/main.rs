use std::error::Error;

use qalo::{evaluator::Evaluator, lexer::Lexer, object::Object, parser::Parser, token::TokenKind};

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let reduce = fn(arr, initial, f) {
            let iter = fn(arr, result) {
                if len(arr) == 0 {
                    result
                } else {
                    iter(rest(arr), f(result, arr[0]));
                }
            };

            iter(arr, initial);
        };

        let sum = fn(arr) {
            return reduce(arr, 0, fn(initial, el) { initial + el });
        };

        sum([1, 2, 3, 4, 5]);
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
