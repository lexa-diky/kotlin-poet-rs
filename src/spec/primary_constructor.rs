use crate::io::RenderKotlin;
use crate::spec::{VisibilityModifier, CodeBlock, Parameter, Property};
use crate::tokens;

#[derive(Debug, Clone)]
enum PrimaryConstructorParameter {
    PropertyParameter(Property),
    FunctionLike(Parameter),
}

impl RenderKotlin for PrimaryConstructorParameter {
    fn render_into(&self, block: &mut CodeBlock) {
        match self {
            PrimaryConstructorParameter::PropertyParameter(property) =>
                block.push_renderable(property),
            PrimaryConstructorParameter::FunctionLike(param) =>
                block.push_renderable(param)
        }
    }
}

/// Defines [Class's primary constructor](https://kotlinlang.org/docs/classes.html#constructors)
#[derive(Debug, Clone)]
pub struct PrimaryConstructor {
    arguments: Vec<PrimaryConstructorParameter>,
    visibility_modifier: VisibilityModifier,
}

impl PrimaryConstructor {
    pub fn new() -> PrimaryConstructor {
        PrimaryConstructor {
            arguments: Vec::new(),
            visibility_modifier: VisibilityModifier::default(),
        }
    }

    pub fn parameter(mut self, parameter: Parameter) -> PrimaryConstructor {
        self.arguments.push(PrimaryConstructorParameter::FunctionLike(parameter));
        self
    }

    pub fn property(mut self, property: Property) -> PrimaryConstructor {
        self.arguments.push(PrimaryConstructorParameter::PropertyParameter(property));
        self
    }

    pub fn visibility_modifier(mut self, visibility_modifier: VisibilityModifier) -> PrimaryConstructor {
        self.visibility_modifier = visibility_modifier;
        self
    }
}

impl RenderKotlin for PrimaryConstructor {
    fn render_into(&self, block: &mut CodeBlock) {
        block.push_renderable(&self.visibility_modifier);
        block.push_space();
        block.push_atom(tokens::keyword::CONSTRUCTOR);
        block.push_round_brackets(|params_block| {
            params_block.push_comma_separated(&self.arguments)
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;
    use crate::spec::{VisibilityModifier, CodeBlock, Parameter, PrimaryConstructor, Property, Type};

    #[test]
    fn primary_constructor_test() {
        let property = Property::new(
            "name".into(),
            Type::string(),
        ).initializer(
            CodeBlock::atom("\"\"")
        );

        let function_parameter = Parameter::new(
            "age".into(),
            Type::int(),
        );

        let primary_constructor = PrimaryConstructor::new()
            .property(property)
            .parameter(function_parameter);

        assert_eq!(
            primary_constructor.render_string(),
            "public constructor(public final val name: kotlin.String = \"\", age: kotlin.Int)"
        );
    }

    #[test]
    fn test_private_constructor() {
        let primary_constructor = PrimaryConstructor::new()
            .visibility_modifier(VisibilityModifier::Private);

        assert_eq!(
            primary_constructor.render_string(),
            "private constructor()"
        );
    }
}