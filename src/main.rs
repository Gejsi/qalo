use std::error::Error;

use jerboa::{evaluator::Evaluator, parser::Parser};

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        identity((b * c));
    "#;

    let mut parser = Parser::new(&input);
    let res = parser.parse_program()?;
    println!("{res}");

    Ok(())
}
