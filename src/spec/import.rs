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
    pub fn class_like<ClassLikeTypeNameLike: Into<ClassLikeTypeName>>(type_name: ClassLikeTypeNameLike) -> Self {
        Import::ClassLikeType {
            type_name: type_name.into(),
            alias: None,
        }
    }

    /// Creates an import statement for a class-like type with an alias
    pub fn class_like_alias<NameLike: Into<Name>, ClassLikeTypeNameLike: Into<ClassLikeTypeName>>(type_name: ClassLikeTypeNameLike, alias: NameLike) -> Self {
        Import::ClassLikeType {
            type_name: type_name.into(),
            alias: Some(alias.into()),
        }
    }

    /// Creates an import statement for all types in a package
    pub fn projection<PackageLike: Into<Package>>(package: PackageLike) -> Self {
        Import::Projection(package.into())
    }

    /// Creates an import statement for a function or property
    pub fn top_level<NameLike: Into<Name>, PackageLike: Into<Package>>(
        package: PackageLike,
        name: NameLike
    ) -> Self {
        Import::TopLevel {
            package: package.into(),
            name: name.into(),
        }
    }
}

impl RenderKotlin for Import {
    fn render_into(&self, block: &mut CodeBlock) {
        block.push_atom(tokens::keyword::IMPORT);
        block.push_space();

        match self {
            Import::ClassLikeType { type_name, alias } => {
                block.push_renderable(type_name);
                if let Some(alias) = alias {
                    block.push_space();
                    block.push_atom(tokens::keyword::AS);
                    block.push_space();
                    block.push_renderable(alias);
                }
            }
            Import::Projection(package) => {
                block.push_renderable(package);
                block.push_atom(tokens::DOT);
                block.push_atom(tokens::STAR);
            }
            Import::TopLevel { package, name } => {
                block.push_renderable(package);
                block.push_atom(tokens::DOT);
                block.push_renderable(name);
            }
        }

        block.push_new_line();
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
