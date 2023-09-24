use std::io;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Prompter {
    prompt: String,
}

impl Prompter {
    pub fn new() -> Self {
        Prompter { prompt: String::from("> ") }
    }

    pub fn lines(&mut self) -> LinesIter {
        LinesIter::new(self)
    }
}

pub struct LinesIter<'a> {
    prompter: &'a mut Prompter,
    stdout: RawTerminal<io::Stdout>,
}

impl<'a> LinesIter<'a> {
    fn new(prompter: &'a mut Prompter) -> Self {
        let stdout = io::stdout().into_raw_mode().expect("termion into_raw_mode error");
        LinesIter { prompter, stdout }
    }
}

impl<'a> Iterator for LinesIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", &self.prompter.prompt);
        io::Write::flush(&mut io::stdout()).expect("flush error");

        let mut input = String::new();
        for c in io::stdin().keys() {
            match c.expect("termion keys error") {
                Key::Char('\n') => return Some(input),
                Key::Char(c) => input.push(c),
                _ => (),
            }
        }
        None
    }
}
