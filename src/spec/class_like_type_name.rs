use crate::io::RenderKotlin;
use crate::io::tokens::{NOTHING, NULLABLE, SEPARATOR};
use crate::spec::name::Name;
use crate::spec::package::Package;

#[derive(PartialEq, Debug, Clone)]
pub struct ClassLikeTypeName {
    package: Package,
    names: Vec<Name>,
    nullable: bool,
}

impl ClassLikeTypeName {
    pub fn simple(package: Package, name: Name) -> ClassLikeTypeName {
        ClassLikeTypeName {
            package,
            names: vec![name],
            nullable: false,
        }
    }

    pub fn nested(package: Package, names: Vec<Name>) -> ClassLikeTypeName {
        ClassLikeTypeName {
            package,
            names,
            nullable: false,
        }
    }

    pub fn nullable(mut self, flag: bool) -> Self {
        self.nullable = flag;
        self
    }
}

impl RenderKotlin for ClassLikeTypeName {
    fn render(&self) -> String {
        let package = self.package.render();
        let names = self.names.iter().map(|it| it.render())
            .collect::<Vec<_>>().join(SEPARATOR);
        let nullability = if self.nullable { NULLABLE } else { NOTHING };
        format!("{}.{}{}", package, names, nullability)
    }
}

#[cfg(test)]
mod test {
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
        assert_eq!(class_like_type_name.render(), "io.github.lexadiky.My.Class");
    }

    #[test]
    fn render_simple_kotlin() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let class_like_type_name = ClassLikeTypeName::simple(
            package,
            Name::from_str("Class").unwrap(),
        );
        assert_eq!(class_like_type_name.render(), "io.github.lexadiky.Class");
    }

    #[test]
    fn render_simple_nullable_kotlin() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let mut class_like_type_name = ClassLikeTypeName::simple(
            package,
            Name::from_str("Class").unwrap()
        ).nullable(true);
        assert_eq!(class_like_type_name.render(), "io.github.lexadiky.Class?");
    }
}
