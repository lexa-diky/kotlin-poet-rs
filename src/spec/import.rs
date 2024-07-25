use crate::io::RenderKotlin;
use crate::spec::{ClassLikeTypeName, CodeBlock, Name, Package};
use crate::tokens;

/// Defines [Kotlin's import statement](https://kotlinlang.org/docs/packages.html#imports)
#[derive(Debug, PartialEq, Clone)]
pub enum Import {
    /// Import a class-like type possibly aliased with a different name
    ClassLikeType { type_name: ClassLikeTypeName, alias: Option<Name> },
    /// Import of all types from a package
    Projection(Package),
    /// Import of function / property
    TopLevel { package: Package, name: Name },
}

impl Import {
    /// Creates an import statement for a class-like type
    pub fn class_like(type_name: ClassLikeTypeName) -> Self {
        Import::ClassLikeType {
            type_name,
            alias: None,
        }
    }

    /// Creates an import statement for a class-like type with an alias
    pub fn class_like_alias(type_name: ClassLikeTypeName, alias: Name) -> Self {
        Import::ClassLikeType {
            type_name,
            alias: Some(alias),
        }
    }

    /// Creates an import statement for all types in a package
    pub fn projection(package: Package) -> Self {
        Import::Projection(package)
    }

    /// Creates an import statement for a function or property
    pub fn top_level(package: Package, name: Name) -> Self {
        Import::TopLevel {
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
                code.with_atom(tokens::DOT);
                code.with_atom(tokens::STAR);
            }
            Import::TopLevel { package, name } => {
                code.with_nested(package.render());
                code.with_atom(tokens::DOT);
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
        assert_eq!(import.render_string(), "import com.example.Foo");
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
        assert_eq!(import.render_string(), "import com.example.Foo as Bar");
    }

    #[test]
    fn test_import_projection() {
        let import = Import::Projection(Package::from_str("com.example").unwrap());
        assert_eq!(import.render_string(), "import com.example.*");
    }

    #[test]
    fn test_import_top_level() {
        let import = Import::TopLevel {
            package: Package::from_str("com.example").unwrap(),
            name: Name::from_str("foo").unwrap(),
        };
        assert_eq!(import.render_string(), "import com.example.foo");
    }
}
