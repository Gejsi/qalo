use jerboa::lexer::Lexer;
use std::error::Error;

// To run with auto-reload:
// cargo watch -w src -s "cargo run" -c
fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        "#;
    let mut lexer = Lexer::new(input);

    input.chars().for_each(|_| {
        let token = lexer.next_token();
        dbg!(token);
    });

    Ok(())
}
