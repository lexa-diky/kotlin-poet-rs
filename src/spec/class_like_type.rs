use std::str::FromStr;

use crate::io::RenderKotlin;
use crate::spec::{ClassLikeTypeName, CodeBlock, Type};
use crate::tokens;
use crate::util::{SemanticConversionError, yolo_from_str};

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
    pub fn generic_argument(mut self, parameter: Type) -> ClassLikeType {
        self.generic_arguments.push(parameter);
        self
    }
}

impl RenderKotlin for ClassLikeType {
    fn render_into(&self, block: &mut CodeBlock) {
        block.with_embedded(&self.type_name);

        if !self.generic_arguments.is_empty() {
            block.with_atom(tokens::ANGLE_BRACKET_LEFT);

            for (idx, generic_argument) in self.generic_arguments.iter().enumerate() {
                block.with_embedded(generic_argument);
                if idx != self.generic_arguments.len() - 1 {
                    block.with_atom(tokens::COMMA);
                    block.with_space();
                }
            }

            block.with_atom(tokens::ANGLE_BRACKET_RIGHT);
        }

        if self.nullable {
            block.with_atom(tokens::QUESTION_MARK);
        };
    }
}

yolo_from_str!(ClassLikeType);
impl FromStr for ClassLikeType {
    type Err = SemanticConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut is_nullable = false;
        let clean_buf = if s.ends_with(tokens::QUESTION_MARK) {
            is_nullable = true;
            s.trim_end_matches(tokens::QUESTION_MARK)
        } else {
            s
        };

        Ok(
            ClassLikeType::new(
                ClassLikeTypeName::from_str(clean_buf)?
            ).nullable(is_nullable)
        )
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
}
