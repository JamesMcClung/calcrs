use std::io::{self, Write};

use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Prompter {
    prompt: String,
    history: Vec<String>,
}

impl Prompter {
    pub fn new() -> Self {
        Prompter { prompt: String::from("> "), history: Vec::new() }
    }

    pub fn lines(&mut self) -> LinesIter<impl Write, impl Iterator<Item = Key>> {
        let terminal = io::stdout().into_raw_mode().expect("termion into_raw_mode error");
        let keys = io::stdin().keys().map(|key| key.expect("termion keys error"));
        LinesIter { prompter: self, terminal, keys }
    }
}

pub struct LinesIter<'a, T: Write, K: Iterator<Item = Key>> {
    prompter: &'a mut Prompter,
    terminal: T,
    keys: K,
}

struct KeyHandler<'a> {
    cursor_pos: usize,
    line_pos: usize,
    input: String,
    history: &'a Vec<String>,
}

fn write<T: Write>(terminal: &mut T, text: &str) {
    write!(terminal, "{text}").expect("write error");
    terminal.flush().expect("flush error");
}

fn set_input_state<T: Write>(terminal: &mut T, prompt: &str, text: &str, cursor_pos: usize) {
    let clear = clear::CurrentLine;
    let move_right = cursor::Right((prompt.len() + cursor_pos) as u16);
    write!(terminal, "\r{clear}{prompt}{text}\r{move_right}").expect("write error");
    terminal.flush().expect("flush error");
}

impl<'a, T: Write, K: Iterator<Item = Key>> Iterator for LinesIter<'a, T, K> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut key_handler = KeyHandler::new(&self.prompter.history);
        set_input_state(&mut self.terminal, &self.prompter.prompt, &key_handler.input, key_handler.cursor_pos);

        loop {
            let c = self.keys.next();
            if let None = c {
                return None;
            }
            match c.expect("just handled none case") {
                Key::Char('\n') => {
                    write(&mut self.terminal, "\n\r");
                    let line_pos = key_handler.line_pos;
                    if line_pos == self.prompter.history.len() {
                        self.prompter.history.push(key_handler.input);
                    }
                    return Some(self.prompter.history[line_pos].clone());
                }
                Key::Char(c) if c.is_ascii() => key_handler.handle_char(c),
                Key::Backspace => key_handler.handle_backspace(),
                Key::Left => key_handler.handle_left(),
                Key::Right => key_handler.handle_right(),
                Key::Up => key_handler.handle_up(),
                Key::Down => key_handler.handle_down(),
                Key::Alt('b') => key_handler.handle_word_left(),       // Mac: Option-Left
                Key::Alt('f') => key_handler.handle_word_right(),      // Mac: Option-Right
                Key::Ctrl('a') => key_handler.handle_line_left(),      // Mac: Command-Left
                Key::Ctrl('e') => key_handler.handle_line_right(),     // Mac: Command-Right
                Key::Ctrl('w') => key_handler.handle_word_backspace(), // Mac: Option-Backspace
                Key::Ctrl('u') => key_handler.handle_line_backspace(), // Mac: Command-Backspace
                _ => (),
            }
            set_input_state(&mut self.terminal, &self.prompter.prompt, key_handler.get_displayed_line(), key_handler.cursor_pos);
        }
    }
}

impl<'a> KeyHandler<'a> {
    fn new(history: &'a Vec<String>) -> Self {
        KeyHandler { cursor_pos: 0, line_pos: history.len(), input: String::new(), history }
    }

    fn get_displayed_line(&self) -> &str {
        self.history.get(self.line_pos).unwrap_or(&self.input)
    }

    fn prepare_for_edit(&mut self) {
        if self.line_pos < self.history.len() {
            self.input = self.history[self.line_pos].clone();
            self.line_pos = self.history.len();
        }
    }

    fn handle_char(&mut self, c: char) {
        self.prepare_for_edit();
        self.input.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }
    fn handle_backspace(&mut self) {
        self.prepare_for_edit();
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.input.remove(self.cursor_pos);
        }
    }
    fn handle_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }
    fn handle_right(&mut self) {
        if self.cursor_pos < self.get_displayed_line().len() {
            self.cursor_pos += 1;
        }
    }
    fn handle_up(&mut self) {
        if self.line_pos > 0 {
            let prev_line_len = self.get_displayed_line().len();
            self.line_pos -= 1;
            let curr_line_len = self.get_displayed_line().len();
            if self.cursor_pos == prev_line_len || self.cursor_pos > curr_line_len {
                self.cursor_pos = curr_line_len;
            }
        }
    }
    fn handle_down(&mut self) {
        if self.line_pos < self.history.len() {
            let prev_line_len = self.get_displayed_line().len();
            self.line_pos += 1;
            let curr_line_len = self.get_displayed_line().len();
            if self.cursor_pos == prev_line_len || self.cursor_pos > curr_line_len {
                self.cursor_pos = curr_line_len;
            }
        }
    }
    fn handle_word_left(&mut self) {
        let curr_line = self.get_displayed_line();
        let mut in_word = false;
        for (i, c) in curr_line.char_indices().rev().skip(curr_line.len() - self.cursor_pos) {
            if c.is_ascii_alphanumeric() {
                in_word = true;
            } else if in_word {
                self.cursor_pos = i + 1;
                return;
            }
        }
        self.cursor_pos = 0;
    }
    fn handle_word_right(&mut self) {
        let curr_line = self.get_displayed_line();
        let mut in_word = false;
        for (i, c) in curr_line.char_indices().skip(self.cursor_pos) {
            if c.is_ascii_alphanumeric() {
                in_word = true;
            } else if in_word {
                self.cursor_pos = i;
                return;
            }
        }
        self.cursor_pos = curr_line.len();
    }
    fn handle_line_left(&mut self) {
        self.cursor_pos = 0;
    }
    fn handle_line_right(&mut self) {
        self.cursor_pos = self.get_displayed_line().len();
    }
    fn handle_word_backspace(&mut self) {
        self.prepare_for_edit();
        let mut in_word = false;
        for (i, c) in self.input.char_indices().rev().skip(self.input.len() - self.cursor_pos) {
            if c.is_ascii_alphanumeric() {
                in_word = true;
            } else if in_word {
                self.input.replace_range(i + 1..self.cursor_pos, "");
                self.cursor_pos = i + 1;
                return;
            }
        }
        self.input.replace_range(..self.cursor_pos, "");
        self.cursor_pos = 0;
    }
    fn handle_line_backspace(&mut self) {
        self.prepare_for_edit();
        self.input.replace_range(..self.cursor_pos, "");
        self.cursor_pos = 0;
    }
}
