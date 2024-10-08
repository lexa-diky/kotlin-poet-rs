use std::str::FromStr;

use crate::io::RenderKotlin;
use crate::spec::{CodeBlock, Name, Package};
use crate::tokens;
use crate::util::{SemanticConversionError, yolo_from_str};

/// Represents a class-like type name.
///
/// This struct does not support generics, nullability or any other possible type 'modifiers'.
/// You can use [ClassLikeType] instead.
#[derive(PartialEq, Debug, Clone)]
pub struct ClassLikeTypeName {
    pub(crate) package: Package,
    names: Vec<Name>,
}

impl ClassLikeTypeName {
    /// Creates top level class name, for example `com.example.MyClass`
    pub fn top_level<NameLike: Into<Name>, PackageLike: Into<Package>>(
        package: PackageLike,
        name: NameLike
    ) -> ClassLikeTypeName {
        ClassLikeTypeName {
            package: package.into(),
            names: vec![name.into()],
        }
    }

    /// Creates nested class name, for example `com.example.MyClass.InnerClass`
    pub fn nested<PackageLike: Into<Package>>(package: PackageLike, names: Vec<Name>) -> ClassLikeTypeName {
        ClassLikeTypeName {
            package: package.into(),
            names,
        }
    }
}

yolo_from_str!(ClassLikeTypeName);
impl FromStr for ClassLikeTypeName {
    type Err = SemanticConversionError;

    #[allow(clippy::comparison_chain)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() > 1 {
            let mut package_parts = Vec::new();
            for part in &parts[0..parts.len() - 1] {
                package_parts.push(Name::from_str(part)?)
            }

            let package = Package::from(package_parts);
            let name = Name::from_str(parts[parts.len() - 1])?;

            Ok(ClassLikeTypeName::top_level(
                package,
                name,
            )
            )
        } else if parts.len() == 1 {
            Ok(
                ClassLikeTypeName::top_level(
                    Package::from(vec![]),
                    Name::from(parts[0]),
                )
            )
        } else {
            Err(
                SemanticConversionError::new(
                    format!("Can't convert {s} to ClassLikeTypeName").as_str()
                )
            )
        }
    }
}

impl RenderKotlin for ClassLikeTypeName {
    fn render_into(&self, block: &mut CodeBlock) {
        if !self.package.parts.is_empty() {
            block.push_renderable(&self.package);
            block.push_static_atom(tokens::DOT);
        }

        for (index, part) in self.names.iter().enumerate() {
            block.push_renderable(part);
            if index != self.names.len() - 1 {
                block.push_static_atom(tokens::DOT);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::io::RenderKotlin;
    use crate::spec::Name;
    use crate::spec::package::Package;

    use super::ClassLikeTypeName;

    #[test]
    fn render_nested_kotlin() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let class_like_type_name = ClassLikeTypeName::nested(
            package, vec![
                Name::from_str("My").unwrap(),
                Name::from_str("Class").unwrap(),
            ],
        );
        assert_eq!(class_like_type_name.render_string(), "io.github.lexadiky.My.Class");
    }

    #[test]
    fn render_simple_kotlin() {
        let class_like_type_name = ClassLikeTypeName::top_level(
            "io.github.lexadiky",
            Name::from_str("Class").unwrap(),
        );
        assert_eq!(class_like_type_name.render_string(), "io.github.lexadiky.Class");
    }

    #[test]
    fn test_from_string_long() {
        let class_like_type = ClassLikeTypeName::from_str("io.github.lexadiky.Class").unwrap();
        assert_eq!(class_like_type.render_string(), "io.github.lexadiky.Class");
    }

    #[test]
    fn test_from_string_short() {
        let class_like_type = ClassLikeTypeName::from_str("github.Class").unwrap();
        assert_eq!(class_like_type.render_string(), "github.Class");
    }

    #[test]
    fn test_from_string_top_level() {
        let class_like_type = ClassLikeTypeName::from_str("Class").unwrap();
        assert_eq!(class_like_type.render_string(), "Class");
    }
}
