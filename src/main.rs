use jerboa::{lexer::Lexer, parser::Parser};
use std::error::Error;

// To run with auto-reload:
// cargo watch -w src -x run -c
fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
    "#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let var = parser.parse_var_statement().unwrap();

    println!("{:#?}", var);

    Ok(())
}
