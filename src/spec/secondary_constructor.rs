use crate::io::RenderKotlin;
use crate::spec::{VisibilityModifier, Argument, CodeBlock, Parameter, PrimaryConstructor};
use crate::spec::kdoc::{KdocSlot, mixin_kdoc_mutators};
use crate::tokens;

/// Defines [Class's secondary constructor](https://kotlinlang.org/docs/classes.html#constructors)
#[derive(Debug, Clone)]
pub struct SecondaryConstructor {
    parameters: Vec<Parameter>,
    delegate_parameters: Vec<Argument>,
    visibility_modifier: VisibilityModifier,
    body: Option<CodeBlock>,
    kdoc: KdocSlot
}

impl SecondaryConstructor {
    pub fn new() -> SecondaryConstructor {
        SecondaryConstructor {
            parameters: Vec::new(),
            delegate_parameters: Vec::new(),
            visibility_modifier: VisibilityModifier::default(),
            body: None,
            kdoc: KdocSlot::default()
        }
    }

    pub fn parameter(mut self, parameter: Parameter) -> SecondaryConstructor {
        self.parameters.push(parameter);
        self
    }

    pub fn delegate_argument(mut self, argument: Argument) -> SecondaryConstructor {
        self.delegate_parameters.push(argument);
        self
    }

    pub fn body<CodeBlockLike: Into<CodeBlock>>(mut self, body: CodeBlockLike) -> SecondaryConstructor {
        self.body = Some(body.into());
        self
    }

    pub fn visibility_modifier(mut self, visibility_modifier: VisibilityModifier) -> SecondaryConstructor {
        self.visibility_modifier = visibility_modifier;
        self
    }

    mixin_kdoc_mutators!();
}

impl RenderKotlin for SecondaryConstructor {
    fn render_into(&self, block: &mut CodeBlock) {
        block.push_renderable(&self.kdoc);

        let mut pc = PrimaryConstructor::new()
            .visibility_modifier(self.visibility_modifier.clone());

        for parameter in &self.parameters {
            pc = pc.parameter(parameter.clone());
        }

        block.push_renderable(&pc);
        block.push_space();
        block.push_static_atom(tokens::COLON);
        block.push_space();
        block.push_static_atom(tokens::keyword::THIS);
        block.push_round_brackets(|params_block| {
            params_block.push_comma_separated(&self.delegate_parameters);
        });
        block.push_space();
        block.push_curly_brackets(|body_block| {
            if let Some(body) = &self.body {
                body_block.push_renderable(body);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;
    use crate::spec::{VisibilityModifier, Argument, CodeBlock, Parameter, SecondaryConstructor, Type, KDoc, Name};

    #[test]
    fn test_secondary_constructor() {
        let secondary_constructor = SecondaryConstructor::new()
            .visibility_modifier(VisibilityModifier::Public)
            .parameter(Parameter::new(Name::from("name"), Type::string()))
            .parameter(Parameter::new(Name::from("age"), Type::int()))
            .delegate_argument(Argument::new_positional(CodeBlock::atom("name")))
            .delegate_argument(Argument::new_positional(CodeBlock::atom("age")))
            .body(CodeBlock::statement("println(42)"));

        let rendered = secondary_constructor.render_string();
        let expected = "public constructor(name: kotlin.String, age: kotlin.Int) : this(name, age) {\n    println(42)\n}";
        assert_eq!(rendered, expected);
    }

    #[test]
    fn test_secondary_constructor_with_kdoc() {
        let secondary_constructor = SecondaryConstructor::new()
            .kdoc(KDoc::from("Hello\nWorld"));

        let rendered = secondary_constructor.render_string();
        let expected = "/**\n * Hello\n * World\n */\npublic constructor() : this() {\n}";
        assert_eq!(rendered, expected);
    }
}