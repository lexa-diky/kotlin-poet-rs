use crate::spec::{CodeBlock, CodeBlockNode};

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

    pub fn level(&self) -> usize {
        self.indent_level
    }
}