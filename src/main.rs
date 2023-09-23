pub mod parse;
pub mod prompt;

fn main() {
    for input in prompt::Prompt {
        match parse::parse(&input) {
            Ok(expr) => println!("{}", expr.eval().unwrap()),
            Err(expr) if expr.is_empty() => return,
            Err(expr) => {
                println!("Couldn't parse: {expr}")
            }
        }
    }
}
