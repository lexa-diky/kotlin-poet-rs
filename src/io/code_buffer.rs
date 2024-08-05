#[derive(Debug, Default)]
pub(crate) struct CodeBuffer {
    buffer: String,
}

impl CodeBuffer {

    pub(crate) fn push(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    pub(crate) fn into_string(self) -> String {
        self.buffer
    }

    pub(crate) fn last_char(&self) -> Option<char> {
        return self.buffer.chars().last();
    }

    const SPACE_LIKE: [char; 2] = [' ', '\n'];

    /// Performs in place trimming of the inner buffer
    pub(crate) fn trim(&mut self) {
        while self.buffer.starts_with(Self::SPACE_LIKE) {
            self.buffer.remove(0);
        }
        while self.buffer.ends_with(Self::SPACE_LIKE) {
            self.buffer.remove(self.buffer.len() - 1);
        }
    }
}