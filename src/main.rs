pub mod parse;
pub mod prompt;

use prompt::Prompter;

fn main() {
    let mut prompter = Prompter::new();

    for input in prompter.lines() {
        if input.is_empty() {
            return;
        }
        match parse::parse(&input) {
            Ok(expr) => match expr.eval() {
                Ok(val) => println!("{val}"),
                Err(err) => println!("{err}"),
            },
            Err(err) => println!("{err}"),
        }
    }
}
