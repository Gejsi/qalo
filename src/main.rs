use std::error::Error;

use jerboa::evaluator::Evaluator;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"
        5 + true;
    "#;

    let mut evaluator = Evaluator::new(&input);
    let res = evaluator.eval_program()?;
    println!("{res:#?}");

    Ok(())
}
