use std::str::FromStr;
use crate::io::RenderKotlin;
use crate::io::tokens::NAME_PROHIBITED_TOKENS;

#[derive(Debug, PartialEq, Clone)]
pub struct Name {
    value: String,
}

impl FromStr for Name {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(
            Name {
                value: s.to_string()
            }
        );
    }
}

impl RenderKotlin for Name {

    fn render(&self) -> String {
        let contains_prohibited_token = NAME_PROHIBITED_TOKENS.iter().any(
            |it| self.value.contains(it)
        );
        if contains_prohibited_token {
            return format!("`{}`", self.value)
        }
        return self.value.clone();
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_name() {
        let name = Name::from_str("Foo").unwrap();
        assert_eq!(name.render(), "Foo");
    }

    #[test]
    fn test_name_with_space() {
        let name = Name::from_str("Foo Bar").unwrap();
        assert_eq!(name.render(), "`Foo Bar`");
    }
}