use crate::io::{RenderContext, RenderKotlin};
use crate::spec::class_like_type::ClassLikeType;
use crate::spec::{ClassLikeTypeName, CodeBlock, LambdaType, Name, Package};

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    ClassLike(ClassLikeType),
    Lambda(LambdaType),
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

    pub fn list(generic_argument: Type) -> Type {
        Type::ClassLike(
            ClassLikeType::new(
                ClassLikeTypeName::simple(
                    Package::from(
                        vec![
                            Name::from("kotlin"),
                            Name::from("collections"),
                        ]
                    ),
                    Name::from("List"),
                )
            ).generic_argument(generic_argument)
        )
    }

    pub fn unit() -> Type {
        Self::basic_type("Unit")
    }

    pub fn string() -> Type {
        Self::basic_type("String")
    }

    pub fn int() -> Type {
        Self::basic_type("Int")
    }

    pub fn double() -> Type {
        Self::basic_type("Double")
    }

    pub fn float() -> Type {
        Self::basic_type("Float")
    }

    pub fn byte() -> Type {
        Self::basic_type("Byte")
    }

    pub fn short() -> Type {
        Self::basic_type("Short")
    }

    pub fn boolean() -> Type {
        Self::basic_type("Boolean")
    }

    fn basic_type(name: &str) -> Type {
        Type::ClassLike(
            ClassLikeType::new(
                ClassLikeTypeName::simple(
                    Package::from(
                        vec![
                            Name::from("kotlin")
                        ]
                    ),
                    Name::from(name),
                )
            )
        )
    }
}

impl RenderKotlin for Type {
    fn render(&self, context: RenderContext) -> CodeBlock {
        match self {
            Type::ClassLike(class_like) => class_like.render(context),
            Type::Generic(name) => name.render(context),
            Type::Lambda(lambda) => lambda.render(context)
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
        assert_eq!(parameter.render_without_context(), "T");
    }
}