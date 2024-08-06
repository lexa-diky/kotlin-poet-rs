use crate::io::RenderKotlin;
use crate::spec::{AnnotationTarget, Argument, ClassLikeTypeName, CodeBlock};
use crate::tokens;

/// Represents an annotation in Kotlin. Used for adding meta information for code entities.
/// Multiple/None annotations are represented with [Vec<Annotation>]
///
/// [Official documentation reference](https://kotlinlang.org/docs/annotations.html)
///
/// # Examples
///
/// ```rust
/// use std::str::FromStr;
/// use kotlin_poet_rs::io::RenderKotlin;
/// use kotlin_poet_rs::spec::{Annotation, Argument, ClassLikeTypeName, CodeBlock, Name, Package};
///
/// let annotation = Annotation::new(
///     ClassLikeTypeName::top_level(
///         Package::from_str("a.b.c").unwrap(),
///         Name::from("MyAnnotation")
///     )
/// ).argument(
///     Argument::new_named("value".into(), CodeBlock::atom("1"))
/// ).argument(
///     Argument::new_named("name".into(), CodeBlock::atom("\"name_value\""))
/// );
///
/// assert_eq!(
///     annotation.render_string(),
///     "@a.b.c.MyAnnotation(value = 1, name = \"name_value\")"
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Annotation {
    type_name: ClassLikeTypeName,
    arguments: Vec<Argument>,
    target: Option<AnnotationTarget>,
}

impl Annotation {
    pub fn new(type_name: ClassLikeTypeName) -> Self {
        Annotation {
            type_name,
            arguments: Vec::new(),
            target: None,
        }
    }

    pub fn argument(mut self, argument: Argument) -> Self {
        self.arguments.push(argument);
        self
    }

    pub fn target(mut self, target: AnnotationTarget) -> Self {
        self.target = Some(target);
        self
    }
}

impl RenderKotlin for Annotation {
    fn render(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();

        code.with_atom(tokens::AT);
        if let Some(target) = &self.target {
            code.with_nested(target.render());
            code.with_atom(tokens::COLON);
        }
        code.with_nested(self.type_name.render());
        code.with_round_brackets(|inner_code| {
            inner_code.with_comma_separated(&self.arguments)
        });

        code
    }
}

macro_rules! mixin_annotation_mutators {
    () => {
        /// Adds [Annotation] to this entity.
        /// They will appear in order this method is called.
        pub fn annotation(mut self, annotation: Annotation) -> Self {
            self.annotations.push(annotation);
            self
        }
    };
}

pub(crate) use mixin_annotation_mutators;

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::io::RenderKotlin;
    use crate::spec::{Annotation, AnnotationTarget, Argument, ClassLikeTypeName, CodeBlock, Package};

    #[test]
    fn test_annotation() {
        let annotation = Annotation::new(
            ClassLikeTypeName::top_level(Package::from_str("a.b.c").unwrap(), "MyAnnotation".into())
        ).argument(
            Argument::new_named("value".into(), CodeBlock::atom("1"))
        ).argument(
            Argument::new_named("name".into(), CodeBlock::atom("\"name_value\""))
        );

        let code = annotation.render_string();

        assert_eq!(
            code,
            "@a.b.c.MyAnnotation(value = 1, name = \"name_value\")"
        );
    }

    #[test]
    fn test_annotation_with_target() {
        let annotation = Annotation::new(
            ClassLikeTypeName::top_level(Package::from_str("a.b.c").unwrap(), "MyAnnotation".into())
        ).argument(
            Argument::new_named("value".into(), CodeBlock::atom("1"))
        ).argument(
            Argument::new_named("name".into(), CodeBlock::atom("\"name_value\""))
        ).target(
            AnnotationTarget::Field
        );

        let code = annotation.render_string();

        assert_eq!(
            code,
            "@field:a.b.c.MyAnnotation(value = 1, name = \"name_value\")"
        );
    }
}