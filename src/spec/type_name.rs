use crate::io::RenderKotlin;
use crate::spec::class_like_type_name::ClassLikeTypeName;

pub enum TypeName {
    ClassLike(ClassLikeTypeName),
}

impl RenderKotlin for TypeName {

    fn render(&self) -> String {
        match self {
            TypeName::ClassLike(class_like) => class_like.render()
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::io::RenderKotlin;
    use crate::spec::class_like_type_name::ClassLikeTypeName;
    use crate::spec::Name;
    use super::TypeName;
    use crate::spec::package::Package;

    #[test]
    fn render_simple_class_like() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let class_like_type_name = ClassLikeTypeName::simple(
            package,
            Name::from_str("Class").unwrap()
        );
        let type_name = TypeName::ClassLike(class_like_type_name);
        assert_eq!(type_name.render(), "io.github.lexadiky.Class");
    }
}