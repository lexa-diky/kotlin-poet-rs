use crate::io::{RenderKotlin, tokens};
use crate::spec::{ClassLikeTypeName, CodeBlock, Name, Package};

#[derive(Debug, PartialEq, Clone)]
pub enum Import {
    ClassLikeType { type_name: ClassLikeTypeName, alias: Option<Name> },
    Projection(Package),
    Function { package: Package, name: Name },
}

impl Import {
    pub fn class_like(type_name: ClassLikeTypeName) -> Self {
        Import::ClassLikeType {
            type_name,
            alias: None,
        }
    }

    pub fn class_like_alias(type_name: ClassLikeTypeName, alias: Name) -> Self {
        Import::ClassLikeType {
            type_name,
            alias: Some(alias),
        }
    }

    pub fn projection(package: Package) -> Self {
        Import::Projection(package)
    }

    pub fn function(package: Package, name: Name) -> Self {
        Import::Function {
            package,
            name,
        }
    }
}

impl RenderKotlin for Import {
    fn render(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();
        code.with_atom(tokens::keyword::IMPORT);
        code.with_space();

        match self {
            Import::ClassLikeType { type_name, alias } => {
                code.with_nested(type_name.render());
                if let Some(alias) = alias {
                    code.with_space();
                    code.with_atom(tokens::keyword::AS);
                    code.with_space();
                    code.with_nested(alias.render());
                }
            }
            Import::Projection(package) => {
                code.with_nested(package.render());
                code.with_atom(tokens::SEPARATOR);
                code.with_atom(tokens::WILDCARD);
            }
            Import::Function { package, name } => {
                code.with_nested(package.render());
                code.with_atom(tokens::SEPARATOR);
                code.with_nested(name.render());
            }
        }

        code.with_new_line();

        code
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::*;
    use crate::spec::{ClassLikeTypeName, Name};

    #[test]
    fn test_import_class_like_type() {
        let import = Import::class_like(
            ClassLikeTypeName::simple(
                Package::from_str("com.example").unwrap(),
                Name::from_str("Foo").unwrap(),
            )
        );
        assert_eq!(import.render_string_in_root(), "import com.example.Foo\n");
    }

    #[test]
    fn test_import_class_like_type_with_alias() {
        let import = Import::class_like_alias(
            ClassLikeTypeName::simple(
                Package::from_str("com.example").unwrap(),
                Name::from_str("Foo").unwrap(),
            ),
            Name::from("Bar"),
        );
        assert_eq!(import.render_string_in_root(), "import com.example.Foo as Bar\n");
    }

    #[test]
    fn test_import_projection() {
        let import = Import::Projection(Package::from_str("com.example").unwrap());
        assert_eq!(import.render_string_in_root(), "import com.example.*\n");
    }

    #[test]
    fn test_import_function() {
        let import = Import::Function {
            package: Package::from_str("com.example").unwrap(),
            name: Name::from_str("foo").unwrap(),
        };
        assert_eq!(import.render_string_in_root(), "import com.example.foo\n");
    }
}
