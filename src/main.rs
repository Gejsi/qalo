use std::error::Error;

use jerboa::{evaluator::Evaluator, object::Object};

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let i = 5;
        let foo = fn(i) {
            i;
        };

        foo(10);
        i;
    "#;

    let mut evaluator = Evaluator::new(&input);
    for (i, obj) in evaluator.eval_program()?.into_iter().enumerate() {
        // if !matches!(obj, Object::UnitValue) {
        println!("{i} {obj}");
        // }
    }

    Ok(())
}
