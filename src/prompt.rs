use std::io;

pub struct Prompt;

impl Iterator for Prompt {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read_line failed");
        Some(input.trim().to_string())
    }
}
