use crate::io::RenderKotlin;
use crate::spec::class_like_type::ClassLikeType;
use crate::spec::{ClassLikeTypeName, Name, Package, TypeName};

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    ClassLike(ClassLikeType),
    Generic(Name),
}

impl Type {
    pub fn array(generic_argument: Type) -> Type {
        Type::ClassLike(
            ClassLikeType::new(
                ClassLikeTypeName::simple(
                    Package::from(
                        vec![
                            Name::from("kotlin")
                        ]
                    ),
                    Name::from("Array"),
                )
            ).generic_argument(generic_argument)
        )
    }

    pub fn unit() -> Type {
        Type::ClassLike(
            ClassLikeType::new(
                ClassLikeTypeName::simple(
                    Package::from(
                        vec![
                            Name::from("kotlin")
                        ]
                    ),
                    Name::from("Unit"),
                )
            )
        )
    }

    pub fn string() -> Type {
        Type::ClassLike(
            ClassLikeType::new(
                ClassLikeTypeName::simple(
                    Package::from(
                        vec![
                            Name::from("kotlin")
                        ]
                    ),
                    Name::from("String"),
                )
            )
        )
    }
}
impl RenderKotlin for Type {
    fn render(&self) -> String {
        match self {
            Type::ClassLike(class_like) => class_like.render(),
            Type::Generic(name) => name.render()
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::io::RenderKotlin;
    use crate::spec::{Name, Type};

    #[test]
    fn render_generic_parameter() {
        let name = Name::from_str("T").unwrap();
        let parameter = Type::Generic(name);
        assert_eq!(parameter.render(), "T");
    }
}