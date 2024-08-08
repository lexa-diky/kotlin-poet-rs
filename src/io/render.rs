use crate::spec::CodeBlock;

/// Responsible for rendering Kotlin code.
/// Normally you need to just override [RenderKotlin::render].
pub trait RenderKotlin {
    /// Renders Kotlin code into [CodeBlock].
    ///
    /// Usually it is good idea to avoid creating new [CodeBlock]s inside of this method implementation.
    /// Implementers should instead try to push their content into [block] parameter.
    ///
    /// When you need to render another [RenderKotlin] object, use [CodeBlock::push_renderable] method.
    /// It will call [RenderKotlin::render_into] on the object and push its content into [block].
    ///
    /// # Implementation example
    /// ```rust
    /// use kotlin_poet_rs::io::RenderKotlin;
    /// use kotlin_poet_rs::spec::{CodeBlock, Name};
    /// use kotlin_poet_rs::tokens;
    ///
    /// pub struct Argument {
    ///     pub name: Option<Name>,
    ///     pub value: CodeBlock,
    /// }
    ///
    /// impl RenderKotlin for Argument {
    ///     fn render_into(&self, block: &mut CodeBlock) {
    ///         if let Some(name) = &self.name {
    ///             block.push_renderable(name);
    ///             block.push_space();
    ///             block.push_atom(tokens::ASSIGN);
    ///             block.push_space();
    ///         }
    ///         block.push_atom(self.value.to_string().as_str());
    ///     }
    /// }
    /// ```
    fn render_into(&self, block: &mut CodeBlock);

    /// Shortcut method for converting [RenderKotlin::render_into] output into [String].
    fn render_string(&self) -> String {
        let mut block = CodeBlock::empty();
        self.render_into(&mut block);
        block.to_string()
    }
}
