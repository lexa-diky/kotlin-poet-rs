use std::str::FromStr;
use crate::io::{RenderContext, RenderKotlin};
use crate::io::tokens::SEPARATOR;
use crate::spec::{CodeBlock, Name};

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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(SEPARATOR)
            .map(|s| Name::from_str(s)).collect::<Result<Vec<_>, ()>>()?;
        Ok(Package::from(parts))
    }
}

impl RenderKotlin for Package {

    fn render(&self, context: RenderContext) -> CodeBlock {
        let mut code = CodeBlock::empty();
        for (index, part) in self.parts.iter().enumerate() {
            code.with_nested(part.render(context));
            if index != self.parts.len() - 1 {
                code.with_atom(SEPARATOR);
            }
        }

        return code;
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
        assert_eq!(package.render_string_in_root(), "io.github.lexadiky");
    }
}
