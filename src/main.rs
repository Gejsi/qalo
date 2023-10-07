use std::error::Error;

use jerboa::parser::Parser;

// To run with auto-reload:
// cargo watch -w src -x run -c
fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let five = 5;
    "#;

    let mut parser = Parser::new(&input);
    let var = parser.parse_var_statement()?;
    println!("{:#?}", var);

    Ok(())
}
