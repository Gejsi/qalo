use std::error::Error;

use jerboa::evaluator::Evaluator;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let bar = fn() { return 1; };

        let foo = if bar() + 1 == 2 {
            if true {
                return 2;
            }

            return 1;
        };

        foo;
    "#;

    let mut evaluator = Evaluator::new(&input);
    for obj in evaluator.eval_program()? {
        println!("{obj}");
    }

    Ok(())
}
