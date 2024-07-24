use crate::io::{RenderKotlin};
use crate::spec::class_like_type::ClassLikeType;
use crate::spec::{ClassLikeTypeName, CodeBlock, FunctionType, Name, Package};

// region stdlib types codegen
macro_rules! fn_basic_type_factory {
    ($identifier:ident, $($package:tt).+, $class:ident) => {
        #[doc = concat!(
            "Creates `",
            stringify!($($package).+),
            ".",
            stringify!($class)
        )]
        pub fn $identifier() -> Type {
            use std::str::FromStr;

            let package = Package::from_str(stringify!($($package).+)).unwrap();
            let name = Name::from_str(stringify!($class)).unwrap();

            Type::ClassLike(
                ClassLikeType::new(
                    ClassLikeTypeName::simple(
                        package,
                        name
                    )
                )
            )
        }
    };
}

macro_rules! fn_generic_type_factory {
    ($identifier:ident, $($package:tt).+, $class:ident<$($generic:ident),+>) => {
        #[doc = concat!(
            "Creates `",
            stringify!($($package).+),
            ".",
            stringify!($class)
        )]
        pub fn $identifier($($generic: Type,)+) -> Type {
            use std::str::FromStr;

            let package = Package::from_str(stringify!($($package).+)).unwrap();
            let name = Name::from_str(stringify!($class)).unwrap();

            let mut inner_type = ClassLikeType::new(
                ClassLikeTypeName::simple(
                    package,
                    name,
                )
            );

            $(
            inner_type = inner_type.generic_argument($generic);
            )+

            Type::ClassLike(
                inner_type
            )
        }
    };
}
// endregion stdlib types codegen

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

    // Integer numbers
    fn_basic_type_factory!(int, kotlin, Int);
    fn_basic_type_factory!(long, kotlin, Long);
    fn_basic_type_factory!(short, kotlin, Short);
    fn_basic_type_factory!(byte, kotlin, Byte);

    // Floating point numbers
    fn_basic_type_factory!(float, kotlin, Boolean);
    fn_basic_type_factory!(double, kotlin, Double);

    // Logic
    fn_basic_type_factory!(boolean, kotlin, Boolean);

    // Text
    fn_basic_type_factory!(char, kotlin, Char);
    fn_basic_type_factory!(string, kotlin, String);

    // Control Types
    fn_basic_type_factory!(unit, kotlin, Unit);
    fn_basic_type_factory!(any, kotlin, Any);
    fn_basic_type_factory!(nothing, kotlin, Nothing);

    // Collections
    fn_generic_type_factory!(map, kotlin.collections, Map<key, value>);
    fn_generic_type_factory!(list, kotlin.collections, List<value>);
    fn_generic_type_factory!(set, kotlin.collections, Set<value>);
    fn_generic_type_factory!(array, kotlin, Array<value>);
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