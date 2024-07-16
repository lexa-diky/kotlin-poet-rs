use std::str::FromStr;
use crate::io::RenderKotlin;

#[derive(Debug, PartialEq)]
pub struct Package {
    parts: Vec<String>
}

const SEPARATOR: &'static str = ".";

impl FromStr for Package {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(SEPARATOR)
            .map(|s| s.to_string()).collect();
        Ok(Package { parts })
    }
}

impl RenderKotlin for Package {

    fn render(&self) -> String {
        self.parts.join(SEPARATOR)
    }
}

#[cfg(test)]
mod test {
    use crate::io::RenderKotlin;

    #[test]
    fn parse_package() {
        let package: super::Package = "io.github.lexadiky".parse().unwrap();
        assert_eq!(package.parts, vec!["io", "github", "lexadiky"]);
    }

    #[test]
    fn render_kotlin() {
        let package: super::Package = "io.github.lexadiky".parse().unwrap();
        assert_eq!(package.render(), "io.github.lexadiky");
    }
}
