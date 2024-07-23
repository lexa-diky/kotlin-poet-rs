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
