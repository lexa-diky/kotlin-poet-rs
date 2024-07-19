use crate::io::{RenderKotlin, tokens};
use crate::spec::{ClassLikeTypeName, CodeBlock, Type};

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
            type_name.with_atom(tokens::GENERIC_BRACE_LEFT);

            for (idx, generic_argument) in self.generic_arguments.iter().enumerate() {
                type_name.with_nested(generic_argument.render());
                if idx != self.generic_arguments.len() - 1 {
                    type_name.with_atom(tokens::COMMA);
                    type_name.with_space();
                }
            }

            type_name.with_atom(tokens::GENERIC_BRACE_RIGHT);
        }

        if self.nullable {
            type_name.with_atom(tokens::NULLABLE);
        };

        type_name.clone()
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
        assert_eq!(parameter.render_string_in_root(), "io.github.lexadiky.Class");
    }

    #[test]
    fn render_nullable_class_like_type() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let type_name = ClassLikeTypeName::simple(
            package,
            Name::from_str("Class").unwrap(),
        );
        let parameter = ClassLikeType::new(type_name).nullable(true);
        assert_eq!(parameter.render_string_in_root(), "io.github.lexadiky.Class?");
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
            parameter.render_string_in_root(),
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
        assert_eq!(parameter.render_string_in_root(), "io.github.lexadiky.Class<io.github.lexadiky.Generic>?");
    }
}
