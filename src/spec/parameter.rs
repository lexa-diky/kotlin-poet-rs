use crate::io::RenderKotlin;
use crate::spec::{Annotation, CodeBlock, Name, Type};
use crate::tokens;

#[derive(Debug, Clone)]
pub struct Parameter {
    name: Name,
    parameter_type: Type,
    default_value: Option<CodeBlock>,
    annotations: Vec<Annotation>
}

impl RenderKotlin for Parameter {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();
        for annotation in& self.annotations {
            block.with_nested(annotation.render());
            block.with_space();
        }
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
            annotations: Vec::new(),
        }
    }

    pub fn default_value(mut self, default_value: CodeBlock) -> Parameter {
        self.default_value = Some(default_value);
        self
    }

    pub fn annotation(mut self, annotation: Annotation) -> Parameter {
        self.annotations.push(annotation);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::io::RenderKotlin;
    use crate::spec::{Annotation, ClassLikeTypeName, CodeBlock, Name, Parameter, Type};

    #[test]
    fn test_rendering() {
        let parameter = Parameter::new(
            Name::from("name"),
            Type::string()
        );

        assert_eq!(
            "name: kotlin.String",
            parameter.render().to_string()
        )
    }

    #[test]
    fn test_rendering_with_default() {
        let parameter = Parameter::new(
            Name::from("age"),
            Type::int()
        ).default_value(CodeBlock::atom("25"));

        assert_eq!(
            "age: kotlin.Int = 25",
            parameter.render().to_string()
        )
    }

    #[test]
    fn test_rendering_with_annotation() {
        let parameter = Parameter::new(
            Name::from("age"),
            Type::int()
        ).annotation(
            Annotation::new(
                ClassLikeTypeName::from_str("io.github.lexadiky.MyAnnotation")
                    .unwrap()
            )
        ).annotation(
            Annotation::new(
                ClassLikeTypeName::from_str("io.github.lexadiky.OtherAnnotation")
                    .unwrap()
            )
        );

        assert_eq!(
            "@io.github.lexadiky.MyAnnotation() @io.github.lexadiky.OtherAnnotation() age: kotlin.Int",
            parameter.render().to_string()
        )
    }
}