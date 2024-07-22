use crate::io::RenderKotlin;
use crate::spec::{CodeBlock, Name, Type};
use crate::tokens;

#[derive(Debug, Clone)]
pub struct TypeAlias {
    pub name: Name,
    pub generic_parameters: Vec<Name>,
    pub actual: Type,
}

impl TypeAlias {

    pub fn new(name: Name, actual: Type) -> Self {
        TypeAlias {
            name,
            generic_parameters: Vec::new(),
            actual,
        }
    }

    pub fn generic_parameter(mut self, name: Name) -> Self {
        self.generic_parameters.push(name);
        self
    }
}

impl RenderKotlin for TypeAlias {
    fn render(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();
        code.with_atom(tokens::keyword::TYPEALIAS);
        code.with_space();
        code.with_nested(self.name.render());
        if !self.generic_parameters.is_empty() {
            code.with_atom(tokens::ANGLE_BRACKET_LEFT);
            code.with_comma_separated(&self.generic_parameters);
            code.with_atom(tokens::ANGLE_BRACKET_RIGHT);
        }

        code.with_space();
        code.with_atom(tokens::EQUALS);
        code.with_space();
        code.with_nested(self.actual.render());
        code
    }
}

#[cfg(test)]
mod test {
    use crate::spec::Type;

    use super::*;

    #[test]
    fn type_alias() {
        let alias = TypeAlias::new(
            Name::from("MyType"),
            Type::string(),
        );

        let actual = alias.render().to_string();
        let expected = "typealias MyType = kotlin.String";
        assert_eq!(actual, expected);
    }

    #[test]
    fn type_alias_with_generic() {
        let alias = TypeAlias::new(
            Name::from("Vec"),
            Type::list(Type::generic("T")),
        ).generic_parameter(Name::from("T"));

        let actual = alias.render().to_string();
        let expected = "typealias Vec<T> = kotlin.collections.List<T>";
        assert_eq!(actual, expected);
    }
}
