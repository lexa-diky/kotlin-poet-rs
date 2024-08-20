use crate::io::RenderKotlin;
use crate::spec::{AnnotationTarget, Argument, ClassLikeTypeName, CodeBlock};
use crate::tokens;

/// Represents an annotation in Kotlin. Used for adding meta information for code entities.
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
///     Argument::new_named("value", CodeBlock::atom("1"))
/// ).argument(
///     Argument::new_named("name", CodeBlock::atom("\"name_value\""))
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

#[derive(Debug, Clone)]
pub(crate) enum  AnnotationSlotRenderMode { Vertical, Horizontal }

#[derive(Debug, Clone)]
pub(crate) struct AnnotationSlot {
    inner: Vec<Annotation>,
    render_mode: AnnotationSlotRenderMode
}

impl AnnotationSlot {

    pub(crate) fn vertical() -> AnnotationSlot {
        AnnotationSlot {
            inner: Vec::new(),
            render_mode: AnnotationSlotRenderMode::Vertical
        }
    }

    pub(crate) fn horizontal() -> AnnotationSlot {
        AnnotationSlot {
            inner: Vec::new(),
            render_mode: AnnotationSlotRenderMode::Horizontal
        }
    }

    pub(crate) fn push(&mut self, new: Annotation) {
        self.inner.push(new)
    }
}

impl RenderKotlin for AnnotationSlot {
    fn render_into(&self, block: &mut CodeBlock) {
        for annotation in &self.inner {
            block.push_renderable(annotation);
            match self.render_mode {
                AnnotationSlotRenderMode::Vertical => {
                    block.push_new_line()
                }
                AnnotationSlotRenderMode::Horizontal => {
                    block.push_space()
                }
            }
        }
    }
}

impl Annotation {
    pub fn new<ClassLikeTypeNameLike: Into<ClassLikeTypeName>>(type_name: ClassLikeTypeNameLike) -> Self {
        Annotation {
            type_name: type_name.into(),
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
    fn render_into(&self, block: &mut CodeBlock) {
        block.push_static_atom(tokens::AT);
        if let Some(target) = &self.target {
            block.push_renderable(target);
            block.push_static_atom(tokens::COLON);
        }
        block.push_renderable(&self.type_name);
        block.push_round_brackets(|inner_code| {
            inner_code.push_comma_separated(&self.arguments)
        });
    }
}

macro_rules! mixin_annotation_mutators {
    () => {
        /// Adds [Annotation] to this entity.
        /// They will appear in order this method is called.
        pub fn annotation(mut self, annotation: Annotation) -> Self {
            self.annotation_slot.push(annotation);
            self
        }
    };
}

pub(crate) use mixin_annotation_mutators;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::io::RenderKotlin;
    use crate::spec::{Annotation, AnnotationTarget, Argument, ClassLikeTypeName, CodeBlock, Package};

    #[test]
    fn test_annotation() {
        let annotation = Annotation::new(
            ClassLikeTypeName::top_level(Package::from_str("a.b.c").unwrap(), "MyAnnotation")
        ).argument(
            Argument::new_named("value", CodeBlock::atom("1"))
        ).argument(
            Argument::new_named("name", CodeBlock::atom("\"name_value\""))
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
            ClassLikeTypeName::top_level(Package::from_str("a.b.c").unwrap(), "MyAnnotation")
        ).argument(
            Argument::new_named("value", CodeBlock::atom("1"))
        ).argument(
            Argument::new_named("name", CodeBlock::atom("\"name_value\""))
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