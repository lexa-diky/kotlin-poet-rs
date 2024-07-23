use crate::io::{RenderKotlin};
use crate::spec::class_like_type::ClassLikeType;
use crate::spec::{ClassLikeTypeName, CodeBlock, FunctionType, Name, Package};

/// Kotlin fully resolved / qualified type
#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    /// Type that behaves like class (e.g. `kotlin.String`, `kotlin.collections.List<String>`)
    ClassLike(ClassLikeType),
    /// Functional type (e.g. `(Int) -> String`)
    Function(FunctionType),
    /// Generic argument as type (e.g. `T`)
    Generic(Name),
}

impl Type {

    /// Creates `kotlin.Array` [Type] with given [generic_argument]
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

    /// Creates `kotlin.collections.List` [Type] with given [generic_argument]
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

    /// Creates `kotlin.Unit` type
    pub fn unit() -> Type {
        Self::basic_type("Unit")
    }

    /// Creates `kotlin.String` type
    pub fn string() -> Type {
        Self::basic_type("String")
    }

    /// Creates `kotlin.Int` type
    pub fn int() -> Type {
        Self::basic_type("Int")
    }

    /// Creates `kotlin.Double` type
    pub fn double() -> Type {
        Self::basic_type("Double")
    }

    /// Creates `kotlin.Float` type
    pub fn float() -> Type {
        Self::basic_type("Float")
    }

    /// Creates `kotlin.Byte` type
    pub fn byte() -> Type {
        Self::basic_type("Byte")
    }

    /// Creates `kotlin.Short` type
    pub fn short() -> Type {
        Self::basic_type("Short")
    }

    /// Creates `kotlin.Boolean` type
    pub fn boolean() -> Type {
        Self::basic_type("Boolean")
    }

    /// Creates generic type
    pub fn generic(name: &str) -> Type {
        Type::Generic(Name::from(name))
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
    fn render(&self) -> CodeBlock {
        match self {
            Type::ClassLike(class_like) => class_like.render(),
            Type::Generic(name) => name.render(),
            Type::Function(lambda) => lambda.render()
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
        assert_eq!(parameter.render_string(), "T");
    }
}