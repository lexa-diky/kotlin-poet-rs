use std::fmt::format;
use crate::io::{RenderContext, RenderKotlin};
use crate::io::tokens::{KW_IMPORT, PROJECTION};
use crate::spec::{ClassLikeTypeName, Name, Package};

#[derive(Debug, PartialEq, Clone)]
pub enum Import {
    ClassLikeType { type_name: ClassLikeTypeName, alias: Option<Name> },
    Projection(Package),
    Function { package: Package, name: Name }
}

impl Import {
    pub fn class_like(type_name: ClassLikeTypeName) -> Self {
        Import::ClassLikeType {
            type_name,
            alias: None
        }
    }

    pub fn class_like_alias(type_name: ClassLikeTypeName, alias: Name) -> Self {
        Import::ClassLikeType {
            type_name,
            alias: Some(alias)
        }
    }

    pub fn projection(package: Package) -> Self {
        Import::Projection(package)
    }

    pub fn function(package: Package, name: Name) -> Self {
        Import::Function {
            package,
            name
        }
    }
}

impl RenderKotlin for Import {
    fn render(&self, context: RenderContext) -> String {
        match self {
            Import::ClassLikeType { type_name, alias } => {
                if let Some(alias) = alias {
                    format!("{} {} as {}", KW_IMPORT, type_name.render(context), alias.render(context))
                } else {
                    format!("{} {}", KW_IMPORT, type_name.render(context))
                }
            }
            Import::Projection(package) => {
                format!("{} {}.{}", KW_IMPORT, package.render(context), PROJECTION)
            }
            Import::Function { package, name } => {
                format!("{} {}.{}", KW_IMPORT, package.render(context), name.render(context))
            }
        }
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
        assert_eq!(import.render_without_context(), "import com.example.Foo");
    }

    #[test]
    fn test_import_class_like_type_with_alias() {
        let import = Import::class_like_alias(
            ClassLikeTypeName::simple(
                Package::from_str("com.example").unwrap(),
                Name::from_str("Foo").unwrap(),
            ),
            Name::from("Bar")
        );
        assert_eq!(import.render_without_context(), "import com.example.Foo as Bar");
    }

    #[test]
    fn test_import_projection() {
        let import = Import::Projection(Package::from_str("com.example").unwrap());
        assert_eq!(import.render_without_context(), "import com.example.*");
    }

    #[test]
    fn test_import_function() {
        let import = Import::Function {
            package: Package::from_str("com.example").unwrap(),
            name: Name::from_str("foo").unwrap(),
        };
        assert_eq!(import.render_without_context(), "import com.example.foo");
    }
}
