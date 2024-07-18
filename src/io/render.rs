use crate::spec::CodeBlock;

pub trait RenderKotlin {

    fn render(&self, context: RenderContext) -> CodeBlock;

    fn render_string(&self, context: RenderContext) -> String {
        self.render(context).render()
    }

    fn render_string_in_root(&self) -> String {
        self.render_string(RenderContext::new())
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