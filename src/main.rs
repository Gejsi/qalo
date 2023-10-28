use std::error::Error;

use jerboa::evaluator::Evaluator;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        let a = 2;
        a * 3;
    "#;

    let mut evaluator = Evaluator::new(&input);
    for obj in evaluator.eval_program()? {
        println!("{obj}");
    }

    Ok(())
}
