use std::error::Error;

use jerboa::{evaluator::Evaluator, parser::Parser};

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let a = fn(a, b) {
        a
            };
    "#;

    let mut parser = Parser::new(&input);
    let res = parser.parse_program()?;
    println!("{res}");

    Ok(())
}
