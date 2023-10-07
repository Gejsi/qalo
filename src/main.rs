use std::error::Error;

use jerboa::parser::Parser;

// To run with auto-reload:
// cargo watch -w src -x run -c
fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let five = 5;
        return token;
    "#;

    let mut parser = Parser::new(&input);
    let res = parser.parse_var_statement()?;
    println!("{:#?}", res);
    parser.eat_token();
    let res = parser.parse_return_statement()?;
    println!("{:#?}", res);

    Ok(())
}
