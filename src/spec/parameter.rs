use crate::io::RenderKotlin;
use crate::spec::{Annotation, CodeBlock, Name, Type};
use crate::spec::annotation::{mixin_annotation_mutators, AnnotationSlot};
use crate::tokens;

#[derive(Debug, Clone)]
pub struct Parameter {
    name: Name,
    parameter_type: Type,
    default_value: Option<CodeBlock>,
    annotation_slot: AnnotationSlot,
}

impl RenderKotlin for Parameter {
    fn render_into(&self, block: &mut CodeBlock) {
        block.push_renderable(&self.annotation_slot);
        block.push_renderable(&self.name);
        block.push_static_atom(tokens::COLON);
        block.push_space();
        block.push_renderable(&self.parameter_type);
        if let Some(default_value) = &self.default_value {
            block.push_space();
            block.push_static_atom(tokens::ASSIGN);
            block.push_space();
            block.push_renderable(default_value);
        }
    }
}

impl Parameter {
    pub fn new<NameLike: Into<Name>, TypeLike: Into<Type>>(name: NameLike, parameter_type: TypeLike) -> Parameter {
        Parameter {
            name: name.into(),
            parameter_type: parameter_type.into(),
            default_value: None,
            annotation_slot: AnnotationSlot::horizontal(),
        }
    }

    pub fn default_value<CodeBlockLike: Into<CodeBlock>>(mut self, default_value: CodeBlockLike) -> Parameter {
        self.default_value = Some(default_value.into());
        self
    }

    mixin_annotation_mutators!();
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
            Type::string(),
        );

        assert_eq!(
            "name: kotlin.String",
            parameter.render_string()
        )
    }

    #[test]
    fn test_rendering_with_default() {
        let parameter = Parameter::new(
            Name::from("age"),
            Type::int(),
        ).default_value(CodeBlock::atom("25"));

        assert_eq!(
            "age: kotlin.Int = 25",
            parameter.render_string()
        )
    }

    #[test]
    fn test_rendering_with_annotation() {
        let parameter = Parameter::new(
            Name::from("age"),
            Type::int(),
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
            parameter.render_string()
        )
    }
}