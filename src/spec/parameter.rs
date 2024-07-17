use crate::io::RenderKotlin;
use crate::spec::class_like_parameter::ClassLikeParameter;
use crate::spec::{ClassLikeTypeName, Name, Package, TypeName};

#[derive(PartialEq, Debug, Clone)]
pub enum ParameterType {
    ClassLike(ClassLikeParameter),
    Generic(Name),
}

impl ParameterType {
    pub fn unit() -> ParameterType {
        ParameterType::ClassLike(
            ClassLikeParameter::new(
                TypeName::ClassLike(
                    ClassLikeTypeName::simple(
                        Package::from(
                            vec![
                                Name::from("kotlin")
                            ]
                        ),
                        Name::from("Unit"),
                    )
                )
            )
        )
    }
}
impl RenderKotlin for ParameterType {
    fn render(&self) -> String {
        match self {
            ParameterType::ClassLike(class_like) => class_like.render(),
            ParameterType::Generic(name) => name.render()
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::io::RenderKotlin;
    use crate::spec::{Name, ParameterType};

    #[test]
    fn render_generic_parameter() {
        let name = Name::from_str("T").unwrap();
        let parameter = ParameterType::Generic(name);
        assert_eq!(parameter.render(), "T");
    }
}