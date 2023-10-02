use jerboa::{lexer::Lexer, token::TokenKind};
use std::error::Error;

// To run with auto-reload:
// cargo watch -w src -s "cargo run" -c
fn main() -> Result<(), Box<dyn Error>> {
    let input = r##"
        "foo    
        bar"
        [1, 2];

        {"foo": "bar", "fiz": [1, 2]};
        <=;
        >=
    "##;
    let mut lexer = Lexer::new(input);

    loop {
        let token = lexer.next_token();
        println!("{token:?}");

        if token.kind == TokenKind::Eof {
            break;
        }
    }

    Ok(())
}
