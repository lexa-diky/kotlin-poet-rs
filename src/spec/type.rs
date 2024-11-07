use std::str::FromStr;

use crate::io::RenderKotlin;
use crate::spec::{ClassLikeTypeName, CodeBlock, FunctionType, Name, Package};
use crate::spec::class_like_type::ClassLikeType;
use crate::tokens;
use crate::util::{SemanticConversionError, yolo_from_str};

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
                    ClassLikeTypeName::top_level(
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
                ClassLikeTypeName::top_level(
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
    pub fn generic<NameLike: Into<Name>>(name: NameLike) -> Type {
        Type::Generic(name.into())
    }

    // Integer numbers
    fn_basic_type_factory!(int, kotlin, Int);
    fn_basic_type_factory!(long, kotlin, Long);
    fn_basic_type_factory!(short, kotlin, Short);
    fn_basic_type_factory!(byte, kotlin, Byte);

    // Floating point numbers
    fn_basic_type_factory!(float, kotlin, Float);
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

impl From<ClassLikeTypeName> for Type {
    fn from(value: ClassLikeTypeName) -> Self {
        Type::ClassLike(ClassLikeType::new(value))
    }
}

impl From<ClassLikeType> for Type {
    fn from(value: ClassLikeType) -> Self {
        Type::ClassLike(value)
    }
}

yolo_from_str!(Type);
impl FromStr for Type {
    type Err = SemanticConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clear = s.trim();

        if !clear.contains(tokens::DOT) {
            return Ok(
                Type::Generic(
                    Name::from_str(clear)?
                )
            );
        }

        if clear.contains(tokens::ROUND_BRACKET_LEFT) {
            return Err(
                SemanticConversionError::new(
                    "Function types are not supported by Type::from_str"
                )
            );
        }

        Ok(
            Type::ClassLike(
                ClassLikeType::from_str(clear)?
            )
        )
    }
}

impl RenderKotlin for Type {
    fn render_into(&self, block: &mut CodeBlock) {
        match self {
            Type::ClassLike(class_like) => block.push_renderable(class_like),
            Type::Generic(name) => block.push_renderable(name),
            Type::Function(lambda) => block.push_renderable(lambda)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::io::RenderKotlin;
    use crate::spec::{Name, Type};

    #[test]
    fn render_generic_parameter() {
        let name = Name::from_str("T").unwrap();
        let parameter = Type::Generic(name);
        assert_eq!(parameter.render_string(), "T");
    }

    #[test]
    fn parse_fn_type() {
        let new_type = Type::from_str("() -> String");
        assert!(matches!(
            new_type,
            Err(SemanticConversionError)
        ));
    }

    #[test]
    fn parse_generic() {
        let new_type = Type::from_str("T");
        assert_eq!(
            new_type.unwrap(),
            Type::generic("T")
        );
    }
}