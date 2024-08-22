use crate::io::RenderKotlin;
use crate::spec::{CodeBlock, Name};
use crate::tokens;

/// Function argument, consists of pair name and value.
/// If name is [None] is considered as positional argument.
///
/// Set of function arguments could is usually represented as [Vec<Argument>].
///
/// # Examples
///
/// ## Named argument
/// ```rust
/// use kotlin_poet_rs::spec::{CodeBlock, Name, Argument};
/// use kotlin_poet_rs::io::RenderKotlin;
///
/// let argument = Argument::new_named(
///      Name::from("name"), CodeBlock::atom("value")
/// );
///
/// assert_eq!(argument.render_string(), "name = value");
/// ```
///
/// ## Positional argument
/// ```rust
/// use kotlin_poet_rs::spec::{CodeBlock, Name, Argument};
/// use kotlin_poet_rs::io::RenderKotlin;
///
/// let argument = Argument::new_positional(
///     CodeBlock::statement("value")
/// );
///
/// assert_eq!(argument.render_string(), "value");
/// ```
#[derive(Debug, Clone)]
pub struct Argument {
    name: Option<Name>,
    value: CodeBlock,
}

impl Argument {
    /// Creates new positional argument
    pub fn new_positional<CodeBlockLike: Into<CodeBlock>>(value: CodeBlockLike) -> Self {
        Argument {
            name: None,
            value: value.into(),
        }
    }

    /// Creates new named argument
    pub fn new_named<NameLike: Into<Name>, CodeBlockLike: Into<CodeBlock>>(name: NameLike, value: CodeBlockLike) -> Self {
        Argument {
            name: Some(name.into()),
            value: value.into(),
        }
    }
}

impl RenderKotlin for Argument {
    fn render_into(&self, block: &mut CodeBlock) {
        if let Some(name) = &self.name {
            block.push_renderable(name);
            block.push_space();
            block.push_static_atom(tokens::ASSIGN);
            block.push_space();
        }
        block.push_atom(self.value.to_string().as_str());
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::{CodeBlock, Name, Argument};
    use crate::io::RenderKotlin;

    #[test]
    fn test_rendering() {
        let argument = Argument::new_positional(CodeBlock::statement("value"));
        assert_eq!(argument.render_string(), "value");

        let argument = Argument::new_named(Name::from("name"), CodeBlock::atom("value"), );
        assert_eq!(argument.render_string(), "name = value");
    }
}
