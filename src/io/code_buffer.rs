#[derive(Debug, Default, PartialEq, Clone)]
pub struct CodeBuffer {
    buffer: String,
}

impl CodeBuffer {
    pub fn from(s: &str) -> CodeBuffer {
        CodeBuffer {
            buffer: s.to_string()
        }
    }

    pub fn push(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    pub fn as_string(&self) -> String {
        self.buffer.clone()
    }

    pub fn into_string(self) -> String {
        self.buffer
    }

    pub fn last_char(&self) -> Option<char> {
        return self.buffer.chars().last();
    }

    /// Performs in place trimming of the inner buffer
    pub fn trim(&mut self) {
        while self.buffer.starts_with([' ', '\n']) {
            self.buffer.remove(0);
        }
        while self.buffer.ends_with([' ', '\n']) {
            self.buffer.remove(self.buffer.len() - 1);
        }
    }
}