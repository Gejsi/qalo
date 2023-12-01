use std::{env, error::Error, fs, process};

use qalo::evaluator::Evaluator;

fn main() -> Result<(), Box<dyn Error>> {
    let files = env::args()
        .into_iter()
        .filter(|file| file.ends_with(".ql"))
        .collect::<Vec<String>>();

    for file in files {
        let source = fs::read_to_string(file).expect("Failed to read a file");

        let mut evaluator = Evaluator::new(&source);
        evaluator.eval_program().unwrap_or_else(|err| {
            eprintln!("| Qalo Error |\n{err}");
            process::exit(1);
        });
    }

    Ok(())
}
