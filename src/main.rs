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
        let add = fn(x, y) { return x + y; };

        let foo = fn() {
            return add(5 + 5, add(1, 1));
        };

        let faz = fn() {
            return 20;
        };

        let bar = if foo() == 12 {
            if foo() == 12 {
                return faz();
            }

            return 100;
        } else {
            return -1;
        };

        bar;
    "#;

    let mut evaluator = Evaluator::new(input);
    for obj in evaluator.eval_program()? {
        // if !matches!(obj, Object::UnitValue) {
        println!("{obj}");
        // }
    }

    Ok(())
}
