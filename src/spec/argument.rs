use crate::io::{RenderKotlin, tokens};
use crate::spec::{CodeBlock, Name};

#[derive(Debug, Clone)]
pub struct Argument {
    pub name: Option<Name>,
    pub value: CodeBlock
}

impl Argument {

    pub fn new(value: CodeBlock) -> Self {
        Argument {
            name: None,
            value
        }
    }

    pub fn new_named(name: Name, value: CodeBlock) -> Self {
        Argument {
            name: Some(name),
            value
        }
    }
}

impl RenderKotlin for Argument {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();
        if let Some(name) = &self.name {
            block.with_nested(name.render());
            block.with_space();
            block.with_atom(tokens::EQUALS);
            block.with_space();
        }
        block.with_atom(self.value.to_string().as_str());
        block
    }
}