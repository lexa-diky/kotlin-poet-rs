use std::str::FromStr;
use crate::io::{RenderKotlin};
use crate::tokens::DOT;
use crate::spec::{CodeBlock, Name};
use crate::util::SemanticConversionError;

/// Fully qualified package name, may be parsed from [&str]
#[derive(Debug, PartialEq, Clone)]
pub struct Package {
    parts: Vec<Name>,
}

impl Package {
    /// Creates new package from [Name] parts
    pub fn from(names: Vec<Name>) -> Package {
        Package { parts: names }
    }
}

impl FromStr for Package {
    type Err = SemanticConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(DOT)
            .filter(|part| !part.is_empty())
            .map(Name::from_str)
            .collect::<Result<Vec<_>, SemanticConversionError>>()?;
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
    use crate::spec::{Name, Package};

    #[test]
    fn parse_package() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        assert_eq!(package.parts, vec![
            Name::from_str("io").unwrap(),
            Name::from_str("github").unwrap(),
            Name::from_str("lexadiky").unwrap(),
        ]);
    }

    #[test]
    fn parse_empty_package() {
        let package: Package = "".parse().unwrap();
        assert_eq!(package.parts, vec![]);
    }

    #[test]
    fn render_kotlin() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        assert_eq!(package.render_string(), "io.github.lexadiky");
    }

    #[test]
    fn render_empty() {
        let package: Package = Package::from(vec![]);
        assert_eq!(package.render_string(), "");
    }
}
