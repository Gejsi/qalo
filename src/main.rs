use std::error::Error;

use jerboa::parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        fn foo(arg) {
            let a = 2;
            a
        }
    "#;

    let mut parser = Parser::new(&input);
    let res = parser.parse_program()?;
    println!("{res}");

    Ok(())
}
