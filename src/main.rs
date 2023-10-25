use std::error::Error;

use jerboa::parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let a = fn(arg) {
            let bar = 2;

            return fn(foo) {
                bar
            };
        };
    "#;

    let mut parser = Parser::new(&input);
    let res = parser.parse_program()?;
    println!("{res}");

    Ok(())
}
