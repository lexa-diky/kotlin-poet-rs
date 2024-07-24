use std::fmt::format;
use std::str::FromStr;
use crate::io::RenderKotlin;
use crate::spec::{ClassLikeTypeName, CodeBlock, Name, Package, Type};
use crate::tokens;
use crate::util::SemanticConversionError;

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

    pub fn nullable(mut self, flag: bool) -> ClassLikeType {
        self.nullable = flag;
        self
    }

    pub fn generic_argument(mut self, parameter: Type) -> ClassLikeType {
        self.generic_arguments.push(parameter);
        self
    }
}

impl RenderKotlin for ClassLikeType {
    fn render(&self) -> CodeBlock {
        let mut type_name = self.type_name.render();

        if !self.generic_arguments.is_empty() {
            type_name.with_atom(tokens::ANGLE_BRACKET_LEFT);

            for (idx, generic_argument) in self.generic_arguments.iter().enumerate() {
                type_name.with_nested(generic_argument.render());
                if idx != self.generic_arguments.len() - 1 {
                    type_name.with_atom(tokens::COMMA);
                    type_name.with_space();
                }
            }

            type_name.with_atom(tokens::ANGLE_BRACKET_RIGHT);
        }

        if self.nullable {
            type_name.with_atom(tokens::QUESTION_MARK);
        };

        type_name
    }
}

impl FromStr for ClassLikeType {
    type Err = SemanticConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() > 1 {
            let mut package_parts = Vec::new();
            for part in &parts[0..parts.len() - 1] {
                package_parts.push(Name::from_str(part)?)
            }

            let package = Package::from(package_parts);
            let name = Name::from_str(parts[parts.len() - 1])?;

            Ok(
                ClassLikeType::new(
                    ClassLikeTypeName::simple(
                        package,
                        name,
                    )
                )
            )
        } else if parts.len() == 1 {
            Ok(
                ClassLikeType::new(
                    ClassLikeTypeName::simple(
                        Package::from(vec![]),
                        Name::from(parts[0])
                    )
                )
            )
        } else {
            Err(
                SemanticConversionError::new(
                    format!("Can't convert {s} to ClassLikeType").as_str()
                )
            )
        }
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
        let type_name = ClassLikeTypeName::simple(
            package,
            Name::from_str("Class").unwrap(),
        );
        let parameter = ClassLikeType::new(type_name);
        assert_eq!(parameter.render_string(), "io.github.lexadiky.Class");
    }

    #[test]
    fn render_nullable_class_like_type() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let type_name = ClassLikeTypeName::simple(
            package,
            Name::from_str("Class").unwrap(),
        );
        let parameter = ClassLikeType::new(type_name).nullable(true);
        assert_eq!(parameter.render_string(), "io.github.lexadiky.Class?");
    }

    #[test]
    fn render_generic_class_like_type() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let type_name = ClassLikeTypeName::simple(
            package.clone(),
            Name::from_str("Class").unwrap(),
        );
        let parameter = ClassLikeType::new(type_name)
            .generic_argument(Type::ClassLike(
                ClassLikeType::new(
                    ClassLikeTypeName::simple(
                        package.clone(),
                        Name::from_str("Generic1").unwrap(),
                    )
                )
            ))
            .generic_argument(Type::ClassLike(
                ClassLikeType::new(
                    ClassLikeTypeName::simple(
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
        let type_name = ClassLikeTypeName::simple(
            package.clone(),
            Name::from_str("Class").unwrap(),
        );
        let parameter = ClassLikeType::new(type_name)
            .generic_argument(Type::ClassLike(
                ClassLikeType::new(
                    ClassLikeTypeName::simple(
                        package,
                        Name::from_str("Generic").unwrap(),
                    )
                )
            )).nullable(true);
        assert_eq!(parameter.render_string(), "io.github.lexadiky.Class<io.github.lexadiky.Generic>?");
    }

    #[test]
    fn test_from_string_long() {
        let class_like_type = ClassLikeType::from_str("io.github.lexadiky.Class").unwrap();
        assert_eq!(class_like_type.render_string(), "io.github.lexadiky.Class");
    }

    #[test]
    fn test_from_string_short() {
        let class_like_type = ClassLikeType::from_str("github.Class").unwrap();
        assert_eq!(class_like_type.render_string(), "github.Class");
    }

    #[test]
    fn test_from_string_top_level() {
        let class_like_type = ClassLikeType::from_str("Class").unwrap();
        assert_eq!(class_like_type.render_string(), "Class");
    }
}
