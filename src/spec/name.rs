use std::str::FromStr;

use crate::io::RenderKotlin;
use crate::tokens::NAME_PROHIBITED_TOKENS;
use crate::spec::CodeBlock;
use crate::util::SemanticConversionError;

/// Kotlin identifier name, automatically escaped with backticks if it contains escapable tokens
#[derive(Debug, PartialEq, Clone)]
pub struct Name {
    value: String,
}

/// Creates new [Name] from [&str], may panic if creates invalid name
impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Name::from_str(value).unwrap()
    }
}

/// Creates new [Name] from [&str]
impl FromStr for Name {
    type Err = SemanticConversionError;

    // TODO add backtick escaping
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
        assert_eq!(name.render_string(), "Foo");
    }

    #[test]
    fn test_name_with_space() {
        let name = Name::from_str("Foo Bar").unwrap();
        assert_eq!(name.render_string(), "`Foo Bar`");
    }
}