use std::error::Error;

use jerboa::parser::Parser;

/*
To run with auto-reload:
cargo watch -w src -x run -c
*/
fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let a = 2 + b * 2 > 3;
    "#;

    let mut parser = Parser::new(&input);
    let res = parser.parse_program()?;
    println!("{:#?}", res);

    Ok(())
}
