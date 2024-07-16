use crate::io::RenderKotlin;
use crate::spec::class_like_parameter::ClassLikeParameter;
use crate::spec::Name;

pub enum Parameter {
    ClassLike(ClassLikeParameter),
    Generic(Name),
}

impl RenderKotlin for Parameter {
    fn render(&self) -> String {
        match self {
            Parameter::ClassLike(class_like) => class_like.render(),
            Parameter::Generic(name) => name.render()
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::io::RenderKotlin;
    use crate::spec::{Name, Parameter};

    #[test]
    fn render_generic_parameter() {
        let name = Name::from_str("T").unwrap();
        let parameter = Parameter::Generic(name);
        assert_eq!(parameter.render(), "T");
    }
}