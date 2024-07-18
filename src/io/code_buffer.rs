#[derive(Debug, Default, PartialEq, Clone)]
pub struct CodeBuffer {
    buffer: String
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

    pub fn last_char(&self) -> Option<char> {
        return self.buffer.chars().last()
    }
}