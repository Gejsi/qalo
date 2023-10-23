use std::error::Error;

use jerboa::parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        if 2 * 2 > 1 {
            let a = 3;
            a
        } else {
            b
        }
    "#;

    let mut parser = Parser::new(&input);
    let res = parser.parse_program()?;
    println!("{res:#?}");

    Ok(())
}
