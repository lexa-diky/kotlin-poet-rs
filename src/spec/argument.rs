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
/// assert_eq!(argument.render().to_string(), "name = value");
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
/// assert_eq!(argument.render().to_string(), "value");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: Option<Name>,
    pub value: CodeBlock,
}

impl Argument {
    /// Creates new positional argument
    pub fn new_positional(value: CodeBlock) -> Self {
        Argument {
            name: None,
            value,
        }
    }

    /// Creates new named argument
    pub fn new_named(name: Name, value: CodeBlock) -> Self {
        Argument {
            name: Some(name),
            value,
        }
    }
}

impl RenderKotlin for Argument {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();
        if let Some(name) = &self.name {
            block.with_nested(name.render());
            block.with_space();
            block.with_atom(tokens::ASSIGN);
            block.with_space();
        }
        block.with_atom(self.value.to_string().as_str());
        block
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::{CodeBlock, Name, Argument};
    use crate::io::RenderKotlin;

    #[test]
    fn test_rendering() {
        let argument = Argument::new_positional(CodeBlock::statement("value"));
        assert_eq!(argument.render().to_string(), "value");

        let argument = Argument::new_named(Name::from("name"), CodeBlock::atom("value"), );
        println!("{}", argument.render().to_string());
        assert_eq!(argument.render().to_string(), "name = value");
    }
}
