pub mod parse;
pub mod prompt;

fn main() {
    for input in prompt::Prompt {
        if input.is_empty() {
            return;
        }
        match parse::parse(&input) {
            Ok(expr) => println!("{}", expr.eval().unwrap()),
            Err(expr) => {
                println!("Couldn't parse: {expr}")
            }
        }
    }
}
