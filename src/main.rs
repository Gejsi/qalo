use std::error::Error;

use jerboa::evaluator::Evaluator;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let foo = fn(x) {
            let double = fn(y) { y * 2; };
            double(x);
        };

        let bar = foo(3);
        bar;
    "#;

    let mut evaluator = Evaluator::new(&input);
    for obj in evaluator.eval_program()? {
        println!("{obj}");
    }

    Ok(())
}
