use crate::io::RenderKotlin;
use crate::spec::{AccessModifier, CodeBlock, FunctionParameter, Property};
use crate::tokens;

#[derive(Debug, Clone)]
enum PrimaryConstructorParameter {
    PropertyParameter(Property),
    FunctionLike(FunctionParameter),
}

impl RenderKotlin for PrimaryConstructorParameter {
    fn render(&self) -> CodeBlock {
        match self {
            PrimaryConstructorParameter::PropertyParameter(property) => property.render(),
            PrimaryConstructorParameter::FunctionLike(param) => param.render()
        }
    }
}

/// Defines [Class's primary constructor](https://kotlinlang.org/docs/classes.html#constructors)
#[derive(Debug, Clone)]
pub struct PrimaryConstructor {
    arguments: Vec<PrimaryConstructorParameter>,
    access_modifier: AccessModifier
}

impl PrimaryConstructor {
    pub fn new() -> PrimaryConstructor {
        PrimaryConstructor {
            arguments: Vec::new(),
            access_modifier: AccessModifier::Public
        }
    }

    pub fn parameter(mut self, parameter: FunctionParameter) -> PrimaryConstructor {
        self.arguments.push(PrimaryConstructorParameter::FunctionLike(parameter));
        self
    }

    pub fn property(mut self, property: Property) -> PrimaryConstructor {
        self.arguments.push(PrimaryConstructorParameter::PropertyParameter(property));
        self
    }

    pub fn access_modifier(mut self, access_modifier: AccessModifier) -> PrimaryConstructor {
        self.access_modifier = access_modifier;
        self
    }
}

impl RenderKotlin for PrimaryConstructor {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();
        block.with_nested(self.access_modifier.render());
        block.with_space();
        block.with_atom(tokens::keyword::CONSTRUCTOR);
        block.with_round_brackets(|params_block| {
            params_block.with_comma_separated(&self.arguments)
        });

        block
    }
}

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;
    use crate::spec::{AccessModifier, CodeBlock, FunctionParameter, PrimaryConstructor, Property, Type};

    #[test]
    fn primary_constructor_test() {
        let property = Property::new(
            "name".into(),
            Type::string(),
        ).initializer(
            CodeBlock::atom("\"\"")
        );

        let function_parameter = FunctionParameter::new(
            "age".into(),
            Type::int()
        );

        let primary_constructor = PrimaryConstructor::new()
            .property(property)
            .parameter(function_parameter);

        assert_eq!(
            primary_constructor.render().to_string(),
            "public constructor(public final val name: kotlin.String = \"\", age: kotlin.Int)"
        );
    }

    #[test]
    fn test_private_constructor() {
        let primary_constructor = PrimaryConstructor::new()
            .access_modifier(AccessModifier::Private);

        assert_eq!(
            primary_constructor.render().to_string(),
            "private constructor()"
        );
    }
}