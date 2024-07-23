use crate::spec::CodeBlock;

/// Responsible for rendering Kotlin code.
/// Normally you need to just override [RenderKotlin::render].
pub trait RenderKotlin {

    /// Converts domain centric internal language entity into [CodeBlock].
    fn render(&self) -> CodeBlock;

    /// Shortcut method for converting [RenderKotlin::render] output into [String]
    fn render_string(&self) -> String {
        self.render().to_string()
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