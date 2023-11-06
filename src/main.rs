use std::error::Error;

use jerboa::{evaluator::Evaluator, object::Object};

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let bar = fn() { return 2; };
        let baz = if true { 2; };

        let foo = if bar() + 1 == 3 {
            if true {
                {
                    return fn(x) { x; };
                }
            }

            return 1;
        };

        let id = foo(3);
        id;
    "#;

    let mut evaluator = Evaluator::new(&input);
    for obj in evaluator.eval_program()? {
        if !matches!(obj, Object::UnitValue) {
            println!("{obj}");
        }
    }

    Ok(())
}
