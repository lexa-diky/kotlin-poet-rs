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

    pub fn to_string(&self) -> String {
        self.buffer.clone()
    }
}