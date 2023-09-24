use std::io;

pub struct Prompter;

impl Prompter {
    pub fn new() -> Self {
        Prompter
    }

    pub fn lines(&mut self) -> &mut Self {
        self
    }
}

impl Iterator for Prompter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read_line failed");
        Some(input.trim().to_string())
    }
}
