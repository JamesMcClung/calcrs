use std::io;

pub struct Prompter {
    prompt: String,
}

impl Prompter {
    pub fn new() -> Self {
        Prompter { prompt: String::from("> ") }
    }

    pub fn lines(&mut self) -> &mut Self {
        self
    }
}

pub struct LinesIter<'a> {
    prompter: &'a mut Prompter,
}

impl<'a> LinesIter<'a> {
    fn new(prompter: &'a mut Prompter) -> Self {
        LinesIter { prompter }
    }
}

impl Iterator for Prompter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", &self.prompt);
        io::Write::flush(&mut io::stdout()).expect("flush failed");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read_line failed");
        Some(input.trim().to_string())
    }
}
