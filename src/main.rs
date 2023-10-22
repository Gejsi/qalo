use std::error::Error;

use jerboa::parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        return 1;

        { a + a return 1; }

        { 2 + 2; }

        let b = a;
        let b = b;
    "#;

    let mut parser = Parser::new(&input);
    let res = parser.parse_program()?;
    println!("{}", res.to_string());

    Ok(())
}
