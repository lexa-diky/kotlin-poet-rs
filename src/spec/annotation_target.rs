use crate::io::RenderKotlin;
use crate::spec::CodeBlock;
use crate::tokens;

/// Specifies exactly where this annotation will be applied.
/// Missing targets are represented as [Option<AnnotationTarget>]
///
/// [Kotlin documentation reference](https://kotlinlang.org/docs/annotations.html#annotation-use-site-targets)
#[derive(Debug, Clone)]
pub enum AnnotationTarget {
    /// Annotation applied to file
    File,
    /// Annotation applied to property.
    /// Annotations with this target are not visible to Java.
    /// If you need Java visibility use [AnnotationTarget::Field]
    Property,
    /// Annotation applied to backing field of the property
    Field,
    /// Property getter
    Get,
    /// Property setter
    Set,
    /// Receiver parameter of an extension function or property
    Receiver,
    /// Constructor parameter
    Param,
    /// Property setter parameter
    SetParam,
    /// The field storing the delegate instance for a delegated property
    Delegate,
}

impl RenderKotlin for AnnotationTarget {
    fn render_into(&self, block: &mut CodeBlock) {
        let atom = match self {
            AnnotationTarget::File => tokens::keyword::FILE,
            AnnotationTarget::Property => tokens::keyword::PROPERTY,
            AnnotationTarget::Field => tokens::keyword::FIELD,
            AnnotationTarget::Get => tokens::keyword::GET,
            AnnotationTarget::Set => tokens::keyword::SET,
            AnnotationTarget::Receiver => tokens::keyword::RECEIVER,
            AnnotationTarget::Param => tokens::keyword::PARAM,
            AnnotationTarget::SetParam => tokens::keyword::SET_PARAM,
            AnnotationTarget::Delegate => tokens::keyword::DELEGATE,
        };
        block.with_atom(atom)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;
    use crate::spec::AnnotationTarget;

    #[test]
    fn test_annotation_target() {
        assert_eq!(AnnotationTarget::Field.render_string(), "field");
        assert_eq!(AnnotationTarget::File.render_string(), "file");
        assert_eq!(AnnotationTarget::Get.render_string(), "get");
        assert_eq!(AnnotationTarget::Param.render_string(), "param");
        assert_eq!(AnnotationTarget::Property.render_string(), "property");
        assert_eq!(AnnotationTarget::Receiver.render_string(), "receiver");
        assert_eq!(AnnotationTarget::Set.render_string(), "set");
        assert_eq!(AnnotationTarget::SetParam.render_string(), "setparam");
        assert_eq!(AnnotationTarget::Delegate.render_string(), "delegate");
    }
}