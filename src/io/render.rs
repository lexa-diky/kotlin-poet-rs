use crate::spec::CodeBlock;

/// Responsible for rendering Kotlin code.
/// Normally you need to just override [RenderKotlin::render].
pub trait RenderKotlin {
    fn render_into(&self, block: &mut CodeBlock);

    /// Shortcut method for converting [RenderKotlin::render] output into [String]
    fn render_string(&self) -> String {
        let mut block = CodeBlock::empty();
        self.render_into(&mut block);
        block.to_string()
    }
}
