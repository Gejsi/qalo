use std::error::Error;

use qalo::{evaluator::Evaluator, object::Object};

fn main() -> Result<(), Box<dyn Error>> {
    // let input = r#"
    //     let counter = fn(x) {
    //         if (x > 5) {
    //             return true;
    //         } else {
    //             let foobar = 9999;
    //             counter(x + 1);
    //         }
    //     };

    //     counter(0);
    // "#;
    let input = r#"
        let add = fn(x, y) { x + y; };
        add(5 + 5, add(1, 2));
    "#;

    let mut evaluator = Evaluator::new(&input);
    for obj in evaluator.eval_program()? {
        if !matches!(obj, Object::UnitValue) {
            println!("{obj}");
        }
    }

    Ok(())
}
