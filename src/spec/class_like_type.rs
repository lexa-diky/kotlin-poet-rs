use std::str::FromStr;

use crate::io::RenderKotlin;
use crate::spec::{ClassLikeTypeName, CodeBlock, Type};
use crate::tokens;
use crate::util::{SemanticConversionError, yolo_from_str};

/// Represents a class like type, such as a class, interface, or enum.
/// This type can be nullable and can have generic arguments and nullability mark.
///
/// # Examples
/// ```rust
/// use std::str::FromStr;
/// use kotlin_poet_rs::io::RenderKotlin;
/// use kotlin_poet_rs::spec::{ClassLikeType, ClassLikeTypeName, Type};
/// let class_like_type = ClassLikeType::from("io.github.lexadiky.Class<T>?");
///
/// assert_eq!(
///     "io.github.lexadiky.Class<T>?",
///     class_like_type.render_string()
/// );
///
/// assert_eq!(
///     ClassLikeType::new(
///         ClassLikeTypeName::from("io.github.lexadiky.Class")
///     ).generic_argument(Type::generic("T"))
///         .nullable(true),
///     class_like_type
/// )
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ClassLikeType {
    type_name: ClassLikeTypeName,
    nullable: bool,
    generic_arguments: Vec<Type>,
}

impl ClassLikeType {
    pub fn new(type_name: ClassLikeTypeName) -> ClassLikeType {
        ClassLikeType {
            type_name,
            nullable: false,
            generic_arguments: vec![],
        }
    }

    /// Marks the type as nullable
    pub fn nullable(mut self, flag: bool) -> ClassLikeType {
        self.nullable = flag;
        self
    }

    /// Adds a generic argument to the type
    pub fn generic_argument<TypeLike: Into<Type>>(mut self, parameter: TypeLike) -> ClassLikeType {
        self.generic_arguments.push(parameter.into());
        self
    }
}

impl RenderKotlin for ClassLikeType {
    fn render_into(&self, block: &mut CodeBlock) {
        block.push_renderable(&self.type_name);

        if !self.generic_arguments.is_empty() {
            block.push_atom(tokens::ANGLE_BRACKET_LEFT);

            for (idx, generic_argument) in self.generic_arguments.iter().enumerate() {
                block.push_renderable(generic_argument);
                if idx != self.generic_arguments.len() - 1 {
                    block.push_atom(tokens::COMMA);
                    block.push_space();
                }
            }

            block.push_atom(tokens::ANGLE_BRACKET_RIGHT);
        }

        if self.nullable {
            block.push_atom(tokens::QUESTION_MARK);
        };
    }
}

yolo_from_str!(ClassLikeType);
impl FromStr for ClassLikeType {
    type Err = SemanticConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut is_nullable = false;
        let nullable_clean_buf = if s.ends_with(tokens::QUESTION_MARK) {
            is_nullable = true;
            s.trim_end_matches(tokens::QUESTION_MARK)
        } else {
            s
        };

        let mut generics: Vec<Type> = Vec::new();
        let final_buf = if nullable_clean_buf.ends_with(tokens::ANGLE_BRACKET_RIGHT) {
            let parts = nullable_clean_buf.split(tokens::ANGLE_BRACKET_LEFT).collect::<Vec<&str>>();
            if parts.len() != 2 {
                return Err(SemanticConversionError::new(
                    format!("Invalid class like type with generic like tokens: {}", s).as_str()
                ));
            }
            let generic_sub_buf = parts[1].trim_end_matches(tokens::ANGLE_BRACKET_RIGHT);
            for generic_buf in generic_sub_buf.split(tokens::COMMA) {
                let generic_buf_type = Type::from_str(generic_buf)?;
                generics.push(generic_buf_type);
            }

            parts[0]
        } else {
            nullable_clean_buf
        };


        let mut class_like_type = ClassLikeType::new(
            ClassLikeTypeName::from_str(final_buf)?
        ).nullable(is_nullable);

        for generic in generics {
            class_like_type = class_like_type.generic_argument(generic);
        }

        Ok(class_like_type)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::io::RenderKotlin;
    use crate::spec::{ClassLikeType, ClassLikeTypeName, Type};
    use crate::spec::{Name, Package};

    #[test]
    fn render_simple_class_like_type() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let type_name = ClassLikeTypeName::top_level(
            package,
            Name::from_str("Class").unwrap(),
        );
        let parameter = ClassLikeType::new(type_name);
        assert_eq!(parameter.render_string(), "io.github.lexadiky.Class");
    }

    #[test]
    fn render_nullable_class_like_type() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let type_name = ClassLikeTypeName::top_level(
            package,
            Name::from_str("Class").unwrap(),
        );
        let parameter = ClassLikeType::new(type_name).nullable(true);
        assert_eq!(parameter.render_string(), "io.github.lexadiky.Class?");
    }

    #[test]
    fn render_generic_class_like_type() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let type_name = ClassLikeTypeName::top_level(
            package.clone(),
            Name::from_str("Class").unwrap(),
        );
        let parameter = ClassLikeType::new(type_name)
            .generic_argument(Type::ClassLike(
                ClassLikeType::new(
                    ClassLikeTypeName::top_level(
                        package.clone(),
                        Name::from_str("Generic1").unwrap(),
                    )
                )
            ))
            .generic_argument(Type::ClassLike(
                ClassLikeType::new(
                    ClassLikeTypeName::top_level(
                        package,
                        Name::from_str("Generic2").unwrap(),
                    )
                )
            ));
        assert_eq!(
            parameter.render_string(),
            "io.github.lexadiky.Class<io.github.lexadiky.Generic1, io.github.lexadiky.Generic2>"
        );
    }

    #[test]
    fn render_nullable_generic_class_like_type() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let type_name = ClassLikeTypeName::top_level(
            package.clone(),
            Name::from_str("Class").unwrap(),
        );
        let parameter = ClassLikeType::new(type_name)
            .generic_argument(Type::ClassLike(
                ClassLikeType::new(
                    ClassLikeTypeName::top_level(
                        package,
                        Name::from_str("Generic").unwrap(),
                    )
                )
            )).nullable(true);
        assert_eq!(parameter.render_string(), "io.github.lexadiky.Class<io.github.lexadiky.Generic>?");
    }

    #[test]
    fn test_from_str() {
        let parsed = ClassLikeType::from_str("io.github.lexadiky.Class").unwrap();
        let expected = ClassLikeType::new(
            ClassLikeTypeName::top_level(
                Package::from(vec![
                    Name::from("io"),
                    Name::from("github"),
                    Name::from("lexadiky"),
                ]),
                Name::from("Class"),
            )
        );

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_from_str_nullable() {
        let parsed = ClassLikeType::from_str("io.github.lexadiky.Class?").unwrap();
        let expected = ClassLikeType::new(
            ClassLikeTypeName::top_level(
                Package::from(vec![
                    Name::from("io"),
                    Name::from("github"),
                    Name::from("lexadiky"),
                ]),
                Name::from("Class"),
            )
        ).nullable(true);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_from_str_generic() {
        let parsed = ClassLikeType::from_str("io.github.lexadiky.Class<T, V>?").unwrap();
        let expected = ClassLikeType::new(
            ClassLikeTypeName::top_level(
                Package::from(vec![
                    Name::from("io"),
                    Name::from("github"),
                    Name::from("lexadiky"),
                ]),
                Name::from("Class"),
            )
        ).nullable(true)
            .generic_argument(Type::generic("T"))
            .generic_argument(Type::generic("V"));

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_from_str_class() {
        let parsed = ClassLikeType::from_str("io.github.lexadiky.Class<kotlin.Int>?").unwrap();
        let expected = ClassLikeType::new(
            ClassLikeTypeName::top_level(
                Package::from(vec![
                    Name::from("io"),
                    Name::from("github"),
                    Name::from("lexadiky"),
                ]),
                Name::from("Class"),
            )
        ).nullable(true)
            .generic_argument(Type::int());

        assert_eq!(parsed, expected);
    }
}
