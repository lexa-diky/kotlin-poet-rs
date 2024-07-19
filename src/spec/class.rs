use crate::io::{RenderKotlin, tokens};
use crate::spec::{AccessModifier, ClassLikeInheritanceModifier, CodeBlock, Name};

#[derive(Debug, Clone)]
pub struct Class {
    name: Name,
    access_modifier: AccessModifier,
    inheritance_modifier: ClassLikeInheritanceModifier
}

impl Class {

    pub fn new(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassLikeInheritanceModifier::Final
        }
    }

    pub fn access_modifier(mut self, access_modifier: AccessModifier) -> Self {
        self.access_modifier = access_modifier;
        self
    }

    pub fn inheritance_modifier(mut self, inheritance_modifier: ClassLikeInheritanceModifier) -> Self {
        self.inheritance_modifier = inheritance_modifier;
        self
    }
}

impl RenderKotlin for Class {
    fn render(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();

        code.with_nested(self.access_modifier.render());
        code.with_space();
        code.with_nested(self.inheritance_modifier.render());
        code.with_space();
        code.with_atom(tokens::KW_CLASS);
        code.with_space();
        code.with_nested(self.name.render());
        code.with_space();
        code.with_atom(tokens::CURLY_BRACE_LEFT);
        code.with_new_line();
        code.with_indent();

        // content

        code.with_unindent();
        code.with_atom(tokens::CURLY_BRACE_RIGHT);

        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class() {
        let class = Class::new(Name::from("Person"));
        let code = class.render();

        assert_eq!(code.to_string(), "public final class Person {\n}");
    }
}