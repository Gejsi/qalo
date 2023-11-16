use std::error::Error;

use qalo::{evaluator::Evaluator, object::Object, parser::Parser};

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        len("hello");
    "#;

    // let mut parser = Parser::new(input);
    // let program = parser.parse_program().unwrap();
    // println!("{program:?}");

    let mut evaluator = Evaluator::new(input);
    for obj in evaluator.eval_program()? {
        // if !matches!(obj, Object::UnitValue) {
        // }
        println!("{obj}");
    }

    Ok(())
}
