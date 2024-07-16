use std::str::FromStr;
use crate::io::RenderKotlin;
use crate::io::tokens::SEPARATOR;
use crate::spec::Name;

#[derive(Debug, PartialEq, Clone)]
pub struct Package {
    parts: Vec<Name>,
}

impl FromStr for Package {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(SEPARATOR)
            .map(|s| Name::from_str(s)).collect::<Result<Vec<_>, ()>>()?;
        Ok(Package { parts })
    }
}

impl RenderKotlin for Package {
    fn render(&self) -> String {
        self.parts.iter().map(|it| it.render())
            .collect::<Vec<_>>()
            .join(SEPARATOR)
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
        assert_eq!(package.render(), "io.github.lexadiky");
    }
}
