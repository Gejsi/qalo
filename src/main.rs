use std::error::Error;

use jerboa::evaluator::Evaluator;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let a = if 2 > 5 {
            let b = 2;
            b + b;
        } else {
            3
        };

        a;
    "#;

    let mut evaluator = Evaluator::new(&input);
    for obj in evaluator.eval_program()? {
        println!("{obj}");
    }

    Ok(())
}
