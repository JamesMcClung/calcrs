use std::io::{self, Write};

use termion::clear;
use termion::cursor;
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

    fn flush(&mut self) {
        self.stdout.flush().expect("flush error");
    }

    fn write(&mut self, text: &str) {
        write!(self.stdout, "{text}").expect("write error");
        self.flush();
    }

    fn set_input_state(&mut self, text: &str, cursor_pos: usize) {
        let clear = clear::CurrentLine;
        let prompt = &self.prompter.prompt;
        let move_right = cursor::Right((prompt.len() + cursor_pos) as u16);
        write!(self.stdout, "\r{clear}{prompt}{text}\r{move_right}").expect("write error");
        self.flush();
    }
}

impl<'a> Iterator for LinesIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut input = String::new();
        let mut cursor_pos = 0usize;
        self.set_input_state(&input, cursor_pos);

        for c in io::stdin().keys() {
            match c.expect("termion keys error") {
                Key::Char('\n') => {
                    self.write("\n\r");
                    return Some(input);
                }
                Key::Char(c) => {
                    input.insert(cursor_pos, c);
                    cursor_pos += 1;
                }
                Key::Backspace => {
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                        input.remove(cursor_pos);
                    }
                }
                Key::Left => {
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                    }
                }
                Key::Right => {
                    if cursor_pos < input.len() {
                        cursor_pos += 1;
                    }
                }
                _ => (),
            }
            self.set_input_state(&input, cursor_pos);
        }
        None
    }
}
