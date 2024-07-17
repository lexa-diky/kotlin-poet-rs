use crate::io::{RenderContext, RenderKotlin};
use crate::io::tokens::SEPARATOR;
use crate::spec::name::Name;
use crate::spec::package::Package;

#[derive(PartialEq, Debug, Clone)]
pub struct ClassLikeTypeName {
    package: Package,
    names: Vec<Name>,
}

impl ClassLikeTypeName {
    pub fn simple(package: Package, name: Name) -> ClassLikeTypeName {
        ClassLikeTypeName {
            package,
            names: vec![name],
        }
    }

    pub fn nested(package: Package, names: Vec<Name>) -> ClassLikeTypeName {
        ClassLikeTypeName {
            package,
            names,
        }
    }
}

impl RenderKotlin for ClassLikeTypeName {
    fn render(&self, context: RenderContext) -> String {
        let package = self.package.render(context);
        let names = self.names.iter().map(|it| it.render(context))
            .collect::<Vec<_>>().join(SEPARATOR);
        format!("{}.{}", package, names)
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
        assert_eq!(class_like_type_name.render_without_context(), "io.github.lexadiky.My.Class");
    }

    #[test]
    fn render_simple_kotlin() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let class_like_type_name = ClassLikeTypeName::simple(
            package,
            Name::from_str("Class").unwrap(),
        );
        assert_eq!(class_like_type_name.render_without_context(), "io.github.lexadiky.Class");
    }
}
