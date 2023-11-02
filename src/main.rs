use std::error::Error;

use jerboa::evaluator::Evaluator;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let foo = fn() {
            let double = fn(y) { y * 2; };
            return double;
        };

        let doubler = foo();
        let bar = doubler(2);
        bar;
    "#;

    let mut evaluator = Evaluator::new(&input);
    for obj in evaluator.eval_program()? {
        println!("{obj}");
    }

    Ok(())
}
