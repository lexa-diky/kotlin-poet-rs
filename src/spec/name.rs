use std::str::FromStr;

use crate::io::RenderKotlin;
use crate::tokens::NAME_PROHIBITED_TOKENS;
use crate::spec::CodeBlock;

#[derive(Debug, PartialEq, Clone)]
pub struct Name {
    value: String,
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Name::from_str(value).unwrap()
    }
}

impl FromStr for Name {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Name {
                value: s.to_string()
            }
        )
    }
}

impl RenderKotlin for Name {
    fn render(&self) -> CodeBlock {
        let contains_prohibited_token = NAME_PROHIBITED_TOKENS.iter().any(
            |it| self.value.contains(it)
        );
        if contains_prohibited_token {
            return CodeBlock::atom(format!("`{}`", self.value).as_str());
        }
        return CodeBlock::atom(self.value.as_str());
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_name() {
        let name = Name::from_str("Foo").unwrap();
        assert_eq!(name.render_string_in_root(), "Foo");
    }

    #[test]
    fn test_name_with_space() {
        let name = Name::from_str("Foo Bar").unwrap();
        assert_eq!(name.render_string_in_root(), "`Foo Bar`");
    }
}