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

mod special_keys {
    use termion::event::Key;
    pub const WORD_LEFT: Key = Key::Alt('b');
    pub const WORD_RIGHT: Key = Key::Alt('f');
    pub const WORD_BACKSPACE: Key = Key::Ctrl('w');
    pub const LINE_LEFT: Key = Key::Ctrl('a');
    pub const LINE_RIGHT: Key = Key::Ctrl('e');
    pub const LINE_BACKSPACE: Key = Key::Ctrl('u');
}

impl<'a, T: Write, K: Iterator<Item = Key>> Iterator for LinesIter<'a, T, K> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut key_handler = KeyHandler::new(&self.prompter.history);
        set_input_state(&mut self.terminal, &self.prompter.prompt, &key_handler.input, key_handler.cursor_pos);

        loop {
            match self.keys.next()? {
                Key::Char('\n') => {
                    write(&mut self.terminal, "\n\r");
                    let line_pos = key_handler.line_pos;
                    if line_pos == self.prompter.history.len() {
                        self.prompter.history.push(key_handler.input);
                    }
                    return Some(self.prompter.history[line_pos].clone());
                },
                Key::Char(c) if c.is_ascii() => key_handler.handle_char(c),
                Key::Backspace => key_handler.handle_backspace(),
                Key::Left => key_handler.handle_left(),
                Key::Right => key_handler.handle_right(),
                Key::Up => key_handler.handle_up(),
                Key::Down => key_handler.handle_down(),
                special_keys::WORD_LEFT => key_handler.handle_word_left(),           // Mac: Option-Left
                special_keys::WORD_RIGHT => key_handler.handle_word_right(),         // Mac: Option-Right
                special_keys::LINE_LEFT => key_handler.handle_line_left(),           // Mac: Command-Left
                special_keys::LINE_RIGHT => key_handler.handle_line_right(),         // Mac: Command-Right
                special_keys::WORD_BACKSPACE => key_handler.handle_word_backspace(), // Mac: Option-Backspace
                special_keys::LINE_BACKSPACE => key_handler.handle_line_backspace(), // Mac: Command-Backspace
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

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(expected_output: Vec<&str>, input: Vec<Key>) {
        let mut prompter = Prompter::new();
        let linesiter = LinesIter { prompter: &mut prompter, terminal: Vec::new(), keys: input.into_iter() };
        assert_eq!(expected_output, linesiter.collect::<Vec<_>>());
    }

    #[test]
    fn words() {
        use Key::Char;
        do_test(vec![""], vec![Char('\n')]);
        do_test(vec!["hi"], vec![Char('h'), Char('i'), Char('\n')]);
        do_test(vec!["hi you"], vec![Char('h'), Char('i'), Char(' '), Char('y'), Char('o'), Char('u'), Char('\n')]);
        do_test(vec!["hi", "you"], vec![Char('h'), Char('i'), Char('\n'), Char('y'), Char('o'), Char('u'), Char('\n')]);
    }

    #[test]
    fn left_right() {
        use Key::{Char, Left, Right};
        do_test(vec![""], vec![Right, Char('\n')]);
        do_test(vec![""], vec![Left, Char('\n')]);
        do_test(vec!["ih"], vec![Char('h'), Left, Char('i'), Char('\n')]);
        do_test(vec!["hi"], vec![Char('h'), Left, Right, Char('i'), Char('\n')]);
        do_test(vec!["ih"], vec![Char('h'), Right, Left, Char('i'), Char('\n')]);
    }

    #[test]
    fn word_left_right() {
        use special_keys::{WORD_LEFT, WORD_RIGHT};
        use Key::{Char, Left};
        do_test(vec![""], vec![WORD_RIGHT, Char('\n')]);
        do_test(vec![""], vec![WORD_LEFT, Char('\n')]);
        do_test(vec!["lhe"], vec![Char('h'), Char('e'), WORD_LEFT, Char('l'), Char('\n')]);
        do_test(vec!["hel"], vec![Char('h'), Char('e'), WORD_LEFT, WORD_RIGHT, Char('l'), Char('\n')]);
        do_test(vec!["ih"], vec![Char('h'), WORD_RIGHT, WORD_LEFT, Char('i'), Char('\n')]);
        do_test(vec!["hi uyo"], vec![Char('h'), Char('i'), Char(' '), Char('y'), Char('o'), WORD_LEFT, Char('u'), Char('\n')]);
        do_test(vec!["hlelo"], vec![Char('h'), Char('e'), Char('l'), Left, Left, Char('l'), WORD_RIGHT, Char('o'), Char('\n')]);
        do_test(vec!["a db c"], vec![Char('a'), Char(' '), Char('b'), Char(' '), Char('c'), WORD_LEFT, WORD_LEFT, Char('d'), Char('\n')]);
        do_test(vec!["a bd c"], vec![Char('a'), Char(' '), Char('b'), Char(' '), Char('c'), Left, Left, Left, Left, Left, WORD_RIGHT, WORD_RIGHT, Char('d'), Char('\n')]);
    }

    #[test]
    fn line_left_right() {
        use special_keys::{LINE_LEFT, LINE_RIGHT};
        use Key::Char;
        do_test(vec![""], vec![LINE_LEFT, Char('\n')]);
        do_test(vec![""], vec![LINE_RIGHT, Char('\n')]);
        do_test(vec!["ohi y"], vec![Char('h'), Char('i'), Char(' '), Char('y'), LINE_LEFT, Char('o'), Char('\n')]);
        do_test(vec!["hi yo"], vec![Char('h'), Char('i'), Char(' '), Char('y'), LINE_LEFT, LINE_RIGHT, Char('o'), Char('\n')]);
    }

    #[test]
    fn backspace() {
        use Key::{Backspace, Char, Left};
        do_test(vec![""], vec![Backspace, Char('\n')]);
        do_test(vec!["i"], vec![Char('h'), Backspace, Char('i'), Char('\n')]);
        do_test(vec![""], vec![Char('h'), Char('i'), Backspace, Backspace, Char('\n')]);
        do_test(vec!["i"], vec![Char('h'), Char('i'), Left, Backspace, Char('\n')]);
    }

    #[test]
    fn word_backspace() {
        use special_keys::WORD_BACKSPACE;
        use Key::{Char, Left};
        do_test(vec![""], vec![WORD_BACKSPACE, Char('\n')]);
        do_test(vec![""], vec![Char('h'), Char('i'), WORD_BACKSPACE, Char('\n')]);
        do_test(vec![""], vec![Char('h'), Char('i'), Char(' '), WORD_BACKSPACE, Char('\n')]);
        do_test(vec!["hi "], vec![Char('h'), Char('i'), Char(' '), Char('y'), WORD_BACKSPACE, Char('\n')]);
        do_test(vec!["hi "], vec![Char('h'), Char('i'), Char(' '), Char('y'), Char('o'), WORD_BACKSPACE, Char('\n')]);
        do_test(vec!["hi o"], vec![Char('h'), Char('i'), Char(' '), Char('y'), Char('o'), Left, WORD_BACKSPACE, Char('\n')]);
    }

    #[test]
    fn line_backspace() {
        use special_keys::LINE_BACKSPACE;
        use Key::{Char, Left};
        do_test(vec![""], vec![LINE_BACKSPACE, Char('\n')]);
        do_test(vec![""], vec![Char('h'), Char('i'), LINE_BACKSPACE, Char('\n')]);
        do_test(vec![""], vec![Char('h'), Char('i'), Char(' '), LINE_BACKSPACE, Char('\n')]);
        do_test(vec![""], vec![Char('h'), Char('i'), Char(' '), Char('y'), LINE_BACKSPACE, Char('\n')]);
        do_test(vec![""], vec![Char('h'), Char('i'), Char(' '), Char('y'), Char('o'), LINE_BACKSPACE, Char('\n')]);
        do_test(vec!["o"], vec![Char('h'), Char('i'), Char(' '), Char('y'), Char('o'), Left, LINE_BACKSPACE, Char('\n')]);
    }

    #[test]
    fn up_down() {
        use Key::{Backspace, Char, Down, Left, Right, Up};
        do_test(vec![""], vec![Up, Char('\n')]);
        do_test(vec![""], vec![Down, Char('\n')]);
        do_test(vec!["a", "a"], vec![Char('a'), Char('\n'), Up, Char('\n')]);
        do_test(vec!["a", "ab"], vec![Char('a'), Char('\n'), Up, Char('b'), Char('\n')]);
        do_test(vec!["a", "a"], vec![Char('a'), Char('\n'), Char('b'), Up, Char('\n')]);
        do_test(vec!["a", "b"], vec![Char('a'), Char('\n'), Up, Down, Char('b'), Char('\n')]);
        do_test(vec!["a", "b"], vec![Char('a'), Char('\n'), Up, Left, Down, Char('b'), Char('\n')]);
        do_test(vec!["ab", "a"], vec![Char('a'), Char('b'), Char('\n'), Up, Backspace, Char('\n')]);
        do_test(vec!["ab", "a"], vec![Char('a'), Char('b'), Char('\n'), Up, Right, Backspace, Char('\n')]);
        do_test(vec!["ab", "b"], vec![Char('a'), Char('b'), Char('\n'), Up, Left, Backspace, Char('\n')]);
        do_test(vec!["a", "ac"], vec![Char('a'), Char('\n'), Char('b'), Up, Char('c'), Char('\n')]);
        do_test(vec!["a", "b", "a"], vec![Char('a'), Char('\n'), Char('b'), Char('\n'), Up, Up, Char('\n')]);
        do_test(vec!["a", "b", "b"], vec![Char('a'), Char('\n'), Char('b'), Char('\n'), Up, Up, Down, Char('\n')]);
        do_test(vec!["abc", "de"], vec![Char('a'), Char('b'), Char('c'), Char('\n'), Char('d'), Up, Left, Down, Char('e'), Char('\n')]);
    }
}
