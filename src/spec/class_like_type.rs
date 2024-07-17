use crate::io::{RenderContext, RenderKotlin};
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

    fn render(&self, context: RenderContext) -> CodeBlock {
        let mut type_name = self.type_name.render(context);
        if self.nullable {
            type_name.with_atom("?");
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
        assert_eq!(parameter.render_without_context(), "io.github.lexadiky.Class");
    }

    #[test]
    fn render_nullable_class_like_type() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let type_name = ClassLikeTypeName::simple(
            package,
            Name::from_str("Class").unwrap(),
        );
        let parameter = ClassLikeType::new(type_name).nullable(true);
        assert_eq!(parameter.render_without_context(), "io.github.lexadiky.Class?");
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
                        package,
                        Name::from_str("Generic").unwrap(),
                    )
                )
            ));
        assert_eq!(parameter.render_without_context(), "io.github.lexadiky.Class<io.github.lexadiky.Generic>");
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
        assert_eq!(parameter.render_without_context(), "io.github.lexadiky.Class<io.github.lexadiky.Generic>?");
    }
}
