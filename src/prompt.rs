pub struct Prompt;

impl Iterator for Prompt {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
