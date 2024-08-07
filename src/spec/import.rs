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
    fn render_into(&self, block: &mut CodeBlock) {
        block.with_atom(tokens::keyword::IMPORT);
        block.with_space();

        match self {
            Import::ClassLikeType { type_name, alias } => {
                block.with_embedded(type_name);
                if let Some(alias) = alias {
                    block.with_space();
                    block.with_atom(tokens::keyword::AS);
                    block.with_space();
                    block.with_embedded(alias);
                }
            }
            Import::Projection(package) => {
                block.with_embedded(package);
                block.with_atom(tokens::DOT);
                block.with_atom(tokens::STAR);
            }
            Import::TopLevel { package, name } => {
                block.with_embedded(package);
                block.with_atom(tokens::DOT);
                block.with_embedded(name);
            }
        }

        block.with_new_line();
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
            ClassLikeTypeName::from_str("com.example.Foo").unwrap()
        );
        assert_eq!(import.render_string(), "import com.example.Foo");
    }

    #[test]
    fn test_import_class_like_type_with_alias() {
        let import = Import::class_like_alias(
            ClassLikeTypeName::top_level(
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
