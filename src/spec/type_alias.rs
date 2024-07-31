use crate::io::RenderKotlin;
use crate::spec::{Annotation, CodeBlock, Name, Type, VisibilityModifier};
use crate::tokens;

/// Kotlin's `typealias` declaration
#[derive(Debug, Clone)]
pub struct TypeAlias {
    name: Name,
    generic_parameters: Vec<Name>,
    actual: Type,
    visibility_modifier: VisibilityModifier,
    annotations: Vec<Annotation>
}

impl TypeAlias {

    /// Creates [TypeAlias] binding [name] to [actual] [Type]
    pub fn new(name: Name, actual: Type) -> TypeAlias {
        TypeAlias {
            name,
            generic_parameters: Vec::new(),
            actual,
            visibility_modifier: VisibilityModifier::Public,
            annotations: Vec::new()
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

    /// Adds annotation to the type alias
    pub fn annotation(mut self, annotation: Annotation) -> Self {
        self.annotations.push(annotation);
        self
    }
}

impl RenderKotlin for TypeAlias {
    fn render(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();
        for annotation in &self.annotations {
            code.with_nested(annotation.render());
            code.with_new_line();
        }
        code.with_nested(self.visibility_modifier.render());
        code.with_space();
        code.with_atom(tokens::keyword::TYPEALIAS);
        code.with_space();
        code.with_nested(self.name.render());
        if !self.generic_parameters.is_empty() {
            code.with_atom(tokens::ANGLE_BRACKET_LEFT);
            code.with_comma_separated(&self.generic_parameters);
            code.with_atom(tokens::ANGLE_BRACKET_RIGHT);
        }

        code.with_space();
        code.with_atom(tokens::ASSIGN);
        code.with_space();
        code.with_nested(self.actual.render());
        code
    }
}

#[cfg(test)]
mod test {
    use crate::spec::{Class, ClassLikeTypeName, Package, Type};

    use super::*;

    #[test]
    fn type_alias() {
        let alias = TypeAlias::new(
            Name::from("MyType"),
            Type::string(),
        );

        let actual = alias.render().to_string();
        let expected = "public typealias MyType = kotlin.String";
        assert_eq!(actual, expected);
    }

    #[test]
    fn private_type_alias() {
        let alias = TypeAlias::new(
            Name::from("MyType"),
            Type::string(),
        ).visibility_modifier(VisibilityModifier::Private);

        let actual = alias.render().to_string();
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

        let actual = alias.render().to_string();
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

        let actual = alias.render().to_string();
        let expected = "@JvmName()\npublic typealias Vec = kotlin.collections.List<kotlin.String>";
        assert_eq!(actual, expected);
    }
}
