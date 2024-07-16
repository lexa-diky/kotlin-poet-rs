#[derive(Debug, Default)]
pub struct CodeBuffer {
    buffer: String
}

impl CodeBuffer {

    pub fn push(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    pub fn to_string(&self) -> String {
        self.buffer.clone()
    }
}