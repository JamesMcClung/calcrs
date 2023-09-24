pub mod parse;
pub mod prompt;

fn main() {
    for input in prompt::Prompt {
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
