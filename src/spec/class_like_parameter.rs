use crate::io::RenderKotlin;
use crate::spec::{Parameter, TypeName};

pub struct ClassLikeParameter {
    type_name: TypeName,
    nullable: bool,
    generic_arguments: Vec<Parameter>,
}

impl ClassLikeParameter {
    pub fn new(type_name: TypeName) -> ClassLikeParameter {
        ClassLikeParameter {
            type_name,
            nullable: false,
            generic_arguments: vec![],
        }
    }

    pub fn nullable(mut self, flag: bool) -> ClassLikeParameter {
        self.nullable = flag;
        self
    }

    pub fn generic_argument(mut self, parameter: Parameter) -> ClassLikeParameter {
        self.generic_arguments.push(parameter);
        self
    }
}

impl RenderKotlin for ClassLikeParameter {
    fn render(&self) -> String {
        let type_name = self.type_name.render();
        let nullable = if self.nullable { "?" } else { "" };
        let generic_arguments = self.generic_arguments.iter()
            .map(|it| it.render()).collect::<Vec<_>>().join(", ");
        if generic_arguments.is_empty() {
            format!("{}{}", type_name, nullable)
        } else {
            format!("{}<{}>{}", type_name, generic_arguments, nullable)
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::io::RenderKotlin;
    use crate::spec::{ClassLikeParameter, ClassLikeTypeName, Parameter, TypeName};
    use crate::spec::{Name, Package};

    #[test]
    fn render_simple_class_like_parameter() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let type_name = TypeName::ClassLike(
            ClassLikeTypeName::simple(
                package,
                Name::from_str("Class").unwrap(),
            )
        );
        let parameter = ClassLikeParameter::new(type_name);
        assert_eq!(parameter.render(), "io.github.lexadiky.Class");
    }

    #[test]
    fn render_nullable_class_like_parameter() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let type_name = TypeName::ClassLike(
            ClassLikeTypeName::simple(
                package,
                Name::from_str("Class").unwrap(),
            )
        );
        let parameter = ClassLikeParameter::new(type_name).nullable(true);
        assert_eq!(parameter.render(), "io.github.lexadiky.Class?");
    }

    #[test]
    fn render_generic_class_like_parameter() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let type_name = TypeName::ClassLike(
            ClassLikeTypeName::simple(
                package.clone(),
                Name::from_str("Class").unwrap(),
            )
        );
        let parameter = ClassLikeParameter::new(type_name)
            .generic_argument(Parameter::ClassLike(
                ClassLikeParameter::new(
                    TypeName::ClassLike(
                        ClassLikeTypeName::simple(
                            package,
                            Name::from_str("Generic").unwrap(),
                        )
                    )
                )
            ));
        assert_eq!(parameter.render(), "io.github.lexadiky.Class<io.github.lexadiky.Generic>");
    }

    #[test]
    fn render_nullable_generic_class_like_parameter() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        let type_name = TypeName::ClassLike(
            ClassLikeTypeName::simple(
                package.clone(),
                Name::from_str("Class").unwrap(),
            )
        );
        let parameter = ClassLikeParameter::new(type_name)
            .generic_argument(Parameter::ClassLike(
                ClassLikeParameter::new(
                    TypeName::ClassLike(
                        ClassLikeTypeName::simple(
                            package,
                            Name::from_str("Generic").unwrap(),
                        )
                    )
                )
            )).nullable(true);
        assert_eq!(parameter.render(), "io.github.lexadiky.Class<io.github.lexadiky.Generic>?");
    }
}
