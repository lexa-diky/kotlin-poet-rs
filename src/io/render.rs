use crate::io::tokens::INDENT;

pub trait RenderKotlin {

    fn render_into(&self, context: RenderContext, buffer: &mut crate::io::CodeBuffer) {
        let content = RenderKotlin::render(self, context);
        buffer.push(content.as_str())
    }

    fn render(&self, context: RenderContext) -> String;

    fn render_without_context(&self) -> String {
        self.render(RenderContext::new())
    }
}

#[derive(Copy, Clone)]
pub struct RenderContext {
    indent_level: usize,
}

impl RenderContext {

    pub fn new() -> RenderContext {
        RenderContext { indent_level: 0 }
    }

    pub fn indent(&self) -> RenderContext {
        RenderContext { indent_level: self.indent_level + 1 }
    }

    pub fn intent_str(&self) -> String {
        let mut buff = String::new();
        for _ in 0..self.indent_level {
            buff.push_str(INDENT);
        }
        buff
    }
}