use std::str::FromStr;
use crate::io::{RenderKotlin};
use crate::tokens::DOT;
use crate::spec::{CodeBlock, Name};
use crate::util::SemanticConversionError;

#[derive(Debug, PartialEq, Clone)]
pub struct Package {
    parts: Vec<Name>,
}

impl Package {
    pub fn from(names: Vec<Name>) -> Package {
        Package { parts: names }
    }
}

impl FromStr for Package {
    type Err = SemanticConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(DOT)
            .map(Name::from_str).collect::<Result<Vec<_>, SemanticConversionError>>()?;
        Ok(Package::from(parts))
    }
}

impl RenderKotlin for Package {

    fn render(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();
        for (index, part) in self.parts.iter().enumerate() {
            code.with_nested(part.render());
            if index != self.parts.len() - 1 {
                code.with_atom(DOT);
            }
        }

        code
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::io::RenderKotlin;
    use crate::spec::Name;

    #[test]
    fn parse_package() {
        let package: super::Package = "io.github.lexadiky".parse().unwrap();
        assert_eq!(package.parts, vec![
            Name::from_str("io").unwrap(),
            Name::from_str("github").unwrap(),
            Name::from_str("lexadiky").unwrap(),
        ]);
    }

    #[test]
    fn render_kotlin() {
        let package: super::Package = "io.github.lexadiky".parse().unwrap();
        assert_eq!(package.render_string(), "io.github.lexadiky");
    }
}
