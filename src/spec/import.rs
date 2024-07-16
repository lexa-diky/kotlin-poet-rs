use crate::io::RenderKotlin;
use crate::io::tokens::{KW_IMPORT, PROJECTION};
use crate::spec::{ClassLikeTypeName, Package};

#[derive(Debug, PartialEq, Clone)]
pub enum Import {
    ClassLikeType(ClassLikeTypeName),
    Projection(Package),
}

impl Import {
    pub fn class_like(type_name: ClassLikeTypeName) -> Self {
        Import::ClassLikeType(type_name)
    }

    pub fn projection(package: Package) -> Self {
        Import::Projection(package)
    }
}

impl RenderKotlin for Import {
    fn render(&self) -> String {
        match self {
            Import::ClassLikeType(type_name) => {
                format!("{} {}", KW_IMPORT, type_name.render())
            }
            Import::Projection(package) => {
                format!("{} {}.{}", KW_IMPORT, package.render(), PROJECTION)
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
        let import = Import::ClassLikeType(
            ClassLikeTypeName::simple(
                Package::from_str("com.example").unwrap(),
                Name::from_str("Foo").unwrap(),
            )
        );
        assert_eq!(import.render(), "import com.example.Foo");
    }

    #[test]
    fn test_import_projection() {
        let import = Import::Projection(Package::from_str("com.example").unwrap());
        assert_eq!(import.render(), "import com.example.*");
    }
}
