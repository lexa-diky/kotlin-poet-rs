use crate::io::RenderKotlin;
use crate::spec::{Annotation, CodeBlock, KDoc, Name, Type, VisibilityModifier};
use crate::spec::annotation::mixin_annotation_mutators;
use crate::spec::kdoc::{KdocSlot, mixin_kdoc_mutators};
use crate::tokens;

/// Kotlin's `typealias` declaration
#[derive(Debug, Clone)]
pub struct TypeAlias {
    name: Name,
    generic_parameters: Vec<Name>,
    actual: Type,
    visibility_modifier: VisibilityModifier,
    annotations: Vec<Annotation>,
    kdoc: KdocSlot
}

impl TypeAlias {

    /// Creates [TypeAlias] binding [name] to [actual] [Type]
    pub fn new(name: Name, actual: Type) -> TypeAlias {
        TypeAlias {
            name,
            generic_parameters: Vec::default(),
            actual,
            visibility_modifier: VisibilityModifier::default(),
            annotations: Vec::default(),
            kdoc: KdocSlot::default()
        }
    }

    /// Adds generic parameter to the type alias
    /// Multiple generic parameters can be added, they will appear in order this method is called.
    pub fn generic_parameter(mut self, name: Name) -> Self {
        self.generic_parameters.push(name);
        self
    }

    /// Sets [VisibilityModifier]. [VisibilityModifier::Protected] is not allowed
    pub fn visibility_modifier(mut self, visibility_modifier: VisibilityModifier) -> Self {
        self.visibility_modifier = visibility_modifier;
        self
    }

    mixin_annotation_mutators!();
    mixin_kdoc_mutators!();
}

impl RenderKotlin for TypeAlias {
    fn render_into(&self, block: &mut CodeBlock) {
        block.push_renderable(&self.kdoc);

        for annotation in &self.annotations {
            block.push_renderable(annotation);
            block.push_new_line();
        }
        block.push_renderable(&self.visibility_modifier);
        block.push_space();
        block.push_atom(tokens::keyword::TYPEALIAS);
        block.push_space();
        block.push_renderable(&self.name);
        if !self.generic_parameters.is_empty() {
            block.push_atom(tokens::ANGLE_BRACKET_LEFT);
            block.push_comma_separated(&self.generic_parameters);
            block.push_atom(tokens::ANGLE_BRACKET_RIGHT);
        }

        block.push_space();
        block.push_atom(tokens::ASSIGN);
        block.push_space();
        block.push_renderable(&self.actual);
    }
}

#[cfg(test)]
mod test {
    use crate::spec::{ClassLikeTypeName, Package, Type};

    use super::*;

    #[test]
    fn type_alias() {
        let alias = TypeAlias::new(
            Name::from("MyType"),
            Type::string(),
        );

        let actual = alias.render_string();
        let expected = "public typealias MyType = kotlin.String";
        assert_eq!(actual, expected);
    }

    #[test]
    fn type_alias_with_kdoc() {
        let alias = TypeAlias::new(
            Name::from("MyType"),
            Type::string(),
        ).kdoc(
            KDoc::from("Hello\nWorld")
        );

        let actual = alias.render_string();
        let expected = "/**\n * Hello\n * World\n */\npublic typealias MyType = kotlin.String";
        assert_eq!(actual, expected);
    }

    #[test]
    fn private_type_alias() {
        let alias = TypeAlias::new(
            Name::from("MyType"),
            Type::string(),
        ).visibility_modifier(VisibilityModifier::Private);

        let actual = alias.render_string();
        let expected = "private typealias MyType = kotlin.String";
        assert_eq!(actual, expected);
    }

    #[test]
    fn type_alias_with_generic() {
        let alias = TypeAlias::new(
            Name::from("Vec"),
            Type::list(Type::generic("T")),
        ).generic_parameter(Name::from("T"))
            .generic_parameter(Name::from("B"));

        let actual = alias.render_string();
        let expected = "public typealias Vec<T, B> = kotlin.collections.List<T>";
        assert_eq!(actual, expected);
    }

    #[test]
    fn type_alias_with_annotation() {
        let alias = TypeAlias::new(
            Name::from("Vec"),
            Type::list(Type::string()),
        ).annotation(
            Annotation::new(
                ClassLikeTypeName::top_level(
                    Package::root(),
                    Name::from("JvmName")
                )
            )
        );

        let actual = alias.render_string();
        let expected = "@JvmName()\npublic typealias Vec = kotlin.collections.List<kotlin.String>";
        assert_eq!(actual, expected);
    }
}
