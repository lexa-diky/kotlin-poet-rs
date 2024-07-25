use crate::io::RenderKotlin;
use crate::spec::{CodeBlock, Name, Type};
use crate::tokens;

#[derive(Debug, Clone)]
pub struct Parameter {
    name: Name,
    parameter_type: Type,
    default_value: Option<CodeBlock>,
}

impl RenderKotlin for Parameter {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();
        block.with_nested(self.name.render());
        block.with_atom(tokens::COLON);
        block.with_space();
        block.with_nested(self.parameter_type.render());
        if let Some(default_value) = &self.default_value {
            block.with_space();
            block.with_atom(tokens::ASSIGN);
            block.with_space();
            block.with_nested(default_value.clone());
        }

        block
    }
}

impl Parameter {
    pub fn new(name: Name, parameter_type: Type) -> Parameter {
        Parameter {
            name,
            parameter_type,
            default_value: None,
        }
    }

    pub fn default_value(mut self, default_value: CodeBlock) -> Parameter {
        self.default_value = Some(default_value);
        self
    }
}