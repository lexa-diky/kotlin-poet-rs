use crate::spec::CodeBlock;

pub trait RenderKotlin {

    fn render(&self) -> CodeBlock;

    fn render_string(&self) -> String {
        self.render().to_string()
    }

    #[cfg(test)]
    fn render_string_in_root(&self) -> String {
        self.render_string()
    }
}

#[derive(Copy, Clone)]
pub struct RenderContext {
    indent_level: usize,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self::new()
    }
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