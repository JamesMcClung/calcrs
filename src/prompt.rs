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
    history: Vec<String>,
}

impl<'a> LinesIter<'a> {
    fn new(prompter: &'a mut Prompter) -> Self {
        let stdout = io::stdout().into_raw_mode().expect("termion into_raw_mode error");
        LinesIter { prompter, stdout, history: Vec::new() }
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
        let mut line_pos = self.history.len();
        let mut cursor_pos = 0usize;
        self.set_input_state(&input, cursor_pos);

        for c in io::stdin().keys() {
            match c.expect("termion keys error") {
                Key::Char('\n') => {
                    self.write("\n\r");
                    if line_pos == self.history.len() {
                        self.history.push(input);
                    }
                    return Some(self.history[line_pos].clone());
                }
                Key::Char(c) if c.is_ascii() => {
                    if line_pos < self.history.len() {
                        input = self.history[line_pos].clone();
                        line_pos = self.history.len();
                    }
                    input.insert(cursor_pos, c);
                    cursor_pos += 1;
                }
                Key::Backspace => {
                    if line_pos < self.history.len() {
                        input = self.history[line_pos].clone();
                        line_pos = self.history.len();
                    }
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
                    if cursor_pos < self.history.get(line_pos).unwrap_or(&input).len() {
                        cursor_pos += 1;
                    }
                }
                Key::Up => {
                    if line_pos > 0 {
                        let prev_line_len = self.history.get(line_pos).unwrap_or(&input).len();
                        line_pos -= 1;
                        let curr_line_len = self.history[line_pos].len();
                        if cursor_pos == prev_line_len || cursor_pos > curr_line_len {
                            cursor_pos = curr_line_len;
                        }
                    }
                }
                Key::Down => {
                    if line_pos < self.history.len() {
                        let prev_line_len = self.history[line_pos].len();
                        line_pos += 1;
                        let curr_line_len = self.history.get(line_pos).unwrap_or(&input).len();
                        if cursor_pos == prev_line_len || cursor_pos > curr_line_len {
                            cursor_pos = curr_line_len;
                        }
                    }
                }
                Key::Alt('b') => {
                    // Mac: produced by Option-Left
                    let curr_line = self.history.get(line_pos).unwrap_or(&input);
                    let mut set_pos = false;
                    let mut in_word = false;
                    for (i, c) in curr_line.char_indices().rev().skip(curr_line.len() - cursor_pos) {
                        if c.is_ascii_alphanumeric() {
                            in_word = true;
                        } else if in_word {
                            cursor_pos = i + 1;
                            set_pos = true;
                            break;
                        }
                    }
                    if !set_pos {
                        cursor_pos = 0;
                    }
                }
                Key::Alt('f') => {
                    // Mac: produced by Option-Right
                    let curr_line = self.history.get(line_pos).unwrap_or(&input);
                    let mut set_pos = false;
                    let mut in_word = false;
                    for (i, c) in curr_line.char_indices().skip(cursor_pos) {
                        if c.is_ascii_alphanumeric() {
                            in_word = true;
                        } else if in_word {
                            cursor_pos = i;
                            set_pos = true;
                            break;
                        }
                    }
                    if !set_pos {
                        cursor_pos = curr_line.len();
                    }
                }
                Key::Ctrl('a') => {
                    // Mac: produced by Command-Left
                    cursor_pos = 0;
                }
                Key::Ctrl('e') => {
                    // Mac: produced by Command-Right
                    cursor_pos = self.history.get(line_pos).unwrap_or(&input).len();
                }
                Key::Ctrl('w') => {
                    // Mac: produced by Option-Backspace
                    let curr_line = self.history.get(line_pos).unwrap_or(&input);
                    let mut deleted = false;
                    let mut in_word = false;
                    for (i, c) in curr_line.char_indices().rev().skip(curr_line.len() - cursor_pos) {
                        if c.is_ascii_alphanumeric() {
                            in_word = true;
                        } else if in_word {
                            if line_pos < self.history.len() {
                                input = self.history[line_pos].clone();
                                line_pos = self.history.len();
                            }
                            input.replace_range(i + 1..cursor_pos, "");
                            cursor_pos = i + 1;
                            deleted = true;
                            break;
                        }
                    }
                    if !deleted {
                        if line_pos < self.history.len() {
                            input = self.history[line_pos].clone();
                            line_pos = self.history.len();
                        }
                        input.replace_range(..cursor_pos, "");
                        cursor_pos = 0;
                    }
                }
                Key::Ctrl('u') => {
                    // Mac: produced by Command-Backspace
                    if line_pos < self.history.len() {
                        input = self.history[line_pos].clone();
                        line_pos = self.history.len();
                    }
                    input.replace_range(..cursor_pos, "");
                    cursor_pos = 0;
                }
                _ => (),
            }
            self.set_input_state(&self.history.get(line_pos).unwrap_or(&input).clone(), cursor_pos);
        }
        None
    }
}
