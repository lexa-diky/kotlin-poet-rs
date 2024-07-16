use crate::io::RenderKotlin;
use crate::io::tokens::{NOTHING, NULLABLE, SEPARATOR};
use crate::spec::package::Package;

#[derive(PartialEq, Debug, Clone)]
pub struct ClassLikeTypeName {
    package: Package,
    names: Vec<String>,
    nullable: bool,
}

impl ClassLikeTypeName {
    pub fn simple(package: Package, name: &str) -> ClassLikeTypeName {
        ClassLikeTypeName {
            package,
            names: vec![name.to_string()],
            nullable: false,
        }
    }

    pub fn nested(package: Package, names: Vec<String>) -> ClassLikeTypeName {
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
        let names = self.names.join(SEPARATOR);
        let nullability = if self.nullable { NULLABLE } else { NOTHING };
        format!("{}.{}{}", package, names, nullability)
    }
}

#[cfg(test)]
mod test {
    use crate::io::RenderKotlin;
    use crate::spec::package::Package;
    use super::ClassLikeTypeName;

    #[test]
    fn render_nested_kotlin() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let class_like_type_name = ClassLikeTypeName::nested(
            package, vec![
                "My".to_string(),
                "Class".to_string(),
            ],
        );
        assert_eq!(class_like_type_name.render(), "io.github.lexadiky.My.Class");
    }

    #[test]
    fn render_simple_kotlin() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let class_like_type_name = ClassLikeTypeName::simple(package, "Class");
        assert_eq!(class_like_type_name.render(), "io.github.lexadiky.Class");
    }

    #[test]
    fn render_simple_nullable_kotlin() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let mut class_like_type_name = ClassLikeTypeName::simple(package, "Class")
            .nullable(true);
        assert_eq!(class_like_type_name.render(), "io.github.lexadiky.Class?");
    }
}
