use crate::io::RenderKotlin;
use crate::spec::{AccessModifier, Argument, CodeBlock, FunctionParameter, PrimaryConstructor};
use crate::tokens;

/// Defines [Class's secondary constructor](https://kotlinlang.org/docs/classes.html#constructors)
#[derive(Debug, Clone)]
pub struct SecondaryConstructor {
    parameters: Vec<FunctionParameter>,
    delegate_parameters: Vec<Argument>,
    access_modifier: AccessModifier,
    body: Option<CodeBlock>,
}

impl SecondaryConstructor {
    pub fn new() -> SecondaryConstructor {
        SecondaryConstructor {
            parameters: Vec::new(),
            delegate_parameters: Vec::new(),
            access_modifier: AccessModifier::Public,
            body: None,
        }
    }

    pub fn parameter(mut self, parameter: FunctionParameter) -> SecondaryConstructor {
        self.parameters.push(parameter);
        self
    }

    pub fn delegate_argument(mut self, argument: Argument) -> SecondaryConstructor {
        self.delegate_parameters.push(argument);
        self
    }

    pub fn body(mut self, body: CodeBlock) -> SecondaryConstructor {
        self.body = Some(body);
        self
    }

    pub fn access_modifier(mut self, access_modifier: AccessModifier) -> SecondaryConstructor {
        self.access_modifier = access_modifier;
        self
    }
}

impl RenderKotlin for SecondaryConstructor {
    fn render(&self) -> CodeBlock {
        let mut cb = CodeBlock::empty();
        let mut pc = PrimaryConstructor::new()
            .access_modifier(self.access_modifier.clone());

        for parameter in &self.parameters {
            pc = pc.parameter(parameter.clone());
        }

        cb.with_nested(pc.render());
        cb.with_space();
        cb.with_atom(tokens::COLON);
        cb.with_space();
        cb.with_atom(tokens::keyword::THIS);
        cb.with_round_brackets(|params_block| {
            params_block.with_comma_separated(&self.delegate_parameters);
        });
        cb.with_space();
        cb.with_curly_brackets(|body_block| {
            if let Some(body) = &self.body {
                body_block.with_nested(body.clone());
            }
        });

        cb
    }
}

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;
    use crate::spec::{AccessModifier, Argument, CodeBlock, FunctionParameter, SecondaryConstructor, Type};

    #[test]
    fn secondary_constructor_test() {
        let secondary_constructor = SecondaryConstructor::new()
            .access_modifier(AccessModifier::Public)
            .parameter(FunctionParameter::new("name".into(), Type::string()))
            .parameter(FunctionParameter::new("age".into(), Type::int()))
            .delegate_argument(Argument::new(CodeBlock::atom("name")))
            .delegate_argument(Argument::new(CodeBlock::atom("age")))
            .body(CodeBlock::statement("println(42)"));

        let rendered = secondary_constructor.render().to_string();
        let expected = "public constructor(name: kotlin.String, age: kotlin.Int) : this(name, age) {\n    println(42)\n}";
        assert_eq!(rendered, expected);
    }
}