use jerboa::lexer::Lexer;
use std::error::Error;

// To run with auto-reload:
// cargo watch -w src -x run -c
fn main() -> Result<(), Box<dyn Error>> {
    let input = r##"
        "fooar"
    "##;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    println!("{tokens:#?}");

    Ok(())
}
